use macroquad::prelude::*;

#[macroquad::main("turtle")]
async fn main() {
    loop {
        clear_background(BLACK);

        next_frame().await
    }
}
