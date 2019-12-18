use crate::input::{keyboard,mouse};

/// 用于用户接口的event
#[derive(PartialEq,Clone,Copy,Debug)]
pub enum Event {
    Keyboard(keyboard::Event),
    Mouse(mouse::Event),
}