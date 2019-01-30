#![feature(try_from)]

use vitruvia::{initialize, Renderer, RendererSupports2D};

pub fn main() {
    let gfx = RendererSupports2D::try_from(initialize()).unwrap();
    gfx.run();
}
