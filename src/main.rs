use vitruvia::graphics;
use vitruvia::graphics::Graphics2D;

pub fn main() {
    let gfx = graphics::new();
    let root = gfx.frame();
    gfx.run(root);
}
