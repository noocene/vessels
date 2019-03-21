use crate::graphics_2d::*;

mod pure2d;

pub(crate) fn new() { // -> impl ContextualGraphics {
    pure2d::new()
}