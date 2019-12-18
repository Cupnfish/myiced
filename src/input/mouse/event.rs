use super::Button;
use crate::input::ButtonState;

#[derive(Debug, Clone,Copy,PartialEq)]
pub enum Event {
    /// 光标进入
    CursorEntered,
    /// 光标离开
    CursorLeft,
    /// 光标移动
    CursorMoved {
        x:f32,
        y:f32,
    },
    /// 鼠标按钮被按或被松开
    Input {
        state: ButtonState,
        button: Button,
    }, 
    /// 鼠标在滚动轴上
    wheelScrolled {
        delta_x:f32,
        delta_y:f32,
    },
}