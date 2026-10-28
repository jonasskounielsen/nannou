use std::fs::File;
use std::io::Write;

use crate::{ball::Ball, collision::{BallCollision, Collision, WallCollision}, vec2::Vec2};

#[derive(Debug)]
pub struct Record {
    data: Vec<SystemState>,
}

impl Record {
    pub fn as_csv(&self) -> String {
        let mut csv = String::from("time");
        let mut header = String::new();
        for i in 0..self.data[0].balls.len() {
            header += ", ball ";
            header += &i.to_string();
        }
        csv += &header;
        for state in &self.data {
            let mut line = String::from("\n");
            line += &state.time.to_string();
            for ball in &state.balls {
                line += ", ";
                line += &ball.vel().mag().to_string();
            }
            csv += &line;
        }
        csv
    }

    pub fn save_to_file(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        write!(file, "{}", self.as_csv()).unwrap();
    }
}

#[derive(Debug)]
struct SystemState {
    time: f32,
    balls: Vec<Ball>,
}

impl SystemState {
    pub fn total_energy(&self) -> f32 {
        self.balls.iter()
            .fold(0.0, |acc, ball| {
                acc + ball.mass * ball.vel().dot(ball.vel())
            })
    }
}

#[derive(Debug)]
pub struct System {
    pub age:   f32,
    pub size: (f32, f32),
    pub balls: Vec<Ball>,
}

#[derive(Clone, Copy, Debug)]
pub enum Wall {
    Top,
    Bottom,
    Left,
    Right,
}

impl Wall {
    pub fn normal(&self) -> Vec2 {
        match self {
            Wall::Top    => Vec2::DOWN,
            Wall::Bottom => Vec2::UP,
            Wall::Left   => Vec2::RIGHT,
            Wall::Right  => Vec2::LEFT,
        }
    }
}

impl System {
    pub fn new(size: (f32, f32), balls: Vec<Ball>) -> Self {
        Self { size, balls: balls, age: 0.0 }
    }

    pub fn record(
        step_size:       f32,
        num_time_steps:  usize,
        initial_delay:   f32,
        initial_balls:   Vec<Ball>,
        size:           (f32, f32),
    ) -> Record {
        let mut system = System::new(size, initial_balls);
        let mut data = Vec::new();
        system.advance(initial_delay);
        for _ in 0..num_time_steps {
            data.push(system.state());
            system.advance(step_size);
        }
        Record { data }
    }

    fn state(&self) -> SystemState {
        SystemState {
            time: self.age,
            balls: self.balls.clone(),
        }
    }

    pub fn advance(&mut self, time: f32) {
        let mut remaining_time = time;
        
        loop {
            let collisions = self.next_collisions();
            let most_urgent_time = Self::most_urgent_collision(&collisions).time();
            
            if most_urgent_time > remaining_time {
                break;
            }

            //dbg!(self.age, &collisions, most_urgent_time);
            //eprintln!();
            self.move_balls(most_urgent_time);
            remaining_time -= most_urgent_time;

            let most_urgent_collisions = collisions
                .into_iter()
                .filter(|collision| {
                    collision.time() == most_urgent_time // Several collisions may occur simultaneously.
                });
            
            most_urgent_collisions
                .for_each(|collision| {
                    collision.handle();
                });

            self.age += most_urgent_time;
        }

        self.move_balls(remaining_time);
        self.age += remaining_time;
    }

    fn move_balls(&self, time: f32) {
        self.balls
            .iter()
            .for_each(|ball| {
                ball.move_by(time);
            });
    }

