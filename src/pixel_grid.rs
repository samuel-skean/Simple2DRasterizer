use std::io::Write;

use crate::point_and_color::Color;

pub struct PixelGrid(pub Vec<Vec<Color>>);

#[derive(Clone, Copy)]
pub struct Resolution {
    pub width: usize,
    pub height: usize,
}

impl PixelGrid {
    pub fn new(res: Resolution) -> Self {
        Self(vec![vec![Color(0, 0, 0); res.width]; res.height])
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
                write!(dest, "{} {} {} ", pixel.0, pixel.1, pixel.2)?;
            }
            writeln!(dest, "")?;
        }
        Ok(())
    }
}
