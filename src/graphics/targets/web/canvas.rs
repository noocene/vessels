use crate::graphics::*;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, window, CanvasRenderingContext2d};

use stdweb::web::event::ResizeEvent;

use stdweb::web::html_element::CanvasElement;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Canvas2D {}

impl Representation for Canvas2D {}

impl Euclidean2D for Canvas2D {}

pub struct CanvasFrame {
    context: CanvasRenderingContext2d,
    canvas: CanvasElement,
}

impl<'a> Object<'a, Canvas2D> for CanvasFrame {}

impl<'a> Frame<'a, Canvas2D> for CanvasFrame {
    fn add(&self, _object: &'a Object<Canvas2D>) {}
    fn resize(&self, size: Size) {
        self.canvas.set_height(size.height as u32);
        self.canvas.set_width(size.width as u32);
    }
}

impl CanvasFrame {
    fn show(&self) {
        document().body().unwrap().append_child(&self.canvas);
    }
}

pub struct Canvas {
    state: Rc<RefCell<CanvasState<'static>>>,
}

pub struct CanvasState<'a> {
    root_frame: Option<&'a Frame<'a, Canvas2D>>,
    size: ObserverCell<Size>,
}

impl Graphics<'static, Canvas2D> for Canvas {
    fn run(&self, root: &'static Frame<Canvas2D>) {
        let mut state = self.state.borrow_mut();
        state.root_frame = Some(root);
        let cloned = self.clone();
        window().request_animation_frame(move |delta| {
            cloned.animate(delta);
        });
    }
    fn frame(&self) -> Box<Frame<Canvas2D>> {
        let d = document();
        let canvas: CanvasElement = d.create_element("canvas").unwrap().try_into().unwrap();
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
        Box::new(CanvasFrame { canvas, context })
    }
}

impl Graphics2D<'static> for Canvas {
    type R = Canvas2D;
}

impl<'a> GraphicsEmpty<'a> for Canvas {}

impl TryFrom<Canvas> for Canvas {
    type Error = ();
    fn try_from(value: Canvas) -> Result<Self, Self::Error> {
        Ok(value)
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

pub fn initialize() -> Box<Canvas> {
    stdweb::initialize();

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

    Box::new(gfx)
}
