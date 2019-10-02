use crate::graphics::canvas::InteractiveCanvas;

mod canvas;

pub(crate) fn new() -> Box<dyn InteractiveCanvas> {
    canvas::new()
}
