use crate::graphics2_d::Vector;

pub trait Mouse: super::Source<Event> {
    fn position(&self) -> Vector;
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
    Move(Vector),
}

pub struct Event {
    pub action: Action,
    pub position: Vector,
}

impl super::Event for Event {}
