use crate::graphics::*;

mod cm;
mod pure2d;

pub(crate) fn new() -> Box<dyn ContextualGraphics> {
    pure2d::new()
}
