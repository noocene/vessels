use crate::graphics::*;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, window, CanvasRenderingContext2d};

use stdweb::web::event::ResizeEvent;

use stdweb::web::html_element::CanvasElement;

use std::cell::RefCell;
use std::rc::Rc;

pub struct CanvasRepresentation {}

impl Representation for CanvasRepresentation {}

impl Euclidean2D for CanvasRepresentation {}

pub struct CanvasFrame {
    context: CanvasRenderingContext2d,
    canvas: CanvasElement,
}

impl Object<CanvasRepresentation> for CanvasFrame {}

impl Frame<CanvasRepresentation> for CanvasFrame {
    fn add(&self, _object: Box<Object<CanvasRepresentation>>) {}
    fn resize(&self, size: Size) {
        self.canvas.set_height(size.height as u32);
        self.canvas.set_width(size.width as u32);
    }
}

pub struct Canvas {
    state: Rc<RefCell<CanvasState>>,
}

pub struct CanvasState {
    root_frame: Option<Box<Frame<CanvasRepresentation>>>,
    size: ObserverCell<Size>,
}

impl Graphics for Canvas {
    type Representation = CanvasRepresentation;
    fn run(&self, root: Box<Frame<CanvasRepresentation>>) {
        let mut state = self.state.borrow_mut();
        state.root_frame = Some(root);
        let cloned = self.clone();
        window().request_animation_frame(move |delta| {
            cloned.animate(delta);
        });
    }
    fn frame(&self) -> Box<Frame<CanvasRepresentation>> {
        let d = document();
        let canvas: CanvasElement = d.create_element("canvas").unwrap().try_into().unwrap();
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
        Box::new(CanvasFrame { canvas, context })
    }
}

impl Graphics2D for Canvas {}

impl TryFrom<AbstractGraphics> for Box<Canvas> {
    type Error = ();
    fn try_from(_value: AbstractGraphics) -> Result<Self, Self::Error> {
        Ok(initialize())
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
