use vitruvia::graphics::{initialize, Graphics2D};
use vitruvia::util::TryInto;

pub fn main() {
    let _gfx: Box<Graphics2D> = initialize().try_into().unwrap();
    //gfx.run();
}
