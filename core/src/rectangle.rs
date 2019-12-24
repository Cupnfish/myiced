use crate::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rectangle<T = f32> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl Rectangle<f32> {
    pub fn contains(&self, point: Point) -> bool {
        self.x <= point.x
            && point.x <= self.x + self.width
            && self.y <= point.y
            && point.y <= self.y + self.height
    }
}

impl std::ops::Mul<f32> for Rectangle<u32> {
    type Output = Self;

    fn mul(self, scale: f32) -> Self {
        Self {
            x: (self.x as f32 * scale).round() as u32,
            y: (self.y as f32 * scale).round() as u32,
            width: (self.width as f32 * scale).round() as u32,
            height: (self.height as f32 * scale).round() as u32,
        }
    }
}
