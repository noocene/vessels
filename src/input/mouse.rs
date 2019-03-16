use crate::graphics2_d::Vec2D;

pub trait Mouse: super::Source<Event> {
    fn position(&self) -> Vec2D;
}

pub enum Button {
    Left,
    Right,
    Middle,
    Auxiliary(u8),
}

pub enum Action {
    Up(Button),
    Down(Button),
    Move(Vec2D),
}

pub struct Event {
    pub action: Action,
    pub position: Vec2D,
}

impl super::Event for Event {}
