mod ball;
mod system;
mod nannou;
mod collision;
mod vec2;

fn main() {
    // debug();
    nannou::run();
}

use std::f32::consts::PI;

use ball::Ball;
use system::System;

#[allow(dead_code)]
fn debug() {
    let mut circles = Vec::new();
    for i in 1..=2 {
        let ball = Ball::new((i as f32 * 10.0, 10.0), (i as f32 * 10.0, 0.0), 8.0, 1.0);
        circles.push(ball);
    }

    let mut system = System::new((400.1, 400.0 - PI), circles);

    system.advance(0.0);

    for i in 0..1000000 {
        system.advance(1.0);

        dbg!(&system, i);
    }
}
