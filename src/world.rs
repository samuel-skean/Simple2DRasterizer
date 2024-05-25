use crate::draw::Draw;

pub type World = Vec<Box<dyn Draw>>;

#[typetag::serde]
impl Draw for World {
    fn draw(&self, target: &crate::PixelGrid) {
        for item in self {
            item.draw(target);
        }
    }
}
