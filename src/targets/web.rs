use crate::render::Renderer;

use std::rc::Rc;
use std::cell::RefCell;

use stdweb::web::html_element::CanvasElement;
use stdweb::web::{window, document, IHtmlElement, INode, IElement, IEventTarget};
use stdweb::web::event::ResizeEvent;

use stdweb::unstable::TryInto;

mod webgl_rendering_context;

use crate::targets::web::webgl_rendering_context::{WebGL2RenderingContext as gl, WebGLFramebuffer, WebGLRenderbuffer};

pub struct WebGL2 {
    state: Rc<RefCell<WebGL2State>>
}

struct WebGL2State {
    context: gl,
    canvas: CanvasElement,
    dpr: f64,
    resized: bool,
    width: u32,
    height: u32,
    framebuffer: WebGLFramebuffer,
    renderbuffer: WebGLRenderbuffer,
}

impl Renderer for WebGL2 {
    fn new() -> WebGL2 {
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
        let context: gl = js!(
            return @{&canvas}.getContext("webgl2", {
                antialias: false
            });
        ).try_into().unwrap();
        let body = doc.body().unwrap();
        body.append_child(&canvas);
        let (width, height) = ((f64::from(canvas.offset_width()) * dpr) as u32, ((f64::from(canvas.offset_height()) * dpr) as u32));
        canvas.set_width(width);
        canvas.set_height(height);
        let framebuffer = context.create_framebuffer().unwrap();
        context.bind_framebuffer(gl::FRAMEBUFFER, Some(&framebuffer));
        let renderbuffer = context.create_renderbuffer().unwrap();
        context.bind_renderbuffer(gl::RENDERBUFFER, Some(&renderbuffer));
        context.renderbuffer_storage_multisample(gl::RENDERBUFFER, 4, gl::RGBA8, width as i32, height as i32); 
        context.framebuffer_renderbuffer(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::RENDERBUFFER, Some(&renderbuffer));
        let state = Rc::new(RefCell::new(WebGL2State { width, height, context, framebuffer, renderbuffer, canvas, resized: false, dpr }));
        WebGL2{state}
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
            state.context.viewport(0, 0, state.canvas.width() as i32, state.canvas.height() as i32);
            state.draw(rc);
        });
    }
}

impl WebGL2State {
    fn draw(&mut self, rc: Rc<RefCell<Self>>) {
        if self.resized {
            let (width, height) = ((f64::from(self.canvas.offset_width()) * self.dpr) as u32, ((f64::from(self.canvas.offset_height()) * self.dpr) as u32));
            self.canvas.set_width(width);
            self.canvas.set_height(height);
            self.context.viewport(0, 0, width as i32, height as i32);
            self.width = width;
            self.height = height;
            self.context.delete_renderbuffer(Some(&self.renderbuffer));
            let renderbuffer = self.context.create_renderbuffer().unwrap();
            self.context.bind_framebuffer(gl::FRAMEBUFFER, Some(&self.framebuffer));
            self.context.bind_renderbuffer(gl::RENDERBUFFER, Some(&renderbuffer));
            self.context.renderbuffer_storage_multisample(gl::RENDERBUFFER, 4, gl::RGBA8, width as i32, height as i32); 
            self.context.framebuffer_renderbuffer(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::RENDERBUFFER, Some(&renderbuffer));
            self.renderbuffer = renderbuffer;
            self.resized = false;
        }

        self.context.clear_color(0., 0., 0., 0.);
        self.context.clear(gl::COLOR_BUFFER_BIT);

        self.context.bind_framebuffer(gl::READ_FRAMEBUFFER, Some(&self.framebuffer));
        self.context.bind_framebuffer(gl::DRAW_FRAMEBUFFER, None);
        self.context.blit_framebuffer(0, 0, self.width as i32, self.height as i32, 0, 0, self.width as i32, self.height as i32, gl::COLOR_BUFFER_BIT, gl::NEAREST);

        window().request_animation_frame( move |_time| {
            let mut state = rc.borrow_mut();
            let rc = rc.clone();
            state.draw(rc);
        });
    }
}
