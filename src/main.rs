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

fn main() {
    let res = Resolution {
        width: 400,
        height: 400,
    };

    let mut image: PixelGrid = PixelGrid::new(res);

    let world: World = serde_json::from_reader(BufReader::new(File::open("sample_world.json").unwrap())).unwrap();
    world.draw(&mut image);

    image.save_as_ppm(&mut std::io::stdout()).unwrap();
}
