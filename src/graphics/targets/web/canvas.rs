use crate::graphics::*;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, window, CanvasRenderingContext2d};

use stdweb::web::event::ResizeEvent;

use stdweb::web::html_element::CanvasElement;

use std::cell::RefCell;
use std::rc::Rc;

pub struct CanvasFrame {
    context: CanvasRenderingContext2d,
    canvas: CanvasElement,
}

impl Object2D for CanvasFrame {}

impl Frame2D for CanvasFrame {
    fn add(&self, _object: Box<Object2D>) {}
    fn resize(&self, size: Size) {
        self.canvas.set_height(size.height as u32);
        self.canvas.set_width(size.width as u32);
    }
}

pub struct Canvas {
    state: Rc<RefCell<CanvasState>>,
}

pub struct CanvasState {
    root_frame: Option<CanvasFrame>,
    size: ObserverCell<Size>,
}

impl Graphics2D for Canvas {
    type Frame = CanvasFrame;
    fn run(&self, root: CanvasFrame) {
        let mut state = self.state.borrow_mut();
        state.root_frame = Some(root);
        let cloned = self.clone();
        window().request_animation_frame(move |delta| {
            cloned.animate(delta);
        });
    }
    fn frame(&self) -> CanvasFrame {
        let d = document();
        let canvas: CanvasElement = d.create_element("canvas").unwrap().try_into().unwrap();
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
        CanvasFrame { canvas, context }
    }
}

impl Canvas {
    fn animate(&self, _delta: f64) {
        let state = self.state.borrow();
        match &state.root_frame {
            Some(frame) => {
                if state.size.is_dirty() {
                    frame.resize(state.size.get());
                }
            }
            None => {}
        }
        let cloned = self.clone();
        window().request_animation_frame(move |delta| {
            cloned.animate(delta);
        });
    }
}

impl Clone for Canvas {
    fn clone(&self) -> Canvas {
        Canvas {
            state: self.state.clone(),
        }
    }
}

pub fn new() -> impl Graphics2D {
    document()
        .head()
        .unwrap()
        .append_html(
            r#"
<style>
body, html, canvas {
    height: 100%;
}
body {
    margin: 0;
    overflow: hidden;
}
canvas {
    width: 100%;
}
</style>
            "#,
        )
        .unwrap();

    let body = document().body().unwrap();

    let gfx = Canvas {
        state: Rc::new(RefCell::new(CanvasState {
            size: ObserverCell::new(Size {
                width: f64::from(body.offset_width()),
                height: f64::from(body.offset_height()),
            }),
            root_frame: None,
        })),
    };

    let gfx_resize = gfx.clone();

    window().add_event_listener(move |_: ResizeEvent| {
        let state = gfx_resize.state.borrow();
        let body = document().body().unwrap();
        state.size.set(Size {
            width: f64::from(body.offset_width()),
            height: f64::from(body.offset_height()),
        });
    });

    gfx
}
