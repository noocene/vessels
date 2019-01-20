use crate::render::{Renderer, Frame, Size, Point};

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
    context: Rc<RefCell<gl>>,
    canvas: CanvasElement,
    dpr: f64,
    resized: bool,
    width: i32,
    height: i32,
    root_frame: WebGL2Frame,
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
        let ctx: gl = js!(
            return @{&canvas}.getContext("webgl2", {
                antialias: false
            });
        ).try_into().unwrap();
        let body = doc.body().unwrap();
        body.append_child(&canvas);
        let (width, height) = ((f64::from(canvas.offset_width()) * dpr) as i32, ((f64::from(canvas.offset_height()) * dpr) as i32));
        canvas.set_width(width as u32);
        canvas.set_height(height as u32);
        let framebuffer = ctx.create_framebuffer().unwrap();
        ctx.bind_framebuffer(gl::FRAMEBUFFER, Some(&framebuffer));
        let renderbuffer = ctx.create_renderbuffer().unwrap();
        ctx.bind_renderbuffer(gl::RENDERBUFFER, Some(&renderbuffer));
        ctx.renderbuffer_storage_multisample(gl::RENDERBUFFER, 4, gl::RGBA8, width, height); 
        ctx.framebuffer_renderbuffer(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::RENDERBUFFER, Some(&renderbuffer));
        let context = Rc::new(RefCell::new(ctx));
        let root_frame = WebGL2Frame{
            width,
            height,
            x: 0,
            y: 0,
            framebuffer,
            renderbuffer,
            context: context.clone(),
            clip_start: None,
            clip_end: None,
            children: vec![],
        };
        let state = Rc::new(RefCell::new(WebGL2State { width, height, root_frame, context, canvas, resized: false, dpr }));
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
            state.context.borrow().viewport(0, 0, state.canvas.width() as i32, state.canvas.height() as i32);
            state.draw(rc);
        });
    }
}

impl WebGL2State {
    fn draw(&mut self, rc: Rc<RefCell<Self>>) {
        let ctx = self.context.borrow();

        if self.resized {
            let (w, h) = ((f64::from(self.canvas.offset_width()) * self.dpr) as i32, ((f64::from(self.canvas.offset_height()) * self.dpr) as i32));
            self.canvas.set_width(w as u32);
            self.canvas.set_height(h as u32);
            ctx.viewport(0, 0, w, h);
            self.width = w;
            self.height = h;
            self.root_frame.resize(Size{w, h});
            self.resized = false;
        }

        ctx.bind_framebuffer(gl::FRAMEBUFFER, None);
        ctx.clear_color(0., 0., 0., 0.);
        ctx.clear(gl::COLOR_BUFFER_BIT);

        self.root_frame.draw(None);

        window().request_animation_frame( move |_time| {
            let mut state = rc.borrow_mut();
            let rc = rc.clone();
            state.draw(rc);
        });
    }
}

pub struct WebGL2Frame {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    clip_start: Option<Point>,
    clip_end: Option<Point>,
    framebuffer: WebGLFramebuffer,
    renderbuffer: WebGLRenderbuffer,
    children: Vec<Rc<RefCell<WebGL2Frame>>>,
    context: Rc<RefCell<gl>>
}

impl Drop for WebGL2Frame {
    fn drop(&mut self) {
        let ctx = self.context.borrow();
        ctx.delete_renderbuffer(Some(&self.renderbuffer));
        ctx.delete_framebuffer(Some(&self.framebuffer));
    }
}

impl Frame for WebGL2Frame { 
    fn resize(&mut self, size: Size) {
        self.width = size.w;
        self.height = size.h;
        let ctx = self.context.borrow();
        ctx.delete_renderbuffer(Some(&self.renderbuffer));
        let renderbuffer = ctx.create_renderbuffer().unwrap();
        ctx.bind_framebuffer(gl::FRAMEBUFFER, Some(&self.framebuffer));
        ctx.bind_renderbuffer(gl::RENDERBUFFER, Some(&renderbuffer));
        ctx.renderbuffer_storage_multisample(gl::RENDERBUFFER, 4, gl::RGBA8, size.w, size.h); 
        ctx.framebuffer_renderbuffer(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::RENDERBUFFER, Some(&renderbuffer));
        self.renderbuffer = renderbuffer;
    }
    fn clip(&mut self, start: Option<Point>, end: Option<Point>) {
        self.clip_start = start;
        self.clip_end = end;
    }
    fn position(&mut self, position: Point) {
        self.x = position.x;
        self.y = position.y;
    }
    fn add_child(&mut self, child: Rc<RefCell<Self>>) {
        self.children.push(child);
    }
}

impl WebGL2Frame {
    fn draw(&self, target: Option<&WebGLFramebuffer>) {
        for child in &self.children {
            child.borrow().draw(Some(&self.framebuffer));
        }
        let ctx = self.context.borrow();
        ctx.bind_framebuffer(gl::FRAMEBUFFER, Some(&self.framebuffer));
        ctx.bind_framebuffer(gl::READ_FRAMEBUFFER, Some(&self.framebuffer));
        ctx.bind_framebuffer(gl::DRAW_FRAMEBUFFER, target);
        let (clip_x, clip_y) = match &self.clip_start {
            None => (0, 0),
            Some(clip) => (clip.x, clip.y)
        };
        let (clip_w, clip_h) = match &self.clip_end {
            None => (self.width, self.height),
            Some(clip) => (clip.x, clip.y)
        };
        ctx.blit_framebuffer(clip_x, clip_y, clip_w, clip_h, self.x, self.y, clip_w - clip_x, clip_h - clip_y, gl::COLOR_BUFFER_BIT, gl::NEAREST);
    }
}