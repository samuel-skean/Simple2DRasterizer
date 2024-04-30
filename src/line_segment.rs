use crate::{draw::Draw, lerp, vec::{Color, Point2D}, PixelGrid};

pub struct LineSegment {
    pub p0: Point2D,
    pub p1: Point2D,
    pub color: Color,
}

const LERP_RESOLUTION_FOR_LINE_SEGMENTS: u64 = 1_000;

impl Draw for LineSegment {
    fn draw(&self, target: &mut PixelGrid) {
        // TODO: Why do my lines come out all weird, even if LERP_RESOLUTION is as high as it can matter? Is that really just an artifact of the limited resolution (observed on a 400*400 image).
        for integral_t in 0..LERP_RESOLUTION_FOR_LINE_SEGMENTS {
            let t = integral_t as f64/LERP_RESOLUTION_FOR_LINE_SEGMENTS as f64;
            let Point2D(x, y) = lerp(self.p0, self.p1, t);
            // TODO: There's probably a way to avoid casting *here* but does allow me to maintain the known bit-width of the point coordinates.
            // Also applies to bezier_curve.rs. Not doing it now.
            // TODO: There's gotta be a way that doesn't involve ever indexing with y, and then x. Also applies to bezier_curve.rs.
            // But hey, I'm not interested in it right now!
            target[y as usize][x as usize] = self.color;
        }
    }
}