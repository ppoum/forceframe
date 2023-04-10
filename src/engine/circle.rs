use std::any::Any;
use crate::engine::engine_object::EngineObject;
use crate::utils::{Vec2f, Vec2i};

#[derive(Copy, Clone)]
pub struct Circle {
    pos: Vec2f,
    last_pos: Vec2f,
    radius: f64,
    acceleration: Vec2f,
}

impl Circle {
    pub fn new(pos: Vec2f, radius: f64) -> Self {
        Circle {
            pos,
            last_pos: pos,
            radius,
            acceleration: Vec2f::default(),
        }
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}

impl EngineObject for Circle {
    fn draw(&self, fb: &mut Vec<u32>, dimensions: &Vec2i) {
        // Iterate over bounding-box area, fill in when needed
        let i_rad = self.radius.ceil() as i64;
        for dx in -i_rad..i_rad {
            for dy in -i_rad..i_rad {
                let x = self.pos.x + dx as f64;
                if x < 0.0 || x >= dimensions.x as f64 { continue; }
                let y = self.pos.y + dy as f64;
                if y < 0.0 || y >= dimensions.y as f64 { continue; }

                let dist = ((dx * dx + dy * dy) as f64).sqrt();
                if dist <= self.radius {
                    let idx = y as u32 * dimensions.x as u32 + x as u32;
                    fb[idx as usize] = 0xFFFFFF;
                }
            }
        }
    }

    fn get_pos(&self) -> Vec2f {
        self.pos
    }

    fn get_last_pos(&self) -> Vec2f { self.last_pos }

    fn get_mass(&self) -> f64 {
        std::f64::consts::TAU * self.radius
    }

    fn min_dist(&self, _b: &dyn EngineObject) -> f64 {
        // Other object not needed, min_dist is always radius for circle
        self.radius
    }

    fn add_pos(&mut self, pos: &Vec2f) {
        self.pos += *pos;
    }

    fn sub_pos(&mut self, pos: &Vec2f) {
        self.pos -= *pos;
    }

    fn add_vel(&mut self, vel: &Vec2f, dt: f64) {
        self.last_pos -= *vel * dt;
    }

    fn accelerate(&mut self, acceleration: &Vec2f) {
        self.acceleration += *acceleration;
    }

    fn constraint(&mut self, center: &Vec2f, world_radius: f64) {
        let v = *center - self.pos;
        let dist = v.get_magnitude();

        if dist > (world_radius - self.radius) {
            let unit_v = v / dist;
            self.pos = *center - unit_v * (world_radius - self.radius);
        }
    }

    fn tick(&mut self, dt: f64) {
        // Update pos
        let dx = self.pos - self.last_pos;
        self.last_pos = self.pos;
        self.pos += dx + self.acceleration * dt * dt;
        self.acceleration = Vec2f::default();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
