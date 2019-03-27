use crate::graphics_2d::ContextualGraphics;

mod canvas;

pub(crate) fn new() -> Box<dyn ContextualGraphics> {
    canvas::new()
}
