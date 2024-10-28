use std::collections::HashMap;
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

    pub fn rotate(self, degrees: f64) -> Vec2 {
        let perp = Vec2::init(-self.y, self.x);
        let radians = degrees * std::f64::consts::PI / 180.0;

        let sin = radians.sin();
        let cos = radians.cos();
        Vec2::init(self.x * cos + perp.x * sin, self.y * cos + perp.y * sin)
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

enum Shape {
    Polygon(Vec<(f64, f64)>),
}

struct TurtleScreen {
    shapes: HashMap<&'static str, Shape>,
}

impl TurtleScreen {
    pub fn init() -> Self {
        Self {
            shapes: HashMap::from([
                (
                    "arrow",
                    Shape::Polygon(vec![(-10.0, 0.0), (10.0, 0.0), (0.0, 10.0)]),
                ),
                (
                    "turtle",
                    Shape::Polygon(vec![
                        (0.0, 16.0),
                        (-2.0, 14.0),
                        (-1.0, 10.0),
                        (-4.0, 7.0),
                        (-7.0, 9.0),
                        (-9.0, 8.0),
                        (-6.0, 5.0),
                        (-7.0, 1.0),
                        (-5.0, -3.0),
                        (-8.0, -6.0),
                        (-6.0, -8.0),
                        (-4.0, -5.0),
                        (0.0, -7.0),
                        (4.0, -5.0),
                        (6.0, -8.0),
                        (8.0, -6.0),
                        (5.0, -3.0),
                        (7.0, 1.0),
                        (6.0, 5.0),
                        (9.0, 8.0),
                        (7.0, 9.0),
                        (4.0, 7.0),
                        (1.0, 10.0),
                        (2.0, 14.0),
                    ]),
                ),
                (
                    "circle",
                    Shape::Polygon(vec![
                        (10.0, 0.0),
                        (9.51, 3.09),
                        (8.09, 5.88),
                        (5.88, 8.09),
                        (3.09, 9.51),
                        (0.0, 10.0),
                        (-3.09, 9.51),
                        (-5.88, 8.09),
                        (-8.09, 5.88),
                        (-9.51, 3.09),
                        (-10.0, 0.0),
                        (-9.51, -3.09),
                        (-8.09, -5.88),
                        (-5.88, -8.09),
                        (-3.09, -9.51),
                        (-0.00, -10.00),
                        (3.09, -9.51),
                        (5.88, -8.09),
                        (8.09, -5.88),
                        (9.51, -3.09),
                    ]),
                ),
                (
                    "square",
                    Shape::Polygon(vec![
                        (10.0, -10.0),
                        (10.0, 10.0),
                        (-10.0, 10.0),
                        (-10.0, -10.0),
                    ]),
                ),
                (
                    "triangle",
                    Shape::Polygon(vec![(10.0, -5.77), (0.0, 11.55), (-10.0, -5.77)]),
                ),
                (
                    "classic",
                    Shape::Polygon(vec![(0.0,0.0),(-5.0,-9.0),(0.0,-7.0),(5.0,-9.0)])
                )
            ]),
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
