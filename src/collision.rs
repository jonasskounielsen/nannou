use crate::{ball::Ball, system::Wall, vec2::Vec2};

#[derive(Clone, Copy, Debug)]
pub enum Collision<'a> {
    BallCollision(BallCollision<'a>),
    WallCollision(WallCollision<'a>),
}

#[derive(Clone, Copy, Debug)]
pub struct BallCollision<'a> {
    pub ball1:    &'a Ball,
    pub ball2:    &'a Ball,
    pub time:         f32,
    pub plane_normal: Vec2,
}

impl<'a> BallCollision<'a> {
    pub fn handle(self) {
        let parallel_component_1 = self.ball1.vel().proj(self.plane_normal);
        let parallel_component_2 = self.ball2.vel().proj(self.plane_normal);
        let new_parallel_component_1 = Self::new_speed(
            self.ball1.mass, self.ball2.mass,
            parallel_component_1, parallel_component_2,
        );
        let new_parallel_component_2 = Self::new_speed(
            self.ball2.mass, self.ball1.mass,
            parallel_component_2, parallel_component_1,
        );
        self.ball1.set_vel(self.ball1.vel() - parallel_component_1 + new_parallel_component_1);
        self.ball2.set_vel(self.ball2.vel() - parallel_component_2 + new_parallel_component_2);
    }

    fn new_speed(mass: f32, other_mass: f32, vel_comp: Vec2, other_vel_comp: Vec2) -> Vec2 {
        (mass * vel_comp + 2.0 * other_mass * other_vel_comp - other_mass * vel_comp) /
            (mass + other_mass)
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
        let parallel_component = self.ball.vel().proj(self.wall.normal());
        self.ball.set_vel(self.ball.vel() - 2.0 * parallel_component);
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
