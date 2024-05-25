use serde::{Deserialize, Serialize};

use crate::PixelGrid;
use std::{ops::{Add, Mul}, sync::atomic::{AtomicU8, Ordering::*}};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Point2D(pub f64, pub f64); // A point in 2d space, represented as (x, y), with (0, 0) as the upper-left corner.
#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Color(pub u8, pub u8, pub u8); // An 8-bit-per-channel/24-bit-total color, suitable for "millions" (about 16.7 million) colors.

pub struct ShareableColor(pub AtomicU8, pub AtomicU8, pub AtomicU8);

impl ShareableColor {
    pub fn store(&self, value: Color) {
        self.0.store(value.0, Release);
        self.1.store(value.1, Release);
        self.2.store(value.2, Release);
    }
}

impl From<Color> for ShareableColor {
    fn from(value: Color) -> Self {
        ShareableColor(value.0.into(), value.1.into(), value.2.into())
    }
}

impl From<ShareableColor> for Color {
    fn from(value: ShareableColor) -> Self {
        Color(value.0.into_inner(), value.1.into_inner(), value.2.into_inner())
    }
}

impl From<&ShareableColor> for sdl2::pixels::Color {
    fn from(value: &ShareableColor) -> Self {
        sdl2::pixels::Color::RGB(value.0.load(Acquire), value.1.load(Acquire), value.2.load(Acquire))
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
        target.0[self.1 as usize][self.0 as usize].store(color);
        // The order of the coordinates is weird and unintuitive!
    }
}
