use crate::{ball::Ball, collision::{BallCollision, Collision, WallCollision}, vec2::Vec2};

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
            Wall::Top    => Vec2::UP,
            Wall::Bottom => Vec2::DOWN,
            Wall::Left   => Vec2::LEFT,
            Wall::Right  => Vec2::RIGHT,
        }
    }
}

impl System {
    pub fn new(size: (f32, f32), circles: Vec<Ball>) -> Self {
        Self { size, balls: circles, age: 0.0 }
    }

    pub fn advance(&mut self, time: f32) {
        let mut remaining_time = time;
        
        loop {
            let collisions = self.next_collisions();
            let most_urgent_time = Self::most_urgent_collision(&collisions).time();
            
            if most_urgent_time > remaining_time {
                break;
            }

            dbg!(self.age, &collisions);
            println!();
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
            .unwrap()
    }

    fn next_collisions(&self) -> Vec<Collision<'_>> {
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

    // We know that the larger of the values we compare ball position to will always be the wall position.
    // We can therefore use an epsilon relative to that.
    // This relies on speed_towards_edge not being very small,
    // and making speed_towards_edge much larger than distance.
    const WALL_COLLISION_EPSILON: f32 = 3.0 * 200.0 * f32::EPSILON;

    fn get_wall_collision<'a>(&self, ball: &'a Ball, wall: Wall) -> Option<Collision<'a>> {
        let distance = match wall {
            Wall::Top    => (ball.pos().y -  self.size.1 / 2.0).abs(),
            Wall::Bottom => (ball.pos().y - -self.size.1 / 2.0).abs(),
            Wall::Left   => (ball.pos().x - -self.size.0 / 2.0).abs(),
            Wall::Right  => (ball.pos().x -  self.size.0 / 2.0).abs(),
        } - ball.rad;
        let wall_normal = wall.normal();
        let speed_towards_edge = ball.vel().component(wall_normal);
        let time_to_collision = distance / speed_towards_edge;
        
        //dbg!(distance, wall_normal, speed_towards_edge, time_to_collision);
        
        // If a collision is to happen in zero or close to zero time, it has already been handled.
        // If it is to happen in negative time, discard it.
        if !time_to_collision.is_finite() ||
            time_to_collision <= Self::WALL_COLLISION_EPSILON {
            return None;
        }
        Some(Collision::WallCollision(WallCollision {
            ball,
            time: time_to_collision,
            wall,
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