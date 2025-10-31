use crate::{ball::Ball, system::Wall};

#[derive(Clone, Copy, Debug)]
pub enum Collision<'a> {
    BallCollision(BallCollision<'a>),
    WallCollision(WallCollision<'a>),
}

#[derive(Clone, Copy, Debug)]
pub struct BallCollision<'a> {
    pub ball1: &'a Ball,
    pub ball2: &'a Ball,
    pub time: f32,
}

impl<'a> BallCollision<'a> {
    pub fn handle(self) {

    }
}

#[derive(Clone, Copy, Debug)]
pub struct WallCollision<'a> {
    pub ball: &'a Ball,
    pub wall: Wall,
    pub time: f32,
}

impl<'a> WallCollision<'a> {
    pub fn handle(self) {
        self.ball.collide(self.wall.normal());
    }
}

impl<'a> Collision<'a> {
    pub fn time(&self) -> f32 {
        match *self {
            Collision::BallCollision(BallCollision { time, .. }) => time,
            Collision::WallCollision(WallCollision { time, .. }) => time,
        }
    }

    pub fn soonest(self, other: Collision<'a>) -> Collision<'a> {
        if self.time() < other.time() {
            self
        } else {
            other
        }
    }
    
    pub fn handle(self) {
        match self {
            Collision::BallCollision(ball_collision) => ball_collision.handle(),
            Collision::WallCollision(wall_collision) => wall_collision.handle(),
        }
    }
}