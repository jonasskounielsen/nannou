use crate::{ball::Ball, collision::{BallCollision, Collision, WallCollision}, vec2::Vec2};

pub struct System {
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
            Wall::Top    => Vec2::UP,
            Wall::Bottom => Vec2::DOWN,
            Wall::Left   => Vec2::LEFT,
            Wall::Right  => Vec2::RIGHT,
        }
    }
}

impl System {
    pub fn new(size: (f32, f32), circles: Vec<Ball>) -> Self {
        Self { size, balls: circles, }
    }

    pub fn advance(&mut self, time: f32) {
        let mut remaining_time = time;
        
        loop {
            let collisions = self.next_collisions();
            let most_urgent_time = Self::most_urgent_collision(&collisions).time();
            
            if most_urgent_time > remaining_time {
                break;
            }

            dbg!(&collisions);
            self.move_balls(most_urgent_time);
            remaining_time -= most_urgent_time;

            let most_urgent_collisions = collisions
                .into_iter()
                .filter(|collision| {
                    collision.time() == most_urgent_time // Several collisions may occur simultaniously.
                });
            
            most_urgent_collisions
                .for_each(|collision| {
                    collision.handle();
                });
        }

        self.move_balls(remaining_time);
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
            .map(|collision| *collision)
            .reduce(
                |collision1, collision2| {
                    collision1.soonest(collision2)
                }
            )
            .unwrap()
    }

    fn next_collisions(&self) -> Vec<Collision> {
        self.balls
            .iter()
            .map(|ball| {
                self.next_collision(ball)
            })
            .collect()
    }

    fn next_collision<'a>(&'a self, ball: &'a Ball) -> Collision<'a> {
        let ball_collisions = self.ball_collisions(ball);
        
        let wall_collisions = self.wall_collisions(ball);

        ball_collisions
            .into_iter()
            .chain(wall_collisions)
            .reduce(|collision1, collision2| {
                collision1.soonest(collision2)
            })
            .unwrap() // There is always a next collision.
    }
    
    fn ball_collisions<'a>(&'a self, ball: &'a Ball) -> Vec<Collision<'a>> {
        self.balls
            .iter()
            .map(|other_ball| {
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

    fn get_wall_collision<'a>(&self, ball: &'a Ball, wall: Wall) -> Option<Collision<'a>> {
        let distance = match wall {
            Wall::Top    => ball.pos().y + self.size.0 / 2.0,
            Wall::Bottom => ball.pos().y - self.size.0 / 2.0,
            Wall::Left   => ball.pos().x - self.size.1 / 2.0,
            Wall::Right  => ball.pos().x + self.size.1 / 2.0,
        } - ball.rad;
        let wall_normal = wall.normal();
        let speed_towards_edge = ball.vel().component(wall_normal);
        let time_to_collision = distance / speed_towards_edge;
        dbg!(ball.pos().x);
        dbg!(distance, wall_normal, speed_towards_edge, time_to_collision);
        println!();
        // If a collision is to happen in zero time, it has already been handled.
        if !time_to_collision.is_finite() || time_to_collision < f32::EPSILON {
            return None;
        }
        Some(Collision::WallCollision(WallCollision {
            ball,
            time: time_to_collision,
            wall: Wall::Left,
        }))
    }

    fn ball_collision<'a>(&self, ball1: &'a Ball, ball2: &'a Ball) -> Collision<'a> {
        Collision::BallCollision(BallCollision {
            ball1,
            ball2,
            time: 1000.0,
        })
    }
}