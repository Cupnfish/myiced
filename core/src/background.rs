use crate::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Background {
    Color(Color),
}

impl From<Color> for Background {
    fn from(color: Color) -> Self {
        Background::Color(color)
    }
}
