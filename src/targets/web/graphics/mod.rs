use crate::graphics_2d::*;

mod canvas;

pub fn new() -> impl ContextualGraphics {
    canvas::new()
}
