use std::ops::{Add, Mul, Neg, Sub};

use macroquad::prelude::*;

#[derive(Debug, Copy, Clone)]
struct Vec2 {
    pub x: f32,
    pub y: f32,
}

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

pub struct Turtle {
    position: Vec2,
    orientation: Vec2,
    line_thickness: f32,
    line_color: Color,
    angle_offset: f32,
    angle_orientation: f32,
}

#[allow(dead_code)]
impl Turtle {
    pub fn init(x: f32, y: f32) -> Self {
        return Self {
            position: Vec2::init(x, y),
            orientation: Vec2::init(0.0, 1.0),
            line_thickness: 1.0,
            line_color: BLACK,
            angle_offset: 360.0 / 4.0,
            angle_orientation: -1.0,
        };
    }

    pub fn set_x(&mut self, x: f32) {
        self.position.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.position.y = y;
    }

    pub fn set_position(&mut self, destination: Vec2) {
        draw_line(
            self.position.x,
            self.position.y,
            destination.x,
            destination.y,
            self.line_thickness,
            self.line_color,
        );
        self.position = destination;
    }

    pub fn heading(&self) -> f32 {
        let result = f32::to_degrees(f32::atan2(self.orientation.y, self.orientation.x)) % 360.0;
        (self.angle_offset + self.angle_orientation * result) % 360.0
    }

    pub fn set_heading(&mut self, to_angle: f32) {
        let mut angle: f32 = (to_angle - self.heading()) * self.angle_orientation;
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
