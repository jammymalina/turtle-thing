use macroquad::prelude::*;

mod turtle;

#[macroquad::main("turtle")]
async fn main() {
    request_new_screen_size(1280.0, 720.0);

    let t = turtle::Turtle::init(20.0, 20.0);
    loop {
        clear_background(BLACK);

        next_frame().await
    }
}
