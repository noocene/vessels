use vitruvia::{initialize, Frame, Geometry, Geometry2D, Rect, Renderer};

extern crate stdweb;

use std::cell::RefCell;
use std::rc::Rc;

struct Cb {
    frame: Box<Frame<Geometry2D>>,
}

impl Cb {
    fn p(&self, rc: Rc<RefCell<Cb>>) {
        stdweb::web::window().request_animation_frame(move |_time| {
            let _p = rc.clone();
            rc.borrow().p(_p);
        });
    }
}

pub fn main() {
    let gfx = initialize();
    let mut root = gfx.root();
    let r = root.child(Rect {
        x: 0,
        y: 0,
        w: 100,
        h: 100,
    });
    let p = Rc::new(RefCell::new(Cb { frame: r }));
    stdweb::web::window().request_animation_frame(move |_time| {
        let _p = p.clone();
        p.borrow().p(_p);
    });
    gfx.run();
}
