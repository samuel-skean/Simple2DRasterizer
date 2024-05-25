use std::{fs::File, io::BufReader};

use pixel_grid::Resolution;

use crate::{
    draw::Draw, pixel_grid::PixelGrid, point_and_color::Point2D, world::World,
};

mod bezier_curve;
mod draw;
mod line_segment;
mod pixel_grid;
mod point_and_color;
mod world;

// This belongs in some other file but I'm lazy...
fn lerp(p0: Point2D, p1: Point2D, t: f64) -> Point2D {
    p0 * (1.0 - t) + p1 * t
}

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    let res = Resolution {
        width: 400,
        height: 400,
    };

    let mut image: PixelGrid = PixelGrid::new(res);

    let world: World = serde_json::from_reader(BufReader::new(File::open("sample_world.json").unwrap())).unwrap();
    world.draw(&mut image);

    image.save_as_ppm(&mut std::io::stdout()).unwrap();



    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Basic 2D Rasterizer", res.width as u32, res.height as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut surface = window.surface(&event_pump)?;

    let surface_slice: &mut [u32] = unsafe { // Look ma! A silly little bit of unsafe!
        std::mem::transmute(surface.without_lock_mut().ok_or("Unable to write to the surface.")?)
    };
    for (i, p) in image.0.iter_mut().flatten().enumerate() {
        let color: sdl2::pixels::Color = (*p).into();
        let color_as_number = color.to_u32(&surface.pixel_format());
        surface_slice[i] = color_as_number;
    }

    surface.finish()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}