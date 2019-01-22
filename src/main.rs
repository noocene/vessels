use vitruvia::{initialize, Rect, Renderer};

pub fn main() {
    let gfx = initialize();
    let mut root = gfx.root();
    {
        let _r = root.child(Rect {
            x: 0,
            y: 0,
            w: 100,
            h: 100,
        });
    };
    gfx.run();
}
