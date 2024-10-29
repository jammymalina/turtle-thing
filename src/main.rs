use macroquad::prelude::*;
use std::sync::Arc;
use turtle::{CoordinateTransform, Turtle, TurtleConfig, TurtleScreen};

mod turtle;

fn update(turtle: &mut Turtle, dt: f32) {
    turtle.clear();

    for _ in 0..3 {
        turtle.forward(100.0);
        turtle.left(30.0);
        turtle.to_origin();
    }
}

#[macroquad::main("turtle")]
async fn main() {
    request_new_screen_size(1280.0, 720.0);

    let mut transform = CoordinateTransform::new(screen_width(), screen_height(), 1.0);

    let screen = Arc::new(TurtleScreen::new());

    let turtle_config = TurtleConfig {
        ..Default::default()
    };
    let mut turtle = Turtle::new(Vec2::new(0.0, 0.0), screen.clone(), &turtle_config);

    update(&mut turtle, 0.0);
    loop {
        clear_background(WHITE);

        let dt = get_frame_time();

        screen.present(&transform);

        transform.update_screen_size(screen_width(), screen_height());
        if is_quit_requested() {
            break;
        }

        next_frame().await
    }
}
