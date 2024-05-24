#![feature(generic_const_exprs)]

use bezier_curve::QuadraticBezierCurve;
use pixel_grid::Resolution;
use point_and_color::Color;

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
    let mut world = World::new();

    world.push(Box::new(QuadraticBezierCurve {
        points: [Point2D(20.0, 300.0), Point2D(350.0, 350.0), Point2D(350.0, 20.0)],
        color: Color(0, 127, 0),
    }));

    world.draw(&mut image);

    image.save_as_ppm(&mut std::io::stdout()).unwrap();
}
