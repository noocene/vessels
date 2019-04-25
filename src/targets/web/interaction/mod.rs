mod keyboard;
mod mouse;
mod windowing;

pub(crate) use crate::targets::web::interaction::keyboard::Keyboard;
pub(crate) use crate::targets::web::interaction::mouse::Mouse;
pub(crate) use crate::targets::web::interaction::windowing::Window;
