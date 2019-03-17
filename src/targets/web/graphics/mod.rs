use crate::graphics_2d::*;

mod canvas;

pub(crate) fn new() -> impl ContextualGraphics {
    canvas::new()
}
