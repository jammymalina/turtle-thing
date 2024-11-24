use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use turtle::{CoordinateTransform, Turtle, TurtleCommand, TurtleConfig, TurtleScreen};

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

enum TurtleCommandsAnimationAutoReset {
    Disabled,
    Enabled { reset_timeout_ms: u64 },
}

struct TurtleCommandsAnimator {
    turtle: Turtle,
    interval_checker: IntervalChecker,
    auto_reset: TurtleCommandsAnimationAutoReset,
    start_reset_timestamp: Option<Instant>,
    commands: Vec<TurtleCommand>,
    current_command_index: usize,
}

impl TurtleCommandsAnimator {
    fn new(
        turtle: Turtle,
        commands: &[TurtleCommand],
        interval_ms: u64,
        auto_reset: TurtleCommandsAnimationAutoReset,
    ) -> Self {
        Self {
            turtle,
            interval_checker: IntervalChecker::new(interval_ms),
            auto_reset,
            start_reset_timestamp: None,
            commands: commands.to_vec(),
            current_command_index: 0,
        }
    }

    fn reset(&mut self) {
        self.current_command_index = 0;
        self.turtle.clear();
    }
}

impl TurtleBehaviour for TurtleCommandsAnimator {
    fn update(&mut self, _dt: f32, time_passed_ms: u64) {
        if self.current_command_index >= self.commands.len() {
            if let TurtleCommandsAnimationAutoReset::Enabled { reset_timeout_ms } = self.auto_reset
            {
                let start_timestamp = self.start_reset_timestamp.get_or_insert(Instant::now());
                let now = Instant::now();
                if now.duration_since(*start_timestamp) >= Duration::from_millis(reset_timeout_ms) {
                    self.start_reset_timestamp = None;
                    self.reset();
                }
            }
            return;
        }

        if !self.interval_checker.should_run(time_passed_ms) {
            return;
        }

        while self.current_command_index < self.commands.len() {
            let command = self.commands[self.current_command_index];
            self.turtle.exec_command(command);
            self.current_command_index += 1;
            if matches!(command, TurtleCommand::FORWARD { .. }) {
                break;
            }
        }
    }
}

struct Spiral {
    interval_checker: IntervalChecker,
    turtle: Turtle,
}

