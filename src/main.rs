use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::thread::ScopedJoinHandle;
use std::{fs::File, io::BufReader};

use clap::Parser;
use pixel_grid::Resolution;
use sdl2::messagebox::MessageBoxFlag;
use user_interaction_helpers::*;

use crate::{draw::Draw, pixel_grid::PixelGrid, point_and_color::Point2D, world::World};

mod bezier_curves;
mod draw;
mod line_segment;
mod pixel_grid;
mod point_and_color;
mod splines;
mod user_interaction_helpers;
mod world;

const APP_NAME: &str = "Skean's Wonderful Bézier Emporium";

// This belongs in some other file but I'm lazy...
fn lerp(p0: Point2D, p1: Point2D, t: f64) -> Point2D {
    p0 * (1.0 - t) + p1 * t
}

use sdl2::event::Event;
use sdl2::keyboard::{self, Keycode};
use std::time::Duration;

#[derive(Parser, Clone)]
struct Cli {
    /// Where are you loading the world from? If you don't supply one here, you
    /// will be asked for one interactively.
    world_path: Option<PathBuf>,
    /// Where do you want to put the image? Both .bmp files and .ppm files are
    /// supported. If you do not supply one, the image will have to be saved
    /// interactively with Ctrl+S (sorry, mac users).
    #[arg(short, long)]
    output_path: Option<PathBuf>,
}

impl Cli {
    // It really *felt* like `arg(value_parser = [expr])` (link to documentation
    // here: https://docs.rs/clap/latest/clap/_derive/index.html#arg-attributes)
    // would be perfect, but it seems to be about actually parsing things from
    // strings in arbitrary ways, not validating them. And in fact, it really
    // seems it should be possible with TypedValueParser::map, but I'm unable to
    // get there because it seems Option<PathBuf> does not implement ValueEnum,
    // which just seems wrong - clap has lots of good support for Options of
    // many things, including of PathBufs. Here's the full message:
    // ```
    // error[E0277]: the trait bound `std::option::Option<PathBuf>: ValueEnum` is not satisfied
    //     --> src/main.rs:43:56
    //     |
    // 43   |     #[arg(short, long, value_parser = ValueParser::new(EnumValueParser::<Option<PathBuf>>::new()))]
    //     |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ValueEnum` is not implemented for `std::option::Option<PathBuf>`
    //     |
    //     = help: the trait `ValueEnum` is implemented for `ColorChoice`
    // note: required by a bound in `EnumValueParser`
    //    --> /Users/samuelskean/.cargo/registry/src/index.crates.io-6f17d22bba15001f/clap_builder-4.5.2/src/builder/value_parser.rs:1117:31
    //     |
    // 1117 | pub struct EnumValueParser<E: crate::ValueEnum + Clone + Send + Sync + 'static>(
    //     |                               ^^^^^^^^^^^^^^^^ required by this bound in `EnumValueParser`
    // ```
    // The above error message refers to no code in version control - I deleted it for cleanliness (and out of shame lol).

    // In the meantime, I guess, you better call this method on all *one* instances of Cli! Or else!
    fn validate(&self) -> Self {
        let extension = self
            .output_path
            .as_ref()
            .map(|output_path| output_path.extension())
            .flatten();
        if extension.is_some_and(|e| e == "ppm" || e == "bmp") {
            self.clone() // TODO: Remove this `clone()` with a static, or a LazyCell, or something.
        } else {
            Cli {
                output_path: None,
                // Yay, my very first usage of struct update syntax! I
                // remembered the syntax without looking it up too, which speaks
                // well of it.
                ..self.clone()
            }
        }
    }
}

