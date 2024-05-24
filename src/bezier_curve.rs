use crate::{
    draw::Draw,
    lerp,
    point_and_color::{Color, Point2D},
    PixelGrid,
};

pub struct BezierCurve<const degree: usize> 
    where [Point2D; degree + 1]: Sized
{
    pub points: [Point2D; degree + 1],
    pub color: Color,
}

pub type QuadraticBezierCurve = BezierCurve<2>;

const LERP_RESOLUTION_FOR_BEZIER_CURVES: u64 = 10_000;

impl <const degree: usize> Draw for BezierCurve<degree> 
where [Point2D; degree + 1]: Sized, [Point2D; degree - 1]: Sized
{
    fn draw(&self, target: &mut PixelGrid) {
        for integral_t in 0..LERP_RESOLUTION_FOR_BEZIER_CURVES {
            let t = integral_t as f64 / LERP_RESOLUTION_FOR_BEZIER_CURVES as f64;
            for num_lerps in (0..degree + 1).rev() {
                let mut points_along_segments = [Point2D(0.0, 0.0); num_lerps]; // At this point I give up.
                for (i, point) in points_along_segments.iter_mut().enumerate() {
                    *point = lerp(self.points[i], self.points[i + 1], t);
                }
                if num_lerps == 1 {
                    points_along_segments[0].draw_specifying_color(target, self.color)
                }
            }
        }
    }
}
