use crate::PixelGrid;

#[typetag::serde(tag = "type")]
pub trait Draw: Sync + Send {
    fn draw(&self, target: &PixelGrid);
}