pub fn main() {
    eprintln!(
        "Implementation of Atomic<Color> is lock free: {}",
        atomic::Atomic::<point_and_color::Color>::is_lock_free(),
    );
    let mut config = Cli::parse().validate();

    let res = Resolution {
        width: 400,
        height: 400,
    };

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window = video_subsystem
        .window(APP_NAME, res.width as u32, res.height as u32)
        .position_centered()
        .build()
        .unwrap();

    let image: PixelGrid = PixelGrid::new(res);

    // I'm not totally clear on why I'm getting this warning - when I follow the
    // advice the compiler gives me, I get a compiler error about World not
    // implementing Copy. I really don't want to make it implement Copy, even if
    // I think that's never triggered. It seems the compiler is having trouble
    // analyzing exactly what can happen in the loop - I think it's behaving as
    // expected and I just want it to (partly) solve the halting problem or
    // whatever, and realize that my flag variable world_loaded has a special
    // role in the loop.
    #[allow(unused_assignments)]
    // TODO: Consider applying method wizardry here. This is a little
    // pyramid-of-doom-y as it stands, but at least it's understandable like
    // this:
    let mut world: Option<World> = match config.world_path {
        Some(ref world_path) => {
            let load_world_result = load_world(&world_path);
            match load_world_result {
                Ok(world) => Some(world),
                // TODO: Get rid of the ugly instanceof (anyhow::Error::is, in this case, but still):
                Err(e) if e.is::<std::io::Error>() => {
                    alert_about_io_error_with_world_file(true, e, &window);
                    config.world_path = None;
                    None
                }
                Err(e) => {
                    alert_about_invalid_world_file(true, e, &window);
                    config.world_path = None;
                    None
                }
            }
        }
        None => None,
    };

    // I'm super happy about scoped threads since they let me do what I want at
    // all, very easily... but I'm not too happy about this extra indentation.
    std::thread::scope(|s| {
        let mut drawing_thread: Option<ScopedJoinHandle<()>> = None;
        let mut event_pump = sdl_context.event_pump().unwrap();
        let surface = window.surface(&event_pump).unwrap();

        put_something_on_the_goshdarn_screen(surface, &image).unwrap();

        let mut save_image = false;
        let mut world_loaded = false;

        // TODO: Attempt to use `unreachable!` as a cleaner way of telling the
        // compiler about how this loop terminates. Maybe it fixes weird
        // borrowing issues with the `world` local variable, without all the
        // crazy crap we have to do here? I haven't thought it through, but it
        // sure *sounds* cool.
        // Documentation for `unreachable!` macro:
        // https://doc.rust-lang.org/std/macro.unreachable.html
        'event_loop: loop {
            'inner_loop: for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        if let Some(ref drawing_thread) = drawing_thread {
                            if !drawing_thread.is_finished() && config.output_path.is_some() {
                                // TODO: Improve this warning message with
                                // information about what happens with each of
                                // the kinds of images that can be saved.
                                let quit_anyway = confer_with_user(
                                    AlertKind::WARNING,
                                    "Unfinished Work",
                                    "Are you sure you want to quit? The drawing thread isn't finished!\n\
                                    Who knows what image you're saving, I haven't bothered to figure it out.",
                                    &window,
                                    "Cancel",
                                    "Save and Quit Now"
                                );
                                if quit_anyway {
                                    break 'event_loop;
                                } else {
                                    continue 'inner_loop;
                                }
                            }
                        }
                        break 'event_loop;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::S),
                        keymod: keyboard::Mod::LCTRLMOD,
                        ..
                    } => {
                        save_image = true;
                    }
                    _ => {}
                }
            }

            if !world_loaded {
                let Some(world_path) = config
                    .world_path
                    .clone()
                    .or_else(|| file_dialog(&["json"]).pick_file())
                else {
                    alert(
                        false,
                        AlertKind::INFORMATION,
                        "No Path Provided",
                        "We didn't get a path back from the dialog box.\n\
                        The application will quit when you dismiss this pop-up.",
                        &window,
                    );
                    return;
                };
                match load_world(&world_path) {
                    Ok(w) => {
                        world = Some(w);
                        window
                            .set_title(
                                &(
                                    world_path
                                    .file_name()
                                    .expect("There was no file name in the path provided for the world, \
                                    and yet we successfully loaded said world. Fascinating...")
                                    .to_string_lossy() + " - " + APP_NAME
                                )
                            )
                            .unwrap();
                        world_loaded = true;
                        let image_borrow = &image; // TODO: Show this to Jacob Cohen.
                        drawing_thread = Some(s.spawn(move || world.unwrap().draw(image_borrow)));
                    }
                    // TODO: Get rid of the ugly instanceof (anyhow::Error::is, in this case, but still):
                    Err(e) if e.is::<std::io::Error>() => {
                        alert_about_io_error_with_world_file(false, e, &window);
                        config.world_path = None;
                    }
                    Err(e) => {
                        alert_about_invalid_world_file(false, e, &window);
                    }
                };
            }

            let surface = window.surface(&event_pump).unwrap();

            // REVISIT: This following block (inside the if statement) doesn't
            // conceptually need to have the complicated control flow it does as
            // a virtue of depending on and setting save_image, since saving the
            // image should not be retried. But it does, because it's hard to
            // factor this out or move it directly into the arm of the
            // match-statement-on-events where save_image is set to true because
            // it needs to mutate the surface, which holds a references the
            // event pump, which needs to be mutably referenced for it to be
            // polled.
            //
            // "There's *gotta* be a better way!", in the words of a friend of
            // mine about something completely different. I've thought about
            // introducing various kinds of interior mutability, but I actually
            // don't think they'd work because the event loop needs to stay
            // mutably borrowed for the duration of the match statement.
            //
            if save_image {
                let Some(file_path) = file_dialog(&["bmp", "ppm"]).save_file() else {
                    save_image = false;
                    continue 'event_loop;
                    // Do not quit the program in this case.
                };
                save_image_file(file_path, &image, &window, &surface).unwrap();
            }

            save_image = false;

            put_something_on_the_goshdarn_screen(surface, &image).unwrap();

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        match config.output_path {
            Some(file_path) if file_path.to_string_lossy() == "-" => {
                image.save_as_ppm(&mut std::io::stdout()).unwrap();
            }
            Some(file_path) => {
                let surface = window.surface(&event_pump).unwrap();
                save_image_file(file_path, &image, &window, &surface).unwrap();
            }
            _ => {
                eprintln!(
                    "INFORMATION: No (valid) location to save was specified on the \
                command line, so no image was saved."
                );
            }
        };

        // This is the nicest way I've found to forcibly kill any running
        // background threads.
        //
        // REVISIT: Unfortunately, it does prevent any `pending` destruction
        // from happening on them.
        std::process::exit(0);
    });
}

