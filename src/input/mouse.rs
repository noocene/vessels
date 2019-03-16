use crate::graphics_2d::Vector;

pub trait Mouse: super::Source<Event> {
    fn position(&self) -> Vector;
}

#[derive(Clone, Copy)]
pub enum Button {
    Left,
    Right,
    Middle,
    Auxiliary(u8),
}

#[derive(Clone, Copy)]
pub enum Action {
    Up(Button),
    Down(Button),
    Move(Vector),
}

#[derive(Clone, Copy)]
pub struct Event {
    pub action: Action,
    pub position: Vector,
}

impl super::Event for Event {}
