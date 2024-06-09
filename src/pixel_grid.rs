use std::io::Write;

use atomic::Atomic;

use crate::point_and_color::Color;

pub struct PixelGrid(pub Vec<Vec<Atomic<Color>>>);

#[derive(Clone, Copy)]
pub struct Resolution {
    pub width: usize,
    pub height: usize,
}

impl PixelGrid {
    pub fn new(res: Resolution) -> Self {
        let mut grid = PixelGrid(Vec::new());
        for j in 0..res.height {
            grid.0.push(Vec::new());
            for _ in 0..res.width {
                grid.0[j].push(Atomic::new(Color::new(0, 0, 0)))
            }
        }
        grid
    }
}

impl PixelGrid {
    pub fn save_as_ppm(&self, dest: &mut impl Write) -> std::io::Result<()> {
        // PPM Image Header
        writeln!(dest, "P3")?;
        writeln!(dest, "{} {}", self.0.len(), self.0[0].len())?;
        writeln!(dest, "255")?;

        // PPM Body
        for scanline in &self.0 {
            for pixel in scanline {
                let pixel = pixel.load(atomic::Ordering::Acquire);
                write!(
                    dest,
                    "{} {} {} ",
                    pixel.0,
                    pixel.1,
                    pixel.2,
                )?;
            }
            writeln!(dest)?;
        }
        Ok(())
    }
}
