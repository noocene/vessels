use crate::graphics2_d::*;

mod canvas;

pub fn new() -> impl ContextualGraphics2D {
    canvas::new()
}
