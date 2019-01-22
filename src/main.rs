use vitruvia::{initialize, Frame, Rect, Renderer};

#[macro_use]
extern crate stdweb;

use std::cell::RefCell;
use std::rc::Rc;

struct Cb {
    frames: Vec<Box<dyn Frame>>,
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
    let p = Rc::new(RefCell::new(Cb { frames: vec![r] }));
    stdweb::web::window().request_animation_frame(move |_time| {
        let _p = p.clone();
        p.borrow().p(_p);
    });
    gfx.run();
}
