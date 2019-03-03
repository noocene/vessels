use crate::graphics::*;

pub mod canvas;

pub fn new() -> impl Graphics2D {
    canvas::new()
}
