use crate::graphics::canvas::ContextualGraphics;

mod cm;
mod pure2d;

pub(crate) fn new() -> Box<dyn ContextualGraphics> {
    pure2d::new()
}
