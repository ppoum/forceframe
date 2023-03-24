use crate::engine::engine_object::EngineObject;
use crate::engine::world::World;
use crate::utils::Vec2;

pub struct Circle {
    pos: Vec2<u32>,
    radius: f64,
}

impl Circle {
    pub fn new(pos: Vec2<u32>, radius: f64) -> Self {
        Circle { pos, radius }
    }
}

impl EngineObject for Circle {
    fn draw(&self, world: &mut World) {
        // Iterate over bounding-box area, fill in when needed
        let i_rad = self.radius.ceil() as i64;
        let mut distance: Vec2<u32> = Vec2::new(0, 0);
        for dx in -i_rad..i_rad {
            for dy in -i_rad..i_rad {
                let x = self.pos.x as i64 + dx;
                let y = self.pos.y as i64 + dy;
                distance.x = x as u32;
                distance.y = y as u32;

                if distance.get_magnitude() <= self.radius {
                    world.draw_pixel(distance.x, distance.y, 0xFFFFFF);
                }

            }
        }
    }

    fn get_pos(&self) -> Vec2<u32> {
        self.pos
    }
}