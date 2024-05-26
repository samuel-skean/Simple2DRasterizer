use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    bezier_curves::CubicBezierCurve,
    draw::Draw,
    pixel_grid::PixelGrid,
    point_and_color::{Color, Point2D},
};

#[derive(Deserialize, Serialize)]
pub struct CubicBezierSpline {
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
