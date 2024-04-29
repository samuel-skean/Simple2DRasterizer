use crate::PixelGrid;

pub trait Draw {
    fn draw(&self, target: &mut PixelGrid);
}