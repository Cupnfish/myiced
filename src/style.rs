use std::hash::{Hash, Hasher};
use stretch::{geometry,style};

/// ['节点']的外观
/// 
/// ['节点']:struct.Node.html
#[derive(Debug, Clone,Copy)]
pub struct Style(pub(crate) style::Style);

impl Style {
    ///定义像素['节点']的宽度
    /// 
    /// ["节点"]:struct.Node.html
    pub fn width(mut self,width:u32) -> Self {
        self.0.size.width = style::Dimension::Points(width as f32);
        self
    }
    /// TODO:
}