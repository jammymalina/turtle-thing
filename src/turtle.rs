use std::ops::{Add, Mul, Neg, Sub};
use std::sync::{Arc, RwLock};

use macroquad::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Vec2 {
    pub fn init(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn len(self) -> f32 {
        return f32::hypot(self.x, self.y);
    }

    pub fn rotate(self, degrees: f32) -> Vec2 {
        let perp = Vec2::init(-self.y, self.x);
        let radians = f32::to_radians(degrees);

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

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
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
    Line {
        a: Vec2,
        b: Vec2,
        thickness: f32,
        color: Color,
    },
}

pub struct TurtleScreen {
    shapes: Arc<RwLock<Vec<Shape>>>,
}

#[allow(dead_code)]
impl TurtleScreen {
    pub fn init() -> Self {
        Self {
            shapes: Arc::new(RwLock::new(vec![])),
        }
    }

    pub fn present(&self) {
        if let Ok(shapes) = self.shapes.read() {
            shapes.iter().for_each(|shape| match shape {
                Shape::Line {
                    a,
                    b,
                    thickness,
                    color,
                } => draw_line(a.x, a.y, b.x, b.y, *thickness, *color),
            });
        }
    }

    pub fn draw_line(&self, a: Vec2, b: Vec2, thickness: f32, color: Color) {
        if let Ok(mut shapes) = self.shapes.write() {
            shapes.push(Shape::Line {
                a,
                b,
                thickness,
                color,
            });
        }
    }

    pub fn clear(&self) {
        if let Ok(mut shapes) = self.shapes.write() {
            shapes.clear();
        }
    }
}

pub struct TurtleConfig {
    pub start_orientation: Vec2,
    pub line_thickness: f32,
    pub line_color: Color,
    pub angle_offset: f32,
    pub angle_orientation: f32,
}

impl Default for TurtleConfig {
    fn default() -> Self {
        Self {
            start_orientation: Vec2::init(0.0, 1.0),
            line_thickness: 1.0,
            line_color: BLACK,
            angle_offset: 360.0 / 4.0,
            angle_orientation: -1.0,
        }
    }
}

pub struct Turtle {
    position: Vec2,
    orientation: Vec2,
    line_thickness: f32,
    line_color: Color,
    angle_offset: f32,
    angle_orientation: f32,
    screen: Arc<TurtleScreen>,
}

#[allow(dead_code)]
impl Turtle {
    pub fn init(x: f32, y: f32, screen: Arc<TurtleScreen>, config: &TurtleConfig) -> Self {
        return Self {
            position: Vec2::init(x, y),
            orientation: config.start_orientation,
            line_thickness: config.line_thickness,
            line_color: config.line_color,
            angle_offset: config.angle_offset,
            angle_orientation: config.angle_orientation,
            screen,
        };
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn set_position(&mut self, destination: Vec2) {
        self.screen.draw_line(
            self.position,
            destination,
            self.line_thickness,
            self.line_color,
        );
        self.position = destination;
    }

    pub fn get_angle(&self) -> f32 {
        let result = f32::to_degrees(f32::atan2(self.orientation.y, self.orientation.x)) % 360.0;
        (self.angle_offset + self.angle_orientation * result) % 360.0
    }

    pub fn set_angle(&mut self, to_angle: f32) {
        let mut angle: f32 = (to_angle - self.get_angle()) * self.angle_orientation;
        angle = (angle + 360.0 / 2.0) % 360.0 - 360.0 / 2.0;
        self.rotate(angle)
    }

    pub fn forward(&mut self, distance: f32) {
        self.go(distance);
    }

    pub fn backward(&mut self, distance: f32) {
        self.go(-distance);
    }

    pub fn left(&mut self, angle: f32) {
        self.rotate(angle);
    }

    pub fn right(&mut self, angle: f32) {
        self.rotate(-angle);
    }

    fn go(&mut self, distance: f32) {
        let destination = self.position + self.orientation * distance;
        self.set_position(destination);
    }

    fn rotate(&mut self, angle: f32) {
        self.orientation = self.orientation.rotate(angle);
    }
}
