use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{
    draw::Draw,
    lerp,
    point_and_color::{Color, Point2D},
    PixelGrid,
};

#[derive(Deserialize, Serialize)]
pub struct LineSegment {
    #[serde(rename = "points")]
    pub p: (Point2D, Point2D),
    #[serde(with = "crate::serde_sdl_color")]
    pub color: Color,
}

const LERP_RESOLUTION_FOR_LINE_SEGMENTS: u64 = 1_000;

#[typetag::serde]
impl Draw for LineSegment {
    fn draw(&self, target: &PixelGrid) {
        for integral_t in 0..LERP_RESOLUTION_FOR_LINE_SEGMENTS {
            let t = integral_t as f64 / LERP_RESOLUTION_FOR_LINE_SEGMENTS as f64;
            let point_on_line = lerp(self.p.0, self.p.1, t);
            point_on_line.draw_specifying_color(target, self.color);

            if cfg!(feature = "leisurely-drawing") {
                std::thread::sleep(Duration::from_millis(1));
            }
        }
    }
}
