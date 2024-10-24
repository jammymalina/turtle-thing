use macroquad::prelude::*;

mod turtle;

#[macroquad::main("turtle")]
async fn main() {
    let t = turtle::Turtle::init(20.0, 20.0);
    loop {
        clear_background(BLACK);

        next_frame().await
    }
}
