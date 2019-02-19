use vitruvia::graphics::{initialize, Graphics2D, TryInto};

pub fn main() {
    let _gfx: Box<Graphics2D> = initialize().try_into().unwrap();
    //gfx.run();
}
