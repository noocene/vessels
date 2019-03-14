use crate::graphics::*;

mod canvas;

pub fn new() -> impl ContextualGraphics2D {
    canvas::new()
}
