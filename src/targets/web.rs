use crate::render::Renderer;

use std::rc::Rc;
use std::cell::RefCell;

use stdweb::web::html_element::CanvasElement;
use stdweb::web::{window, document, IHtmlElement, INode, IElement, IEventTarget};
use stdweb::web::event::ResizeEvent;

use stdweb::unstable::TryInto;

mod webgl_rendering_context;

use crate::targets::web::webgl_rendering_context::WebGLRenderingContext as gl;

pub struct WebGL {
    state: Rc<RefCell<WebGLState>>
}

struct WebGLState {
    context: gl,
    canvas: CanvasElement,
    dpr: f64,
    resized: bool,
}

impl Renderer for WebGL {
    fn new() -> WebGL {
        stdweb::initialize();
        let doc = document();
        doc.head().unwrap().append_html(r#"
        <style>
        canvas {
            height: 100vh;
            width: 100vw;
            display: block;
        }
        body {
            margin: 0;
        }
        body, html {
            width: 100%;
            height: 100%;
        }
        </style>
        "#).unwrap();
        let win = window();
        let dpr = win.device_pixel_ratio();
        let canvas: CanvasElement = doc.create_element("canvas").unwrap().try_into().unwrap();
        let context: gl = canvas.get_context().unwrap();
        let body = doc.body().unwrap();
        body.append_child(&canvas);
        canvas.set_width((f64::from(canvas.offset_width()) * dpr) as u32);
        canvas.set_height((f64::from(canvas.offset_height()) * dpr) as u32);
        let state = Rc::new(RefCell::new(WebGLState { context, canvas, resized: false, dpr }));
        WebGL{state}
    }
    fn run(&self) {
        let win = window();

        {
            let state = self.state.clone();
            win.add_event_listener( move |_: ResizeEvent| {
                let mut x = state.borrow_mut();
                x.resized = true;
            });
        }
        
        let state = self.state.clone();

        win.request_animation_frame( move |_time| {
            let rc = state.clone();
            let mut state = state.borrow_mut();
            state.draw(rc);
        });
    }
}

impl WebGLState {
    fn draw(&mut self, rc: Rc<RefCell<Self>>) {
        if self.resized {
            self.canvas.set_width((f64::from(self.canvas.offset_width()) * self.dpr) as u32);
            self.canvas.set_height((f64::from(self.canvas.offset_height()) * self.dpr) as u32);
            self.resized = false;
        }

        self.context.clear_color(0.5, 0.5, 0.5, 0.9);
        self.context.viewport(0, 0, self.canvas.width() as i32, self.canvas.height() as i32);
        self.context.clear(gl::COLOR_BUFFER_BIT);

        window().request_animation_frame( move |_time| {
            let mut state = rc.borrow_mut();
            let rc = rc.clone();
            state.draw(rc);
        });
    }
}
