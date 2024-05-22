use crate::PixelGrid;

#[typetag::serde(tag = "type")]
pub trait Draw {
    fn draw(&self, target: &mut PixelGrid);
}
