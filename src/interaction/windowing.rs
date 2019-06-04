use crate::graphics_2d::Vector;

/// An interaction event source that represents a window.
pub trait Window: super::Source<Event = Event> {
    /// Sets the window title.
    fn set_title(&mut self, title: &'_ str);
}

/// A window interaction.
#[derive(Clone, Copy, Debug)]
pub enum Action {
    /// A window resize event.
    Resize,
    /// A window move event.
    Move(Vector),
}

/// A window event.
#[derive(Clone, Copy, Debug)]
pub struct Event {
    /// The associated action.
    pub action: Action,
}

impl super::Event for Event {}
