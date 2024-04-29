use vec::Color;

use crate::{draw::Draw, line_segment::LineSegment, vec::Point2D};


mod draw;
mod line_segment;
mod vec;

struct Resolution {
    width: u64,
    height: u64,
}


type PixelGrid = Vec<Vec<Color>>;

fn main() {
    let res = Resolution {
        width: 400,
        height: 400,
    };

    let mut image: PixelGrid = vec![ vec![Color(0, 0, 0); 400]; 400 ];

    let line_seg1 = LineSegment {
        p0: Point2D(0, 0),
        p1: Point2D(250, 20),
        color: Color(255, 255, 255),
    };

    line_seg1.draw(&mut image);

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
