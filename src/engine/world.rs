use std::ops::Deref;
use sfml::graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::system::Vector2f;
use crate::engine::circle::Circle;
use crate::engine::engine_object::EngineObject;
use crate::utils::Vec2f;

const GRAVITY: Vec2f = Vec2f { x: 0.0, y: 1000.0 };

pub struct World {
    world_center: Vec2f,
    world_radius: f64,
    objects: Vec<Box<dyn EngineObject>>,
    sub_step_cnt: u32,
}

impl World {
    pub fn new(center: Vec2f, radius: f64) -> Self {
        World {
            world_center: center,
            world_radius: radius,
            objects: Vec::new(),
            sub_step_cnt: 1,
        }
    }

    pub fn sfml_render(&mut self, window: &mut RenderWindow) {
        // Clear window
        window.clear(Color {
            r: 32,
            g: 32,
            b: 32,
            a: 255,
        });

        // Draw world boundary
        let mut world_boundary = CircleShape::new(1.0, 64);
        // Set origin to middle of 2x2 circle (r=1), then scale to proper size
        world_boundary.set_origin(Vector2f {
            x: 1.0,
            y: 1.0,
        });
        world_boundary.set_position(Vector2f {
            x: self.world_center.x as f32,
            y: self.world_center.y as f32,
        });
        world_boundary.set_scale(Vector2f {
            x: self.world_radius as f32,
            y: self.world_radius as f32,
        });
        world_boundary.set_fill_color(Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        });
        window.draw(&world_boundary);

        for obj in &self.objects {
            if let Some(circle) = obj.as_any().downcast_ref::<Circle>() {
                let mut shape = CircleShape::new(1.0, 32);

                shape.set_origin(Vector2f {
                    x: 1.0,
                    y: 1.0,
                });
                // let mut shape = CircleShape::new(1.0, 32);
                shape.set_position(Vector2f {
                    x: circle.get_pos().x as f32,
                    y: circle.get_pos().y as f32,
                });
                shape.set_scale(Vector2f {
                    x: circle.get_radius() as f32,
                    y: circle.get_radius() as f32,
                });

                shape.set_fill_color(Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                });
                window.draw(&shape);
            }
        }
    }

    pub fn add_object(&mut self, object: Box<dyn EngineObject>) {
        self.objects.push(object);
    }

    pub fn set_sub_step_cnt(&mut self, cnt: u32) {
        self.sub_step_cnt = cnt;
    }

    pub fn tick(&mut self, dt: f64) {
        let step_dt = dt / self.sub_step_cnt as f64;

        for _ in 0..self.sub_step_cnt {
            self.apply_gravity();
            self.check_collisions(step_dt);
            self.apply_constraints();
            for i in 0..self.objects.len() {
                self.objects[i].tick(step_dt);
            }
            self.apply_constraints();
        }
    }

    fn apply_gravity(&mut self) {
        for obj in self.objects.iter_mut() {
            obj.accelerate(&GRAVITY);
        }
    }

    fn check_collisions(&mut self, _dt: f64) {
        for i in 0..self.objects.len() {
            for j in i + 1..self.objects.len() {
                let a = self.objects[i].deref();
                let b = self.objects[j].deref();
                let vec = a.get_pos() - b.get_pos();
                let dist_sq = vec.get_magnitude_squared();
                let min_dist = a.min_dist(b) + b.min_dist(a);
                // Check if distance is larger than some number to avoid moving cells with very
                // slight overlap
                if dist_sq > 0.0001 && dist_sq < min_dist * min_dist {
                    // Collision, move objects
                    let dist = dist_sq.sqrt();
                    // Get unit vector to use for direction
                    let unit_vec = vec / dist;
                    // Get ratio of masses to calculate movement
                    let r1 = a.get_mass() / (a.get_mass() + b.get_mass());
                    let r2 = b.get_mass() / (a.get_mass() + b.get_mass());
                    // Get half the movement needed, if both objects move by half, full movement done
                    let half_delta = 0.5 * 0.75 * (dist - min_dist);

                    self.objects[i].sub_pos(&(unit_vec * half_delta * r1));
                    self.objects[j].add_pos(&(unit_vec * half_delta * r2));
                }
            }
        }
    }

    fn apply_constraints(&mut self) {
        for obj in self.objects.iter_mut() {
            obj.constraint(&self.world_center, self.world_radius);
        }
    }
}