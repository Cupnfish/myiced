use super::KeyCode;
use crate::input::ButtonState;

#[derive(Debug, Clone,Copy,PartialEq)]
///  键盘事件
pub enum Event {
    Input {
        state: ButtonState,
        key_code: KeyCode,
    },
    TextEntered {
        character: char,
    },
}