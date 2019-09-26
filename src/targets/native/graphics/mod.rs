use crate::graphics::canvas::InteractiveCanvas;

mod cm;
mod pure2d;

pub(crate) fn new() -> Box<dyn InteractiveCanvas> {
    pure2d::new()
}
