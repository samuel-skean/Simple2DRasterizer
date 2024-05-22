use crate::{
    draw::Draw,
    lerp,
    point_and_color::{Color, Point2D},
    PixelGrid,
};

pub struct QuadraticBezierCurve {
    pub p0: Point2D,
    pub p1: Point2D,
    pub p2: Point2D,
    pub color: Color,
}

const LERP_RESOLUTION_FOR_BEZIER_CURVES: u64 = 10_000;

impl Draw for QuadraticBezierCurve {
    fn draw(&self, target: &mut PixelGrid) {
        for integral_t in 0..LERP_RESOLUTION_FOR_BEZIER_CURVES {
            let t = integral_t as f64 / LERP_RESOLUTION_FOR_BEZIER_CURVES as f64;
            let point_along_segment_from_p0_to_p1 = lerp(self.p0, self.p1, t);
            let point_along_segment_from_p1_to_p2 = lerp(self.p1, self.p2, t);
            let point_on_curve = lerp(
                point_along_segment_from_p0_to_p1,
                point_along_segment_from_p1_to_p2,
                t,
            );

            point_on_curve.draw_specifying_color(target, self.color);
        }
    }
}
