use std::ops::{Add, Mul, Neg, Sub};

struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    pub fn init(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn len(self) -> f64 {
        return f64::hypot(self.x, self.y);
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: other * self.x,
            y: other * self.y,
        }
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Self;

    fn mul(self, other: Vec2) -> Self::Output {
        Self {
            x: other.x * self.x,
            y: other.y * self.y,
        }
    }
}

pub struct Turtle {
    position: Vec2,
}

impl Turtle {
    pub fn init(x: f64, y: f64) -> Self {
        return Self {
            position: Vec2::init(x, y),
        };
    }
}
