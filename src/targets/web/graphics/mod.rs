use crate::graphics::ContextualGraphics;

mod canvas;

pub(crate) fn new() -> Box<dyn ContextualGraphics> {
    canvas::new()
}
