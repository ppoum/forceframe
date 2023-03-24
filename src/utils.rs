#[derive(Copy, Clone)]
pub struct Vec2f {
    pub x: f64,
    pub y: f64,
}

impl Vec2f {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2f { x, y }
    }

    pub fn get_magnitude_squared(&self) -> f64 {
        self.x * self.x + self.y + self.y
    }

    pub fn get_magnitude(&self) -> f64 {
        self.get_magnitude_squared().sqrt()
    }
}

#[derive(Copy, Clone)]
pub struct Vec2i {
    pub x: i64,
    pub y: i64
}

impl Vec2i {
    pub fn new(x: i64, y: i64) -> Self {
        Vec2i { x, y }
    }

    pub fn get_magnitude_squared(&self) -> i64 {
        self.x * self.x + self.y + self.y
    }

    pub fn get_magnitude(&self) -> f64 {
        (self.get_magnitude_squared() as f64).sqrt()
    }
}