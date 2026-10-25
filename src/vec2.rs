#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const UP:    Self = Vec2 { x:  0.0, y:  1.0};
    pub const DOWN:  Self = Vec2 { x:  0.0, y: -1.0};
    pub const LEFT:  Self = Vec2 { x: -1.0, y:  0.0};
    pub const RIGHT: Self = Vec2 { x:  1.0, y:  0.0};
    
    pub fn new((x, y): (f32, f32)) -> Self {
        Self { x, y, }
    }

    pub fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn scale(&self, new_mag: f32) -> Vec2 {
        if self.mag() == 0.0 {
            panic!("vector has no direction");
        }
        *self * (new_mag / self.mag())
    }

    pub fn scale_or(&self, new_mag: f32, fallback_direction: Vec2) -> Vec2 {
        if self.mag() == 0.0 {
            fallback_direction
        } else {
            *self
        }.scale(new_mag)
    }

    pub fn norm(&self) -> Vec2 {
        self.scale(1.0)
    }

    pub fn slope(&self) -> f32 {
        if self.x == 0.0 {
            panic!("slope is infinite");
        }
        self.y / self.x
    }

    pub fn dist(&self, other: Vec2) -> f32 {
        (*self - other).mag()
    }

    pub fn dot(&self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn comp(&self, along: Vec2) -> f32 {
        self.dot(along) / along.mag()
    }

    pub fn proj(&self, along: Vec2) -> Vec2 {
        self.dot(along) / along.dot(along) * along
    }

    pub fn reflect(&self, about: Vec2) -> Vec2 {
        *self - about * (self.dot(about)) * 2.0
    }

    pub fn inv(&self) -> Vec2 {
        *self * -1.0
    }

    pub fn perp(&self) -> Vec2 {
        Vec2 {
            x: -self.y,
            y:  self.x,
        }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;
    
    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Mul<Vec2> for f32 {
    type Output = Vec2;
    
    fn mul(self, rhs: Vec2) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;
    
    fn div(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    
    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
