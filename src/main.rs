use vitruvia::graphics;
use vitruvia::graphics::AbstractGraphics2D;
use vitruvia::util::TryInto;

pub fn main() {
    let gfx: AbstractGraphics2D = graphics::new().try_into().unwrap();
    let root = gfx.frame();
    gfx.run(root);
}
