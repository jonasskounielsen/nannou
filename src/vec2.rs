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

    pub fn dist(&self, other: Vec2) -> f32 {
        (*self - other).mag()
    }

    pub fn dot(&self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn component(&self, along: Vec2) -> f32 {
        self.dot(along) / along.mag()
    }

    pub fn reflect(&self, about: Vec2) -> Vec2 {
        *self - about *(self.dot(about)) * 2.0
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