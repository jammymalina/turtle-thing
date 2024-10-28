use macroquad::prelude::*;
use turtle::Turtle;

mod turtle;

fn draw(turtle: &mut Turtle) {
    for _ in 0..3 {
        turtle.forward(100.0);
        turtle.left(90.0);
    }
}

#[macroquad::main("turtle")]
async fn main() {
    let screen_width = 1280.0_f32;
    let screen_height = 720.0_f32;
    request_new_screen_size(screen_width, screen_height);

    let mut turtle = turtle::Turtle::init(screen_width / 2.0, screen_height / 2.0);
    loop {
        clear_background(WHITE);

        draw(&mut turtle);

        next_frame().await
    }
}
