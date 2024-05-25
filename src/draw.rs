use crate::PixelGrid;

#[typetag::serde(tag = "type")]
pub trait Draw: Sync {
    fn draw(&self, target: &PixelGrid);
}
