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
    fn bind<F>(&self, handler: F)
    where
        F: Fn(Self::Event) + 'static;
}

/// A class of events.
pub trait Event {}

/// A context that provides input handling functionality.
pub trait Context {
    /// The associated concrete type that provides mouse input.
    type Mouse: Mouse;
    /// The associated concrete type that provides keyboard input.
    type Keyboard: Keyboard;
    /// Returns mouse input bindings.
    fn mouse(&self) -> Self::Mouse;
    /// Returns keyboard input bindings.
    fn keyboard(&self) -> Self::Keyboard;
}
