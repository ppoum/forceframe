use std::ops;

#[derive(Copy, Clone, Default)]
pub struct Vec2f {
    pub x: f64,
    pub y: f64,
}

impl Vec2f {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2f { x, y }
    }

    pub fn get_magnitude_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn get_magnitude(&self) -> f64 {
        self.get_magnitude_squared().sqrt()
    }
}

// + op
impl ops::Add<Vec2f> for Vec2f {
    type Output = Vec2f;

    fn add(self, rhs: Vec2f) -> Self::Output {
        Vec2f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// - op
impl ops::Sub<Vec2f> for Vec2f {
    type Output = Vec2f;

    fn sub(self, rhs: Vec2f) -> Self::Output {
        Vec2f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Neg for Vec2f {
    type Output = Vec2f;

    fn neg(self) -> Self::Output {
        Vec2f {
            x: -self.x,
            y: -self.y,
        }
    }
}

// * op
impl ops::Mul<f64> for Vec2f {
    type Output = Vec2f;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2f {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

// '/' op
impl ops::Div<f64> for Vec2f {
    type Output = Vec2f;

    fn div(self, rhs: f64) -> Self::Output {
        Vec2f {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

// += op
impl ops::AddAssign<Vec2f> for Vec2f {
    fn add_assign(&mut self, rhs: Vec2f) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// -= op
impl ops::SubAssign<Vec2f> for Vec2f {
    fn sub_assign(&mut self, rhs: Vec2f) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[derive(Copy, Clone)]
pub struct Vec2i {
    pub x: i64,
    pub y: i64,
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