/// Types to permit handling of keyboard interaction.
pub mod keyboard;
pub use crate::interaction::keyboard::Keyboard;
/// Types to permit handling of mouse interaction.
pub mod mouse;
pub use crate::interaction::mouse::Mouse;
/// Types to permit handling of windowing.
pub mod windowing;
pub use crate::interaction::windowing::Window;

/// A source of interaction events.
pub trait Source {
    /// The associated event type.
    type Event: Event;
    /// Binds the provided handler to be called when an event occurs.
    fn bind(&self, handler: Box<dyn Fn(Self::Event) + 'static + Send + Sync>);
    #[doc(hidden)]
    fn box_clone(&self) -> Box<dyn Source<Event = Self::Event>>;
}

impl<T> Clone for Box<dyn Source<Event = T>> where T: Event
{
    fn clone(&self) -> Box<dyn Source<Event = T>> {
        self.box_clone()
    }
}

/// A class of events.
pub trait Event {}

/// A context that provides interaction handling functionality.
pub trait Context {
    /// Returns windowing bindings.
    fn window(&self) -> Box<dyn Window>;
    /// Returns mouse interaction bindings.
    fn mouse(&self) -> Box<dyn Mouse>;
    /// Returns keyboard interaction bindings.
    fn keyboard(&self) -> Box<dyn Keyboard>;
}
