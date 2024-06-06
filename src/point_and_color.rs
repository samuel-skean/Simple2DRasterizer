use serde::{Deserialize, Serialize};

use crate::{meaningless_bytes::MeaninglessBytes, PixelGrid};
use std::ops::{Add, Mul};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Point2D(pub f64, pub f64); // A point in 2d space, represented as (x, y), with (0, 0) as the upper-left corner.
#[derive(Clone, Copy, Deserialize, Serialize, bytemuck::NoUninit)]
#[repr(C, align(4))]
pub struct Color(
    pub u8,
    pub u8,
    pub u8,
    // Logical padding, though actual padding is not allowed because of
    // bytemuck::NoUninit, required to use Atomic<Color>. Implicitly initialized
    // using Default::default(), which returns 0.
    #[serde(skip)]
    MeaninglessBytes<1>,
); // An 8-bit-per-channel/24-bit-total color, suitable for "millions" (about 16.7 million) colors.

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color(r, g, b, Default::default())
    }
}

impl From<Color> for sdl2::pixels::Color {
    fn from(value: Color) -> Self {
        sdl2::pixels::Color::RGB(
            value.0,
            value.1,
            value.2,
        )
    }
}

impl Mul<f64> for Point2D {
    type Output = Point2D;
    fn mul(self, rhs: f64) -> Self::Output {
        Point2D(self.0 * rhs, self.1 * rhs)
    }
}

impl Add for Point2D {
    type Output = Point2D;
    fn add(self, rhs: Self) -> Self::Output {
        Point2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Point2D {
    pub fn draw_specifying_color(&self, target: &PixelGrid, color: Color) {
        target.0[self.1 as usize][self.0 as usize].store(color, atomic::Ordering::Release);
        // The order of the coordinates is weird and unintuitive!
    }
}
