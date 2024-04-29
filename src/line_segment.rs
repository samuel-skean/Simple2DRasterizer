use crate::{draw::Draw, vec::{Color, Point2D}, PixelGrid};

pub struct LineSegment {
    pub p0: Point2D,
    pub p1: Point2D,
    pub color: Color,
}

const LERP_RESOLUTION: u64 = 1000;

impl Draw for LineSegment {
    fn draw(&self, target: &mut PixelGrid) {
        for t in 0..LERP_RESOLUTION {
            let Point2D(x, y) = self.p0 + self.p1 * (1.0 - t as f64/LERP_RESOLUTION as f64);
            // TODO: There's probably a better way that doesn't involve casting *here* but does allow me to maintain the known bit-width of the point coordinates. Not doing it now.
            target[y as usize][x as usize] = self.color;
        }
    }
}