use nannou::prelude::*;

use crate::{ball::Ball, system::System};

struct Model {
    system: System,
}

pub fn run() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(400, 400)
        .view(view)
        .build()
        .unwrap();

    let mut circles = Vec::new();
    for i in 1..=15 {
        let ball = Ball::new((i as f32 * 10.0 - 200.0, 10.0), (i as f32 * 10.0, i as f32 * 5.0), 8.0 + (i as f32) / 10.0 * 8.0, 1.0 + (i as f32) / 10.0);
        circles.push(ball);
    }
    
    let system = System::new((400.0, 400.0), circles);

    Model {
        system,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.system.advance(0.01);
    dbg!(model.system.total_energy());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(CORNFLOWERBLUE);

    draw.rect()
        .color(REBECCAPURPLE)
        .w_h(model.system.size.0, model.system.size.1)
        .x_y(0.0, 0.0);

    model.system.balls.iter().for_each(
        |ball| {
            draw.ellipse()
                .color(STEELBLUE)
                .w_h(ball.rad * 2.0, ball.rad * 2.0)
                .x_y(ball.pos().x, ball.pos().y);
        }
    );

    draw.to_frame(app, &frame).unwrap();
}
