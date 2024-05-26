use std::path::PathBuf;
use std::process::exit;
use std::sync::{Arc, RwLock};
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
mod splines;
mod world;

// This belongs in some other file but I'm lazy...
fn lerp(p0: Point2D, p1: Point2D, t: f64) -> Point2D {
    p0 * (1.0 - t) + p1 * t
}

use sdl2::event::Event;
use sdl2::keyboard::{self, Keycode};
use std::time::Duration;

impl Draw for RwLock<Option<World>> {
    fn draw(&self, target: &PixelGrid) {
        while self.read().unwrap().is_none() {
            std::thread::sleep(Duration::from_millis(10));
        }

        self.read().unwrap().as_ref().unwrap().draw(target);
    }

    // I really *don't* intend anyone to ever serialize or deserialize this -
    // not that it'd be awful.
    #[doc(hidden)]
    fn typetag_name(&self) -> &'static str {
        unimplemented!();
    }

    #[doc(hidden)]
    fn typetag_deserialize(&self) {
        unimplemented!();
    }
}

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

    let world: Arc<RwLock<Option<World>>> = Arc::new(None.into());

    // I'm super happy about scoped threads since they let me do what I want at
    // all, very easily... but I'm not too happy about this extra indentation.
    std::thread::scope(|s| -> Result<(), String> {
        s.spawn(|| world.draw(&image));

        let mut event_pump = sdl_context.event_pump()?;
        let surface = window.surface(&event_pump)?;

        put_something_on_the_goshdarn_screen(surface, &image)?;

        let mut save_file = false;
        let mut no_world_loaded = true;

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

            if no_world_loaded {
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
                                *world.write().unwrap() = Some(w);
                                no_world_loaded = false;
                            },
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

            save_file = false;

            put_something_on_the_goshdarn_screen(surface, &image)?;

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            // The rest of the game loop goes here...
        }
    })
    .and_then(|_| {
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
