use crate::graphics_2d::ContextualGraphics;

mod canvas;

pub(crate) fn new() -> impl ContextualGraphics {
    canvas::new()
}
