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

    let circles = vec![
        Ball::new((   0.0,    0.0), ( 0.0,   0.0)),
        Ball::new((  50.0,   50.0), ( 3.5, -10.0)),
        Ball::new((-100.0, -100.0), (20.0,  10.0)),
    ];

    let system = System::new((400.1, 400.0 - PI), circles);
    
    Model {
        system,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.system.advance(1000.0);
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