use crate::PixelGrid;

#[typetag::serde(tag = "type")]
pub trait Draw: Send + Sync {
    fn draw(&self, target: &PixelGrid);
}
