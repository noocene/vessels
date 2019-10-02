use crate::graphics::Vector2;

/// A windowing event.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// A window resize event.
    Resize,
    /// A window move event.
    Move(Vector2),
    /// A redraw event. Provides the delta since the last redraw.
    Redraw(f64),
}
