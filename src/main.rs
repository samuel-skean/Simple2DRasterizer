use pixel_grid::Resolution;
use point_and_color::Color;

use crate::{
    bezier_curve::QuadraticBezierCurve, draw::Draw, line_segment::LineSegment,
    pixel_grid::PixelGrid, point_and_color::Point2D, world::World,
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

    let mut world: World = vec![
        Box::new(LineSegment {
            p0: Point2D(0.0, 0.0),
            p1: Point2D(250.0, 20.0),
            color: Color(255, 255, 255),
        }),
        Box::new(LineSegment {
            p0: Point2D(350.0, 30.0),
            p1: Point2D(40.0, 90.0),
            color: Color(130, 20, 75),
        }),
        Box::new(LineSegment {
            p0: Point2D(0.0, 200.0),
            p1: Point2D(400.0, 200.0),
            color: Color(75, 75, 75),
        }),
    ];

    // TODO: It seems like there should be a way to construct a vec of Boxes
    // that point to different types at once, but I don't know how to figure it
    // out.
    world.push(Box::new(QuadraticBezierCurve {
        p0: Point2D(20.0, 300.0),
        p1: Point2D(350.0, 350.0),
        p2: Point2D(350.0, 20.0),
        color: Color(0, 127, 0),
    }));

    world.draw(&mut image);

    image.save_as_ppm(&mut std::io::stdout()).unwrap();
}
