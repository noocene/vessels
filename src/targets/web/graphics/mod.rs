use crate::graphics::canvas::ContextualGraphics;

mod canvas;

pub(crate) fn new() -> Box<dyn ContextualGraphics> {
    canvas::new()
}
