use std::path::PathBuf;
use std::process::exit;
use std::{fs::File, io::BufReader};

use pixel_grid::Resolution;
use rfd::FileDialog;
use sdl2::messagebox::{show_simple_message_box, MessageBoxFlag};

use crate::{draw::Draw, pixel_grid::PixelGrid, point_and_color::Point2D, world::World};

mod bezier_curves;
mod draw;
mod line_segment;
mod pixel_grid;
mod point_and_color;
mod world;
mod splines;

// This belongs in some other file but I'm lazy...
fn lerp(p0: Point2D, p1: Point2D, t: f64) -> Point2D {
    p0 * (1.0 - t) + p1 * t
}

use sdl2::event::Event;
use sdl2::keyboard::{self, Keycode};
use std::time::Duration;

pub fn main() -> Result<(), String> {
    let res = Resolution {
        width: 400,
        height: 400,
    };

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Basic 2D Rasterizer", res.width as u32, res.height as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

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
    // Starts out not existing - it starts to exist once it's loaded, further
    // down:
    let mut world: Option<World> = None;

    // I'm super happy about scoped threads since they let me do what I want at 
    // all, very easily... but I'm not too happy about this extra indentation.
    std::thread::scope(|s| -> Result<(), String> {
        let mut event_pump = sdl_context.event_pump()?;
        let surface = window.surface(&event_pump)?;

        put_something_on_the_goshdarn_screen(surface, &image)?;

        let mut save_file = false;
        let mut world_loaded = false;

        loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => exit(0),
                    Event::KeyDown {
                        keycode: Some(Keycode::S),
                        keymod: keyboard::Mod::LCTRLMOD,
                        ..
                    } => {
                        save_file = true;
                    }
                    _ => {}
                }
            }

            let surface = window.surface(&event_pump)?;

            if save_file {
                let file_path_option = FileDialog::new().set_directory(".").save_file();
                match file_path_option {
                    Some(file_path) => {
                        surface.save_bmp(file_path)?;
                    }
                    None => show_simple_message_box(
                        MessageBoxFlag::INFORMATION,
                        "Invalid Path",
                        "We didn't get a valid path back from the dialog box.",
                        &window,
                    )
                    .unwrap(),
                }
            }

            save_file = false;
            if !world_loaded {
                let world_path_option = FileDialog::new().set_directory(".").pick_file();
                match world_path_option {
                    Some(world_path) => {
                        fn load_world(world_path: PathBuf) -> anyhow::Result<World> {
                            Ok(serde_json::from_reader(BufReader::new(File::open(
                                world_path,
                            )?))?)
                        }
                        match load_world(world_path) {
                            Ok(w) => {
                                world = Some(w);
                                world_loaded = true;
                                let image_borrow = &image; // TODO: Show this to Jacob Cohen.
                                s.spawn(move || world.unwrap().draw(image_borrow));
                            }
                            Err(e) => show_simple_message_box(
                                MessageBoxFlag::INFORMATION,
                                "Invalid World File",
                                e.to_string().as_str(),
                                &window,
                            )
                            .unwrap(),
                        };
                    }
                    None => show_simple_message_box(
                        MessageBoxFlag::INFORMATION,
                        "Invalid Path",
                        "We didn't get a valid path back from the message box.",
                        &window,
                    )
                    .unwrap(),
                }
            }

            put_something_on_the_goshdarn_screen(surface, &image)?;

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            // The rest of the game loop goes here...
        }
    }).and_then(|_| {
        image.save_as_ppm(&mut std::io::stdout()).unwrap();
        Ok(())
    })
}

fn put_something_on_the_goshdarn_screen(
    mut surface: sdl2::video::WindowSurfaceRef,
    image: &PixelGrid,
) -> Result<(), String> {
    let surface_slice: &mut [u32] = unsafe {
        // Look ma! A silly little bit of unsafe!
        std::mem::transmute(
            surface
                .without_lock_mut()
                .ok_or("Unable to write to the surface.")?,
        )
    };
    for (i, p) in image.0.iter().flatten().enumerate() {
        let color: sdl2::pixels::Color = p.into();
        let color_as_number = color.to_u32(&surface.pixel_format());
        surface_slice[i] = color_as_number;
    }
    surface.finish()?;
    Ok(())
}