fn load_world(world_path: &Path) -> anyhow::Result<World> {
    Ok(serde_json::from_reader(BufReader::new(File::open(
        world_path,
    )?))?)
}

// TODO: Change this function to account for the fact that it *may* be called to
// save the image in any circumstance, whether the path was specified on the
// command line or graphically.
fn save_image_file(
    file_path: PathBuf,
    image: &PixelGrid,
    window: &sdl2::video::Window,
    surface: &sdl2::video::WindowSurfaceRef,
) -> Result<(), String> {
    Ok(match file_path.extension() {
        Some(s) if s == "ppm" => {
            if confer_with_user(
                MessageBoxFlag::WARNING,
                "Warning",
                "This might not do what you expect. \
                This saves the image that has currently been created, \
                not the image that is currently on the screen.",
                window,
                "Cancel",
                "Save",
            ) {
                image
                    .save_as_ppm(&mut BufWriter::new(
                        File::create(file_path).map_err(|e| e.to_string())?,
                    ))
                    .map_err(|e| e.to_string())?;
            }
        }
        Some(s) if s == "bmp" => {
            surface.save_bmp(file_path)?;
        }
        _ => {
            alert(
                false,
                MessageBoxFlag::ERROR,
                "How on Earth Did We Get Here",
                "You gave me a filename with an extension I don't support, \
                I think through the GUI. How dare you?! (I only support .bmp and .ppm \
                files for saving, btw).",
                window,
            );
        }
    })
}

fn put_something_on_the_goshdarn_screen(
    mut surface: sdl2::video::WindowSurfaceRef,
    image: &PixelGrid,
) -> Result<(), String> {
    let byte_slice = surface
        .without_lock_mut()
        .ok_or("Unable to write to the surface.")?;
    let surface_slice: &mut [u32] = unsafe {
        // Look ma! A silly little bit of unsafe!
        let length_dividend = std::mem::size_of::<u32>() / std::mem::size_of::<u8>();
        std::slice::from_raw_parts_mut(
            std::mem::transmute(byte_slice.as_mut_ptr()),
            byte_slice.len() / length_dividend,
        )
    };
    for (i, p) in image.0.iter().flatten().enumerate() {
        let color = sdl2::pixels::Color::from(p.load(atomic::Ordering::Acquire));
        let color_as_number = color.to_u32(&surface.pixel_format());
        surface_slice[i] = color_as_number;
    }
    surface.finish()?;
    Ok(())
}
