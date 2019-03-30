extern crate vitruvia;

use vitruvia::graphics_2d::Graphics;
use vitruvia::graphics_2d::ContextualGraphics;

fn main() {
    let window = vitruvia::graphics_2d::new();
    let root = window.frame();
    let context = window.start(root.clone());
    context.run(Box::new(move |context| {
        println!("We in this");
    }));
}