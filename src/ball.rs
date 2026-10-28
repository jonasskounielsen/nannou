use std::cell::RefCell;

use crate::vec2::Vec2;

#[derive(Debug, Clone)]
pub struct Ball {
    pos: RefCell<Vec2>,
    vel: RefCell<Vec2>,
    pub rad:  f32,
    pub mass: f32,
}

impl Ball {
    pub fn new(pos: (f32, f32), vel: (f32, f32), rad: f32, mass: f32) -> Self {
        Self {
            pos: RefCell::new(Vec2::new(pos)),
            vel: RefCell::new(Vec2::new(vel)),
            rad,
            mass,
        }
    }

    pub fn pos(&self) -> Vec2 {
        *self.pos.borrow()
    }

    pub fn vel(&self) -> Vec2 {
        *self.vel.borrow()
    }

    pub fn set_vel(&self, new_vel: Vec2) {
        *self.vel.borrow_mut() = new_vel;
    }
    
    pub fn move_by(&self, time: f32) {
        let movement = self.vel() * time;
        *self.pos.borrow_mut() += movement;
    }
}