    fn most_urgent_collision<'a>(collisions: &[Collision<'a>]) -> Collision<'a> {
        collisions
            .iter()
            .copied()
            .reduce(
                |collision1, collision2| {
                    collision1.soonest(collision2)
                }
            )
            .expect("no next collision") // There is always a next collision.
    }

    fn next_collisions(&self) -> Vec<Collision<'_>> {
        self.balls
            .iter()
            .enumerate()
            .flat_map(|(index, ball)| {
                let wall_collisions = self.wall_collisions(ball);
                
                let ball_collisions = self.ball_collisions(ball, index);

                [wall_collisions, ball_collisions].concat()
            })
            .collect()
    }
    
    fn ball_collisions<'a>(&'a self, ball: &'a Ball, from_index: usize) -> Vec<Collision<'a>> {
        self.balls
            .iter()
            .enumerate()
            .filter_map(|(i, other_ball)| {
                if i <= from_index { // Also skip the ball at i == from_index since that is the one we are comparing against.
                    return None;
                }
                self.ball_collision(ball, other_ball)
            })
            .collect()
    }
    
    fn wall_collisions<'a>(&self, ball: &'a Ball) -> Vec<Collision<'a>> {
        let collisions = [
            self.get_wall_collision(ball, Wall::Top),
            self.get_wall_collision(ball, Wall::Bottom),
            self.get_wall_collision(ball, Wall::Left),
            self.get_wall_collision(ball, Wall::Right),
        ];

        collisions
            .iter()
            .filter_map(|collision| *collision)
            .collect()
    }

    // We know that the larger of the values we compare ball position to will always be the wall position.
    // We can therefore use an epsilon relative to that.
    const WALL_COLLISION_EPSILON: f32 = 3.0 * 200.0 * f32::EPSILON;

    fn get_wall_collision<'a>(&self, ball: &'a Ball, wall: Wall) -> Option<Collision<'a>> {
        let distance = match wall {
            Wall::Top    => (ball.pos().y -  self.size.1 / 2.0).abs(),
            Wall::Bottom => (ball.pos().y - -self.size.1 / 2.0).abs(),
            Wall::Left   => (ball.pos().x - -self.size.0 / 2.0).abs(),
            Wall::Right  => (ball.pos().x -  self.size.0 / 2.0).abs(),
        } - ball.rad;
        let speed_towards_edge = ball.vel().comp(wall.normal().inv());
        let time_to_collision = distance / speed_towards_edge;
        
        // If a wall is extremely close, the collision probably has already been handled.
        // If a collision is to happen in negative time, discard it.
        if !time_to_collision.is_finite() ||
            distance <= Self::WALL_COLLISION_EPSILON ||
            time_to_collision < 0.0 {
            return None;
        }

        Some(Collision::WallCollision(WallCollision {
            ball,
            time: time_to_collision,
            wall,
        }))
    }

    const BALL_COLLISION_EPSILON: f32 = 0.0;

    fn ball_collision<'a>(&self, ball1: &'a Ball, ball2: &'a Ball) -> Option<Collision<'a>> {
        // https://www.geogebra.org/classic/dcwgqnr7
        let total_radius       = ball1.rad + ball2.rad;
        let relative_position = ball1.pos() - ball2.pos();
        let relative_velocity = ball1.vel() - ball2.vel();
        
        let discriminant =
            relative_position.dot(relative_velocity).powi(2) -
            relative_velocity.dot(relative_velocity) *
                (relative_position.dot(relative_position) - total_radius.powi(2));

        if discriminant < 0.0 {
            return None;
        }

        let intersection_point =
            relative_position + relative_velocity *
                (-relative_position.dot(relative_velocity) - discriminant.sqrt()) /
                relative_velocity.dot(relative_velocity);
        
        let collision_plane_normal = intersection_point.norm();

        let distance = intersection_point.dist(relative_position);
        let time_to_collision = distance / relative_velocity.mag();
        
        // If a ball is extremely close, the collision probably has already been handled.
        // If a collision is to happen in negative time, discard it.
        if !time_to_collision.is_finite() ||
            distance <= Self::BALL_COLLISION_EPSILON ||
            time_to_collision < 0.0 {
            return None;
        }

        Some(Collision::BallCollision(BallCollision {
            ball1,
            ball2,
            time: time_to_collision,
            plane_normal: collision_plane_normal,
        }))
    }
}
