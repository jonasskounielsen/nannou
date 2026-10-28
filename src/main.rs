mod ball;
mod system;
mod nannou;
mod collision;
mod vec2;

fn main() {
    // debug();
    // nannou::run();
    collect();
}

use std::f32::consts::PI;

use ball::Ball;
use system::System;

#[allow(dead_code)]
fn collect() {
    let mut balls = Vec::new();
    for i in 1..=15 {
        let ball = Ball::new((i as f32 * 10.0 - 200.0, 10.0), (i as f32 * 10.0, i as f32 * 5.0), 8.0 + (i as f32) / 10.0 * 8.0, 1.0 + (i as f32) / 10.0);
        balls.push(ball);
    }

    let record = System::record(
        30.0,
        1000,
        30.0,
        balls,
        (500.0, 500.0),
    );

    record.save_to_file("../data.csv");
}

#[allow(dead_code)]
fn debug() {
    let mut balls = Vec::new();
    for i in 1..=2 {
        let ball = Ball::new((i as f32 * 10.0, 10.0), (i as f32 * 10.0, 0.0), 8.0, 1.0);
        balls.push(ball);
    }

    let mut system = System::new((400.1, 400.0 - PI), balls);

    system.advance(0.0);

    for i in 0..1000000 {
        system.advance(1.0);

        dbg!(&system, i);
    }
}
