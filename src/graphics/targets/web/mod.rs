use crate::graphics::*;

mod canvas;

pub fn new() -> impl Graphics2D {
    canvas::new()
}
