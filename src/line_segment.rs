use crate::{draw::Draw, vec::{Color, Point2D}, PixelGrid};

pub struct LineSegment {
    pub p0: Point2D,
    pub p1: Point2D,
    pub color: Color,
}

const LERP_RESOLUTION: u64 = 1_000;

impl Draw for LineSegment {
    fn draw(&self, target: &mut PixelGrid) {
        // TODO: Why do my lines come out all weird, even if LERP_RESOLUTION is as high as it can matter? Is that really just an artifact of the limited resolution (observed on a 400*400 image).
        for integral_t in 0..LERP_RESOLUTION {
            let t = integral_t as f64/LERP_RESOLUTION as f64;
            let Point2D(x, y) = self.p0 * (1.0 - t)  + self.p1 * t;
            // TODO: There's probably a way to avoid casting *here* but does allow me to maintain the known bit-width of the point coordinates. Not doing it now.
            // TODO: There's gotta be a way that doesn't involve ever indexing with y, and then x. But hey, I'm not interested in it right now!
            target[y as usize][x as usize] = self.color;
        }
    }
}