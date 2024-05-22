use crate::draw::Draw;

pub type World = Vec<Box<dyn Draw>>;

impl Draw for World {
    fn draw(&self, target: &mut crate::PixelGrid) {
        for item in self {
            item.draw(target);
        }
    }
}