impl Spiral {
    fn new(screen: Arc<TurtleScreen>, interval_ms: u64) -> Self {
        let turtle_config = TurtleConfig {
            ..Default::default()
        };
        let turtle = Turtle::new(Vec2::new(0.0, 0.0), screen, &turtle_config);

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

struct SpiralMotion1 {
    interval_checker: IntervalChecker,
    turtles: Vec<Turtle>,
    screen: Arc<TurtleScreen>,
}

impl SpiralMotion1 {
    fn new(screen: Arc<TurtleScreen>, interval_ms: u64) -> Self {
        let turtle_config = TurtleConfig {
            pen_width: 10.0,
            ..Default::default()
        };
        let turtles = (0..50)
            .map(|i| {
                let mut turtle = Turtle::new(
                    Vec2::new(5.0 * i as f32 + 10.0, 120.0),
                    screen.clone(),
                    &turtle_config,
                );
                turtle.set_angle(i as f32 * 15.0);
                turtle
            })
            .collect();

        Self {
            turtles,
            screen: screen.clone(),
            interval_checker: IntervalChecker::new(interval_ms),
        }
    }
}

impl TurtleBehaviour for SpiralMotion1 {
    fn update(&mut self, _dt: f32, time_passed_ms: u64) {
        if !self.interval_checker.should_run(time_passed_ms) {
            return;
        }

        self.screen.clear();

        self.turtles.iter_mut().for_each(|turtle| {
            turtle.forward(2.0);
            turtle.right(2.0);
        })
    }
}

struct SpiralMotion2 {
    interval_checker: IntervalChecker,
    turtles: Vec<Turtle>,
    screen: Arc<TurtleScreen>,
}

impl SpiralMotion2 {
    fn new(screen: Arc<TurtleScreen>, interval_ms: u64) -> Self {
        let turtle_config = TurtleConfig {
            pen_width: 3.0,
            pen_color: YELLOW,
            ..Default::default()
        };
        let turtles = (0..50)
            .map(|i| {
                let mut turtle = Turtle::new(
                    Vec2::new(5.0 * i as f32 + 10.0, 120.0),
                    screen.clone(),
                    &turtle_config,
                );
                turtle.set_angle(i as f32 * 15.0);
                turtle
            })
            .collect();

        Self {
            turtles,
            screen: screen.clone(),
            interval_checker: IntervalChecker::new(interval_ms),
        }
    }
}

impl TurtleBehaviour for SpiralMotion2 {
    fn update(&mut self, _dt: f32, time_passed_ms: u64) {
        if !self.interval_checker.should_run(time_passed_ms) {
            return;
        }

        self.screen.clear();

        let n = self.turtles.len() as f32;
        self.turtles.iter_mut().enumerate().for_each(|(i, turtle)| {
            turtle.forward(100.0);
            turtle.backward(98.0);
            turtle.right(1.0 + i as f32 / n);
        })
    }
}

struct ExpandingCircleMotion {
    interval_checker: IntervalChecker,
    turtles: Vec<Turtle>,
    screen: Arc<TurtleScreen>,
}

impl ExpandingCircleMotion {
    fn new(screen: Arc<TurtleScreen>, interval_ms: u64, angle_multiplier: f32) -> Self {
        let turtle_config = TurtleConfig {
            pen_width: 15.0,
            pen_color: BLUE,
            ..Default::default()
        };
        const N: u32 = 60;
        let turtles = (0..N)
            .map(|i| {
                let mut turtle = Turtle::new(Vec2::new(0.0, 0.0), screen.clone(), &turtle_config);
                turtle.pen_down = false;
                turtle.set_angle(360.0 / N as f32 * i as f32);
                turtle.forward(100.0);
                turtle.pen_down = true;
                turtle.set_angle(turtle.get_angle() * angle_multiplier);
                turtle
            })
            .collect();

        Self {
            turtles,
            screen: screen.clone(),
            interval_checker: IntervalChecker::new(interval_ms),
        }
    }
}

impl TurtleBehaviour for ExpandingCircleMotion {
    fn update(&mut self, _dt: f32, time_passed_ms: u64) {
        if !self.interval_checker.should_run(time_passed_ms) {
            return;
        }

        self.screen.clear();

        self.turtles.iter_mut().for_each(|turtle| {
            turtle.forward(4.0);
            turtle.right(2.0);
        })
    }
}

struct ExpandingSpiral {
    interval_checker: IntervalChecker,
    turtles: Vec<Turtle>,
    screen: Arc<TurtleScreen>,
}

impl ExpandingSpiral {
    fn new(screen: Arc<TurtleScreen>, interval_ms: u64, angle_mutliplier: f32) -> Self {
        let turtle_config = TurtleConfig {
            pen_width: 1.0,
            pen_color: BLUE,
            ..Default::default()
        };
        const N: u32 = 360;
        let turtles = (0..N)
            .map(|i| {
                let mut turtle = Turtle::new(Vec2::new(0.0, 0.0), screen.clone(), &turtle_config);
                turtle.pen_down = false;
                turtle.set_angle(360.0 / N as f32 * i as f32);
                turtle.forward(100.0);
                turtle.pen_down = true;
                turtle.set_angle(turtle.get_angle() * angle_mutliplier);
                turtle
            })
            .collect();

        Self {
            turtles,
            screen: screen.clone(),
            interval_checker: IntervalChecker::new(interval_ms),
        }
    }
}

impl TurtleBehaviour for ExpandingSpiral {
    fn update(&mut self, _dt: f32, time_passed_ms: u64) {
        if !self.interval_checker.should_run(time_passed_ms) {
            return;
        }

        self.screen.clear();

        self.turtles.iter_mut().for_each(|turtle| {
            turtle.forward(204.0);
            turtle.backward(200.0);
            turtle.right(2.0);
        })
    }
}

struct SierpinskiTriangle {
    animator: TurtleCommandsAnimator,
}

impl SierpinskiTriangle {
    fn new(screen: Arc<TurtleScreen>, n: u32, a: f32, interval_ms: u64) -> Self {
        let turtle_config = TurtleConfig {
            pen_width: 1.0,
            ..Default::default()
        };
        let turtle = Turtle::new(Vec2::new(0.0, 0.0), screen.clone(), &turtle_config);
        let triangle_commands = Self::triangle(n, a);

        Self {
            animator: TurtleCommandsAnimator::new(
                turtle,
                &triangle_commands,
                interval_ms,
                TurtleCommandsAnimationAutoReset::Enabled {
                    reset_timeout_ms: 3000,
                },
            ),
        }
    }

    fn triangle(n: u32, a: f32) -> Vec<TurtleCommand> {
        let mut result = vec![];
        Self::triangle_recursive(n, a, &mut result);

        result
    }

    fn triangle_recursive(n: u32, a: f32, result: &mut Vec<TurtleCommand>) {
        if n == 0 {
            return;
        }

        for _ in 0..3 {
            result.push(TurtleCommand::FORWARD { distance: a });
            result.push(TurtleCommand::LEFT { angle: 120.0 });
            Self::triangle_recursive(n - 1, a / 2.0, result);
        }
    }
}

impl TurtleBehaviour for SierpinskiTriangle {
    fn update(&mut self, dt: f32, time_passed_ms: u64) {
        self.animator.update(dt, time_passed_ms);
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
    request_new_screen_size(1600.0, 900.0);

    let mut transform = CoordinateTransform::new(screen_width(), screen_height(), 1.0);
    let screen = Arc::new(TurtleScreen::new(WHITE));

    let _spiral = Spiral::new(screen.clone(), 5);
    let _spiral_motion_1 = SpiralMotion1::new(screen.clone(), 1);
    let _spiral_motion_2 = SpiralMotion2::new(screen.clone(), 1);
    let _expanding_circle_motion = ExpandingCircleMotion::new(screen.clone(), 1, 1.0);
    let _expanding_spiral = ExpandingSpiral::new(screen.clone(), 3, 50.0);
    let sierpinski_triangle = SierpinskiTriangle::new(screen.clone(), 5, 300.0, 100);

    let mut turtle_behaviours: Vec<Box<dyn TurtleBehaviour>> = vec![Box::new(sierpinski_triangle)];

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
