
struct Resolution {
    width: u64,
    height: u64,
}

type PixelGrid = Vec<Vec<(u8, u8, u8)>>;

fn main() {
    let res = Resolution {
        width: 4,
        height: 4,
    };

    let image: PixelGrid = vec![
        vec![(255,0,0), (0,0,0), (0,0,0), (0,0,0)],
        vec![(0,0,0), (0,0,0), (0,0,0), (0,0,0)],
        vec![(0,0,0), (0,0,0), (0,0,0), (0,0,0)],
        vec![(0,0,0), (0,0,0), (0,0,0), (0,0,127)],
    ];


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
