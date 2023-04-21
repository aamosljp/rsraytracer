use crate::vec3::{ Vec3, Point };

pub struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    pub fn origin(&self) -> Point {
        self.origin.clone()
    }
    pub fn direction(&self) -> Vec3 {
        self.direction.clone()
    }
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}
