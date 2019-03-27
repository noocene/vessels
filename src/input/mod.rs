/// Types to permit handling of keyboard input.
pub mod keyboard;
pub use crate::input::keyboard::Keyboard;
/// Types to permit handling of mouse input.
pub mod mouse;
pub use crate::input::mouse::Mouse;

/// A source of input events.
pub trait Source {
    /// The associated event type.
    type Event: Event;
    /// Binds the provided handler to be called when an event occurs.
    fn bind(&self, handler: Box<dyn Fn(Self::Event) + 'static>);
}

/// A class of events.
pub trait Event {}

/// A context that provides input handling functionality.
pub trait Context {
    /// Returns mouse input bindings.
    fn mouse(&self) -> Box<dyn Mouse>;
    /// Returns keyboard input bindings.
    fn keyboard(&self) -> Box<dyn Keyboard>;
}
