/// 鼠标的光标状态
#[derive(Debug, Eq,PartialEq,Clone,Copy)]
pub enum MouseCursor { 
/// 光标离开用户界面的范围
    OutOfBounds,

    /// 光标在非交互组件上时
    Idle,

    /// 光标在一个可以点击的组件上面时
    Pointer,

    /// 光标在忙碌的组件上面时 
    Working,

    /// 光标在可抓取的组件上面时
    Grab,

    /// 光标正在抓取一个组件时
    Grabbing,
}