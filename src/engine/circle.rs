use crate::engine::engine_object::EngineObject;
use crate::utils::Vec2i;

pub struct Circle {
    pos: Vec2i,
    radius: f64,
}

impl Circle {
    pub fn new(pos: Vec2i, radius: f64) -> Self {
        Circle { pos, radius }
    }
}

impl EngineObject for Circle {
    fn draw(&self, fb: &mut Vec<u32>, width: u32, height: u32) {
        // Iterate over bounding-box area, fill in when needed
        let i_rad = self.radius.ceil() as i64;
        for dx in -i_rad..i_rad {
            for dy in -i_rad..i_rad {
                let x = self.pos.x + dx;
                if x < 0 || x >= height as i64 { continue; }
                let y = self.pos.y + dy;
                if y < 0 || y >= height as i64 { continue; }

                let dist = ((dx * dx + dy * dy) as f64).sqrt();
                if dist <= self.radius {
                    let idx = y as u32 * width + x as u32;
                    fb[idx as usize] = 0xFFFFFF;
                }
            }
        }
    }

    fn get_pos(&self) -> Vec2i {
        self.pos
    }
}