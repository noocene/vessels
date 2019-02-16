use vitruvia::{initialize, Graphics2D, TryInto};

pub fn main() {
    let _gfx: Graphics2D = initialize().try_into().unwrap();
    //gfx.run();
}
