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
}