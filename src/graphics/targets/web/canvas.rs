use crate::graphics::*;
use crate::util::*;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, window, CanvasRenderingContext2d};

use stdweb::web::event::ResizeEvent;

use stdweb::web::html_element::CanvasElement;

use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub struct Canvas {
    state: Rc<RefCell<CanvasState>>,
}

struct CanvasState {
    context: CanvasRenderingContext2d,
    canvas: CanvasElement,
    size: Cell<Size>,
    dirty: Cell<bool>,
}

impl Graphics for Canvas {
    fn run(&self, _root: Box<Frame<Object<Geometry, Material>>>) {}
}

impl GraphicsEmpty for Canvas {}

impl Graphics2D for Canvas {
    fn frame(&mut self) -> Frame2D {
        Box::new(CanvasFrame {})
    }
}

pub struct CanvasFrame {}

impl Object<dyn Geometry2D, TextureTarget2D> for CanvasFrame {}

impl<'a> Frame<'a, Object2D> for CanvasFrame {
    fn add(&self, _object: &'a Object2D) {}
}

impl crate::util::TryInto<Box<Graphics2D>> for Canvas {
    type Error = ();
    fn try_into(self) -> Result<Box<dyn Graphics2D>, Self::Error> {
        Ok(Box::new(self))
    }
}

impl Canvas {
    fn animate(&self, _delta: f64) {
        let state = self.state.borrow();
        if state.dirty.get() {
            console!(log, "test");
            state.canvas.set_width(state.size.get().width as u32);
            state.canvas.set_height(state.size.get().height as u32);
            state.dirty.set(false);
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

pub fn initialize() -> impl GraphicsEmpty {
    stdweb::initialize();

    let d = document();

    let canvas: CanvasElement = d.create_element("canvas").unwrap().try_into().unwrap();

    d.body().unwrap().append_child(&canvas);

    d.head()
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

    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

    let _gfx = Canvas {
        state: Rc::new(RefCell::new(CanvasState {
            context,
            size: Cell::new(Size {
                width: f64::from(canvas.offset_width()),
                height: f64::from(canvas.offset_height()),
            }),
            canvas,
            dirty: Cell::new(true),
        })),
    };

    let gfx = _gfx.clone();

    let gfx_resize = gfx.clone();

    window().add_event_listener(move |_: ResizeEvent| {
        let state = gfx_resize.state.borrow();
        state.size.set(Size {
            width: f64::from(state.canvas.offset_width()),
            height: f64::from(state.canvas.offset_height()),
        });
        state.dirty.set(true);
    });

    window().request_animation_frame(move |delta| {
        _gfx.animate(delta);
    });

    gfx
}
