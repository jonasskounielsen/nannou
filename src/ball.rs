use std::cell::RefCell;

use crate::vec2::Vec2;

#[derive(Debug)]
pub struct Ball {
    pos: RefCell<Vec2>,
    vel: RefCell<Vec2>,
    pub rad: f32,
}

impl Ball {
    pub fn new(pos: (f32, f32), vel: (f32, f32)) -> Self {
        Self {
            pos: RefCell::new(Vec2::new(pos)),
            vel: RefCell::new(Vec2::new(vel)),
            rad: 20.0,
        }
    }

    pub fn pos(&self) -> Vec2 {
        *self.pos.borrow()
    }

    pub fn vel(&self) -> Vec2 {
        *self.vel.borrow()
    }
    
    pub fn move_by(&self, time: f32) {
        let movement = self.vel() * time;
        *self.pos.borrow_mut() += movement;
    }

    pub fn collide(&self, plane_normal: Vec2) {
        *self.vel.borrow_mut() = self.vel().reflect(plane_normal);
    }

    pub fn swap_speed(&self, other: &Ball, collision_normal: Vec2) {
        let new_vel_self  = self .vel().scale_or(other.vel().mag(), collision_normal.inv());
        let new_vel_other = other.vel().scale_or(self .vel().mag(), collision_normal);
        *self .vel.borrow_mut() = new_vel_self;
        *other.vel.borrow_mut() = new_vel_other;
    }
}