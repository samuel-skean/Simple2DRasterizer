use crate::{draw::Draw, lerp, point_and_color::{Color, Point2D}, PixelGrid};

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
            let point_on_line = lerp(self.p0, self.p1, t);
            point_on_line.draw_specifying_color(target, self.color);
        }
    }
}
