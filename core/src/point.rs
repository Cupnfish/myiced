use crate::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,

    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<[f32; 2]> for Point {
    fn from([x, y]: [f32; 2]) -> Self {
        Self { x, y }
    }
}

impl From<[u16; 2]> for Point {
    fn from([x, y]: [u16; 2]) -> Self {
        Point::new(x.into(), y.into())
    }
}
impl std::ops::Add<Vector> for Point {
    type Output = Self;

    fn add(self, vector: Vector) -> Self {
        Self {
            x: self.x + vector.x,
            y: self.y + vector.y,
        }
    }
}
