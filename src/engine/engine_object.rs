use std::any::Any;
use crate::utils::{Vec2f, Vec2i};

pub trait EngineObject {
    fn draw(&self, fb: &mut Vec<u32>, dimensions: &Vec2i);
    fn get_pos(&self) -> Vec2f;
    fn get_mass(&self) -> f64;
    fn min_dist(&self, b: &dyn EngineObject) -> f64;
    fn add_pos(&mut self, pos: &Vec2f);
    fn sub_pos(&mut self, pos: &Vec2f);
    fn add_vel(&mut self, vel: &Vec2f, dt: f64);
    fn accelerate(&mut self, acceleration: &Vec2f);
    fn constraint(&mut self, center: &Vec2f, radius: f64);
    fn tick(&mut self, dt: f64);
    fn as_any(&self) -> &dyn Any;
}