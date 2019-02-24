extern crate no_panic;

use no_panic::no_panic;

use vitruvia::graphics::{initialize, Graphics2D};
use vitruvia::util::TryInto;

#[no_panic]
pub fn main() {
    let gfx: &Graphics2D = initialize().try_into().unwrap();
    let root = gfx.frame();
    gfx.run(root);
}
