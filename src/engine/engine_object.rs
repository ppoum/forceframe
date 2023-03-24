use crate::utils::Vec2;

pub trait EngineObject {
    fn draw(&self, fb: &mut Vec<u32>, width: u32, height: u32);
    fn get_pos(&self) -> Vec2<u32>;
}