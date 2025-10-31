mod ball;
mod system;
mod nannou;
mod collision;
mod vec2;

fn main() {
    debug();
    //nannou::run();
}

use std::f32::consts::PI;

use ball::Ball;
use system::System;

fn debug() {
    let circles = vec![
        Ball::new((   0.0,    0.0), ( 0.0,   0.0)),
        Ball::new((  50.0,   50.0), ( 3.5, -10.0)),
        Ball::new((-100.0, -100.0), (20.0,  10.0)),
    ];

    let mut system = System::new((400.1, 400.0 - PI), circles);

    //system.advance(340.0);

    loop {
        system.advance(10.0);

        dbg!(&system);
        println!();

        let ball = &system.balls[0];

        if ball.pos().x.abs() > 190.0 {
            break;
        }
    }
}