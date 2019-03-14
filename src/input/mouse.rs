use crate::graphics::Vec2D;

pub enum Button {
    Left,
    Right,
    Middle,
    Auxiliary(u8),
}

pub enum Event {
    Up(Button),
    Down(Button),
    Move(Vec2D),
}

impl super::Event for Event {}
