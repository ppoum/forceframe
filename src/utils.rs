use std::ops::{Add, Mul};

#[derive(Copy, Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Copy + Mul + Into<f64>> Vec2<T>
    where <T as Mul>::Output: Add<Output=T> {  // T*T=T (same type)

    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    pub fn get_magnitude_square(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    pub fn get_magnitude(&self) -> f64 {
        self.get_magnitude_square().into().sqrt()
    }
}
