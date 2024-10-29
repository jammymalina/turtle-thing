use std::sync::{Arc, RwLock};

use macroquad::prelude::*;

pub struct CoordinateTransform {
    screen_width: f32,
    screen_height: f32,
    scale: f32,
}

#[allow(dead_code)]
impl CoordinateTransform {
    pub fn new(screen_width: f32, screen_height: f32, scale: f32) -> Self {
        Self {
            screen_width,
            screen_height,
            scale,
        }
    }

    /// Convert a single point from centered to screen coordinates
    pub fn to_screen(&self, p: Vec2) -> Vec2 {
        let screen_x = (p.x * self.scale) + (self.screen_width / 2.0);
        let screen_y = (-p.y * self.scale) + (self.screen_height / 2.0);
        Vec2::new(screen_x, screen_y)
    }

    /// Convert a single point from screen to centered coordinates
    pub fn to_centered(&self, p: Vec2) -> Vec2 {
        let x = (p.x - (self.screen_width / 2.0)) / self.scale;
        let y = -(p.y - (self.screen_height / 2.0)) / self.scale;
        Vec2::new(x, y)
    }

    /// Update screen dimensions (useful for window resize)
    pub fn update_screen_size(&mut self, width: f32, height: f32) {
        self.screen_width = width;
        self.screen_height = height;
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
    pub fn new() -> Self {
        Self {
            shapes: Arc::new(RwLock::new(vec![])),
        }
    }

    pub fn present(&self, coordinate_transform: &CoordinateTransform) {
        if let Ok(shapes) = self.shapes.read() {
            shapes.iter().for_each(|shape| match shape {
                Shape::Line {
                    a,
                    b,
                    thickness,
                    color,
                } => {
                    let screen_a = coordinate_transform.to_screen(*a);
                    let screen_b = coordinate_transform.to_screen(*b);
                    draw_line(
                        screen_a.x, screen_a.y, screen_b.x, screen_b.y, *thickness, *color,
                    );
                }
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

#[derive(Clone)]
pub struct TurtleConfig {
    pub start_rotation: Vec2,
    pub line_thickness: f32,
    pub line_color: Color,
    pub angle_offset: f32,
    pub angle_rotation: f32,
}

impl Default for TurtleConfig {
    fn default() -> Self {
        Self {
            start_rotation: Vec2::new(0.0, 1.0),
            line_thickness: 1.0,
            line_color: BLACK,
            angle_offset: 360.0 / 4.0,
            angle_rotation: -1.0,
        }
    }
}

pub struct Turtle {
    position: Vec2,
    origin: Vec2,
    rotation: Vec2,
    line_thickness: f32,
    line_color: Color,
    angle_offset: f32,
    angle_rotation: f32,
    original_config: TurtleConfig,
    screen: Arc<TurtleScreen>,
}

#[allow(dead_code)]
impl Turtle {
    pub fn new(origin: Vec2, screen: Arc<TurtleScreen>, config: &TurtleConfig) -> Self {
        return Self {
            position: origin,
            origin,
            rotation: config.start_rotation.normalize(),
            line_thickness: config.line_thickness,
            line_color: config.line_color,
            angle_offset: config.angle_offset,
            angle_rotation: config.angle_rotation,
            original_config: config.clone(),
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

    pub fn teleport(&mut self, destination: Vec2) {
        self.position = destination;
    }

    pub fn get_origin(&self) -> Vec2 {
        self.origin
    }

    pub fn to_origin(&mut self) {
        self.teleport(self.origin);
    }

    pub fn get_angle(&self) -> f32 {
        let result = f32::to_degrees(self.rotation.to_angle()).rem_euclid(360.0);
        (self.angle_offset + self.angle_rotation * result).rem_euclid(360.0)
    }

    pub fn set_angle(&mut self, to_angle: f32) {
        let mut angle: f32 = (to_angle - self.get_angle()) * self.angle_rotation;
        angle = (angle + 360.0 / 2.0).rem_euclid(360.0) - 360.0 / 2.0;
        self.rotate(angle)
    }

    pub fn reset(&mut self) {
        self.to_origin();
        self.rotation = self.original_config.start_rotation.normalize();
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

    pub fn clear(&self) {
        self.screen.clear();
    }

    fn go(&mut self, distance: f32) {
        let destination = self.position + self.rotation * distance;
        self.set_position(destination);
    }

    fn rotate(&mut self, angle: f32) {
        self.rotation = self
            .rotation
            .rotate(Vec2::from_angle(angle.to_radians()))
            .normalize();
    }
}
