use num_traits::Num;
use std::ops::{Add, Mul, Neg, Sub};

struct Vec2<T>
where
    T: Num + Into<f64> + Copy,
{
    x: T,
    y: T,
}

impl<T: Num + Into<f64> + Copy> Vec2<T> {
    pub fn init(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn len(self) -> f64 {
        return f64::hypot(self.x.into(), self.y.into());
    }
}

impl<T: Neg<Output = T> + Num + Into<f64> + Copy> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Add<Output = T> + Num + Into<f64> + Copy> Add<Vec2<T>> for Vec2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T> + Num + Into<f64> + Copy> Sub<Vec2<T>> for Vec2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Mul<Output = T> + Num + Into<f64> + Copy> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Self {
            x: other * self.x,
            y: other * self.y,
        }
    }
}

impl<T: Mul<Output = T> + Num + Into<f64> + Copy> Mul<Vec2<T>> for Vec2<T> {
    type Output = Self;

    fn mul(self, other: Vec2<T>) -> Self::Output {
        Self {
            x: other.x * self.x,
            y: other.y * self.y,
        }
    }
}

pub struct Turtle<T>
where
    T: Num + Into<f64> + Copy,
{
    position: Vec2<T>,
}

impl<T: Num + Into<f64> + Copy> Turtle<T> {
    pub fn init(x: T, y: T) -> Self {
        return Self {
            position: Vec2::init(x, y),
        };
    }
}
