use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{
    draw::Draw,
    lerp,
    point_and_color::{Color, Point2D},
    PixelGrid,
};

#[derive(Deserialize, Serialize)]
pub struct QuadraticBezierCurve {
    #[serde(rename = "points")]
    pub p: (Point2D, Point2D, Point2D),
    pub color: Color,
}

const LERP_RESOLUTION_FOR_BEZIER_CURVES: u64 = 10_000;

#[typetag::serde]
impl Draw for QuadraticBezierCurve {
    fn draw(&self, target: &PixelGrid) {
        for integral_t in 0..LERP_RESOLUTION_FOR_BEZIER_CURVES {
            let t = integral_t as f64 / LERP_RESOLUTION_FOR_BEZIER_CURVES as f64;
            let point_on_segment_from_p0_to_p1 = lerp(self.p.0, self.p.1, t);
            let point_on_segment_from_p1_to_p2 = lerp(self.p.1, self.p.2, t);
            let point_on_curve = lerp(
                point_on_segment_from_p0_to_p1,
                point_on_segment_from_p1_to_p2,
                t,
            );

            point_on_curve.draw_specifying_color(target, self.color);
            std::thread::sleep(Duration::from_micros(200));
        }
    }
}


#[derive(Deserialize, Serialize)]
pub struct CubicBezierCurve {
    #[serde(rename = "points")]
    pub p: (Point2D, Point2D, Point2D, Point2D),
    pub color: Color,
}

#[typetag::serde]
impl Draw for CubicBezierCurve {
    fn draw(&self, target: &PixelGrid) {
        for integral_t in 0..LERP_RESOLUTION_FOR_BEZIER_CURVES {
            let t = integral_t as f64 / LERP_RESOLUTION_FOR_BEZIER_CURVES as f64;

            let point_on_segment_from_p0_to_p1 = lerp(self.p.0, self.p.1, t);
            let point_on_segment_from_p1_to_p2 = lerp(self.p.1, self.p.2, t);
            let point_on_segment_from_p2_to_p3 = lerp(self.p.2, self.p.3, t);

            let point_on_first_lerp = lerp(point_on_segment_from_p0_to_p1, point_on_segment_from_p1_to_p2, t);
            let point_on_second_lerp = lerp(point_on_segment_from_p1_to_p2, point_on_segment_from_p2_to_p3, t);

            let point_on_curve = lerp(point_on_first_lerp, point_on_second_lerp, t);

            point_on_curve.draw_specifying_color(target, self.color);
            std::thread::sleep(Duration::from_micros(200));

        }
    }
}