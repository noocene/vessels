use crate::graphics::Vector2;

/// A mouse button.
#[derive(Clone, Copy, Debug)]
pub enum Button {
    /// The left or primary mouse button.
    Left,
    /// The right or secondary mouse button.
    Right,
    /// The middle or tertiary mouse button. Often bound to scroll wheel depression.
    Middle,
    /// An auxiliary mouse button at the given index. Indices start at zero and represent the offset
    /// from the tertiary button i.e. ```Auxiliary(0)``` is the fourth mouse button.
    Auxiliary(u8),
}

impl From<Button> for u8 {
    fn from(input: Button) -> u8 {
        match input {
            Button::Left => 0,
            Button::Right => 1,
            Button::Middle => 2,
            Button::Auxiliary(index) => index + 3,
        }
    }
}

impl From<u8> for Button {
    fn from(input: u8) -> Button {
        match input {
            0 => Button::Left,
            1 => Button::Right,
            2 => Button::Middle,
            index => Button::Auxiliary(index - 3),
        }
    }
}

/// A mouse event.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// The return of a button from an activated state to an inactivated state.
    Up(Button),
    /// The transition of a button from an inactivated state to an activated state.
    Down(Button),
    /// A mouse movement to the given position.
    Move(Vector2),
    /// A mouse scroll input.
    Scroll(Vector2),
}
