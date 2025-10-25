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
        //Ball::new((  50.0,   50.0), ( 0.0, -10.0)),
        Ball::new((-100.0, -100.0), (20.0,  10.0)),
    ];

    let system = System::new((400.0, 400.0), circles);
    
    Model {
        system,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.system.advance(1.0);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(CORNFLOWERBLUE);

    model.system.balls.iter().for_each(
        |ball| {
            draw.ellipse()
                .color(STEELBLUE)
                .w_h(10.0, 10.0)
                .x_y(ball.pos().x, ball.pos().y);
        }
    );

    draw.to_frame(app, &frame).unwrap();
}