use futures::Stream;

/// Types to permit handling of keyboard input.
pub mod keyboard;
/// Types to permit handling of mouse input.
pub mod mouse;
/// Types to permit handling of windowing.
pub mod windowing;

/// Encapsulates a variety of possible input events.
#[derive(Clone, Debug, Copy)]
pub enum Event {
    /// A keyboard event.
    Keyboard(keyboard::Event),
    /// A mouse event.
    Mouse(mouse::Event),
    /// A windowing event.
    Windowing(windowing::Event),
}

/// A context that provides input handling functionality.
pub trait Provider {
    /// Returns an input event stream for this context.
    fn input(&self) -> Box<dyn Input>;
}

/// An input event stream.
pub trait Input: Stream<Item = Event, Error = ()> {
    #[doc(hidden)]
    fn box_clone(&self) -> Box<dyn Input>;
}

impl Clone for Box<dyn Input> {
    fn clone(&self) -> Box<dyn Input> {
        self.box_clone()
    }
}
