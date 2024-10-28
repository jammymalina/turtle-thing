use macroquad::prelude::*;
use std::sync::Arc;
use turtle::{Turtle, TurtleConfig, TurtleScreen};

mod turtle;

fn update(turtle: &mut Turtle, dt: f32) {
    turtle.clear();

    // let rotation_speed = 60_f32;

    // turtle.left(rotation_speed * dt);
    let n = 1;
    for _ in 0..n {
        turtle.forward(200.0);
        turtle.left(90.0);
        turtle.forward(200.0);
        turtle.left(90.0);
        turtle.forward(200.0);

        turtle.to_origin();
    }
}

#[macroquad::main("turtle")]
async fn main() {
    let screen_width = 1280.0_f32;
    let screen_height = 720.0_f32;
    request_new_screen_size(screen_width, screen_height);

    let screen = Arc::new(TurtleScreen::init());

    let turtle_config = TurtleConfig {
        ..Default::default()
    };
    let mut turtle = turtle::Turtle::init(
        Vec2::new(screen_width / 2.0, screen_height / 2.0),
        screen.clone(),
        &turtle_config,
    );

    update(&mut turtle, 0.0);
    loop {
        clear_background(WHITE);

        let dt = get_frame_time();

        screen.present();
        next_frame().await
    }
}
