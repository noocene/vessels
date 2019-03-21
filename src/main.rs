extern crate vitruvia;

use vitruvia::graphics_2d::Graphics;
use vitruvia::graphics_2d::ContextualGraphics;

fn main() {
    let window = vitruvia::graphics_2d::new();
    let root = window.frame();
    window.run(root.clone());
}