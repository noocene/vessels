mod keyboard;
mod mouse;
mod windowing;

pub(crate) use crate::targets::native::interaction::keyboard::Keyboard;
pub(crate) use crate::targets::native::interaction::mouse::Mouse;
pub(crate) use crate::targets::native::interaction::windowing::Window;
