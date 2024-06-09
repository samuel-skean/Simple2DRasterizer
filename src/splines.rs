use itertools::Itertools;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    bezier_curves::CubicBezierCurve,
    draw::Draw,
    pixel_grid::PixelGrid,
    point_and_color::{Color, Point2D},
};

fn deserialize_cubic_spline_points<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<Point2D>, D::Error> {
    let points = Vec::<Point2D>::deserialize(deserializer)?;
    if (points.len() - 1) % 3 == 0 {
        Ok(points)
    } else {
        Err(serde::de::Error::invalid_value(serde::de::Unexpected::Seq,
            &"a valid sequence of points for a cubic spline - a sequence of at least length 4, which is 1 greater than a multiple of 3"))
    }
}

#[derive(Deserialize, Serialize)]
pub struct CubicBezierSpline {
    #[serde(deserialize_with = "deserialize_cubic_spline_points")]
    points: Vec<Point2D>,
    color: Color,
}

#[typetag::serde]
impl Draw for CubicBezierSpline {
    fn draw(&self, target: &PixelGrid) {
        for points in self
            .points
            .clone()
            .into_iter()
            .tuple_windows::<(Point2D, Point2D, Point2D, Point2D)>()
            .step_by(3)
        {
            let curve = CubicBezierCurve {
                p: points,
                color: self.color,
            };
            curve.draw(target);
        }
    }
}
