use crate::engine::world::World;
use crate::utils::Vec2;

pub trait EngineObject {
    fn draw(&self, world: &mut World);
    fn get_pos(&self) -> Vec2<u32>;
}