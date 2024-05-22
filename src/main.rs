use point_and_color::Color;

use crate::{bezier_curve::QuadraticBezierCurve, draw::Draw, line_segment::LineSegment, point_and_color::Point2D};


mod draw;
mod line_segment;
mod point_and_color;
mod bezier_curve;

struct Resolution {
    width: u64,
    height: u64,
}

// This belongs in some other file but I'm lazy...
fn lerp(p0: Point2D, p1: Point2D, t: f64) -> Point2D {
    p0 * (1.0 - t)  + p1 * t
}


type PixelGrid = Vec<Vec<Color>>;

fn main() {
    let res = Resolution {
        width: 400,
        height: 400,
    };

    let mut image: PixelGrid = vec![ vec![Color(0, 0, 0); 400]; 400 ];

    let line_seg1 = LineSegment {
        p0: Point2D(0.0, 0.0),
        p1: Point2D(250.0, 20.0),
        color: Color(255, 255, 255),
    };

    let line_seg2 = LineSegment {
        p0: Point2D(350.0, 30.0),
        p1: Point2D(40.0, 90.0),
        color: Color(130, 20, 75),
    };

    let line_seg3 = LineSegment {
        p0: Point2D(0.0, 200.0),
        p1: Point2D(400.0, 200.0),
        color: Color(75, 75, 75)
    };

    let bezier1 = QuadraticBezierCurve {
        p0: Point2D(20.0, 300.0),
        p1: Point2D(350.0, 350.0),
        p2: Point2D(350.0, 20.0),
        color: Color(0, 127, 0),
    };

    line_seg1.draw(&mut image);
    line_seg2.draw(&mut image);
    line_seg3.draw(&mut image);
    bezier1.draw(&mut image);

    // PPM Image Header
    println!("P3");
    println!("{} {}", res.width, res.height);
    println!("255");

    // PPM Body
    for scanline in image {
        for pixel in scanline {
            print!("{} {} {} ", pixel.0, pixel.1, pixel.2);
        }
        println!("");
    }
}
