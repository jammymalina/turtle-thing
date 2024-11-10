use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use turtle::{CoordinateTransform, Turtle, TurtleConfig, TurtleScreen};

mod turtle;

struct IntervalChecker {
    last_interval: u64,
    interval_ms: u64,
}

impl IntervalChecker {
    fn new(interval_ms: u64) -> Self {
        Self {
            last_interval: 0,
            interval_ms,
        }
    }

    fn should_run(&mut self, time_ms: u64) -> bool {
        let current_interval = time_ms / self.interval_ms;
        if current_interval > self.last_interval {
            self.last_interval = current_interval;
            true
        } else {
            false
        }
    }
}

trait TurtleBehaviour {
    fn update(&mut self, dt: f32, time_passed_ms: u64);
}

struct Spiral {
    interval_checker: IntervalChecker,
    turtle: Turtle,
}

impl Spiral {
    fn new(turtle: Turtle, interval_ms: u64) -> Self {
        Self {
            interval_checker: IntervalChecker::new(interval_ms),
            turtle,
        }
    }
}

impl TurtleBehaviour for Spiral {
    fn update(&mut self, _dt: f32, time_passed_ms: u64) {
        static COUNTER: AtomicU32 = AtomicU32::new(0);

        if !self.interval_checker.should_run(time_passed_ms) {
            return;
        }

        let count = COUNTER.fetch_add(1, Ordering::Relaxed);
        if count > 1000 {
            return;
        }


        self.turtle.forward(count as f32 / 100.0);
        self.turtle.right(2.0);
    }
}

fn update(dt: f32, turtle_behaviours: &mut [Box<dyn TurtleBehaviour>]) {
    static TIME_MS: AtomicU64 = AtomicU64::new(0);

    let time_passed_ms = TIME_MS.fetch_add((1000.0 * dt).floor() as u64, Ordering::Relaxed);

    turtle_behaviours
        .iter_mut()
        .for_each(|b| b.update(dt, time_passed_ms));
}

#[macroquad::main("turtle")]
async fn main() {
    request_new_screen_size(1280.0, 720.0);

    let mut transform = CoordinateTransform::new(screen_width(), screen_height(), 1.0);
    let screen = Arc::new(TurtleScreen::new());

    let turtle_config = TurtleConfig {
        ..Default::default()
    };

    let spiral = Spiral::new(
        Turtle::new(Vec2::new(0.0, 0.0), screen.clone(), &turtle_config),
        50,
    );

    let mut turtle_behaviours: Vec<Box<dyn TurtleBehaviour>> = vec![Box::new(spiral)];

    loop {
        clear_background(WHITE);

        let dt = get_frame_time();

        update(dt, &mut turtle_behaviours);

        screen.present(&transform);

        transform.update_screen_size(screen_width(), screen_height());
        if is_quit_requested() {
            break;
        }

        next_frame().await
    }
}
