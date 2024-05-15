use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
pub struct Point2D(pub u64, pub u64); // A point in 2d space, represented as (x, y), with (0, 0) as the upper-left corner.
#[derive(Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8); // An 8-bit-per-channel/24-bit-total color, suitable for "millions" (about 16.7 million) colors.


impl Mul<f64> for Point2D {
    type Output = Point2D;
    fn mul(self, rhs: f64) -> Self::Output {
        Point2D((self.0 as f64 * rhs) as u64, (self.1 as f64 * rhs) as u64)
    }
}

impl Add for Point2D {
    type Output = Point2D;
    fn add(self, rhs: Self) -> Self::Output {
        Point2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}