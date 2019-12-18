use crate::Point;

/// 一个普通的矩形
#[derive(Debug, PartialEq,Eq, Clone, Copy)]
pub struct Rectangle<T> {
    /// 左上角坐标的X值
    pub x: T,

    /// 左上角坐标的Y值
    pub y: T,

    /// 矩形的宽带
    pub width: T,

    /// 矩形的高度
    pub height: T,
}

impl Rectangle<f32> {
    /// 给定['点']如果在['矩形']内，返回true
    /// 
    /// ['点']：type.point.html
    /// ['矩形']:struct.Rectangle.html
    pub fn contains(&self, point:Point) -> bool {
        self.x <= point.x 
        && point.x <= self.x + self.width
        && self.y <= point.y
        && point.y <= self.y + self.height
    }
}