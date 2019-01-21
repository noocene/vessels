use crate::render::{Renderer, RootFrame, Frame, Size, Point, Rect};

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::hash::{Hash, Hasher};

use std::collections::HashSet;

use stdweb::web::html_element::CanvasElement;
use stdweb::web::{window, document, IHtmlElement, INode, IElement, IEventTarget, TypedArray};
use stdweb::web::event::ResizeEvent;

use stdweb::unstable::TryInto;

mod webgl_rendering_context;

use crate::targets::web::webgl_rendering_context::{WebGL2RenderingContext as gl, WebGLFramebuffer, WebGLRenderbuffer, WebGLTexture};

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
    root_frame: WebGL2RootFrame,
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
        let root_frame_state = Rc::new(RefCell::new(WebGL2RootFrameState{
            width,
            height,
            framebuffer,
            renderbuffer,
            context: context.clone(),
            children: HashSet::new(),
        }));
        let root_frame = WebGL2RootFrame{
            state: root_frame_state,
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
    fn root(&self) -> Box<dyn RootFrame> {
        Box::new(self.state.borrow().root_frame.clone())
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

        self.root_frame.draw();

        window().request_animation_frame( move |_time| {
            let mut state = rc.borrow_mut();
            let rc = rc.clone();
            state.draw(rc);
        });
    }
}

pub struct WebGL2FrameWeak {
    state: Weak<RefCell<WebGL2FrameState>>
}

impl WebGL2FrameWeak {
    fn upgrade(&self) -> Option<WebGL2Frame> {
        match self.state.upgrade() {
            None => None,
            Some(state) => Some(WebGL2Frame{
                state
            })
        }
    }
}

pub struct WebGL2Frame {
    state: Rc<RefCell<WebGL2FrameState>>
}

pub struct WebGL2RootFrame {
    state: Rc<RefCell<WebGL2RootFrameState>>
}

struct WebGL2RootFrameState {
    width: i32,
    height: i32,
    framebuffer: WebGLFramebuffer,
    renderbuffer: WebGLRenderbuffer,
    children: HashSet<WebGL2FrameWeak>,
    context: Rc<RefCell<gl>>,
}

struct WebGL2FrameState {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    clip_start: Option<Point>,
    clip_end: Option<Point>,
    framebuffer: WebGLFramebuffer,
    render_target: WebGLTexture,
    children: HashSet<WebGL2FrameWeak>,
    context: Rc<RefCell<gl>>
}

impl Drop for WebGL2FrameState {
    fn drop(&mut self) {
        let ctx = self.context.borrow();
        ctx.delete_texture(Some(&self.render_target));
        ctx.delete_framebuffer(Some(&self.framebuffer));
    }
}

impl Frame for WebGL2Frame { 
    fn resize(&mut self, size: Size) {
        let mut state = self.state.borrow_mut();
        state.width = size.w;
        state.height = size.h;
        let render_target = {
            let ctx = state.context.borrow();
            ctx.delete_texture(Some(&state.render_target));
            ctx.bind_framebuffer(gl::FRAMEBUFFER, Some(&state.framebuffer));
            let render_target = ctx.create_texture().unwrap();
            ctx.bind_texture(gl::TEXTURE_2D, Some(&render_target));
            ctx.tex_image2_d(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, size.w, size.h, 0, gl::RGBA8, gl::UNSIGNED_BYTE, None::<&TypedArray<u8>>);
            ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            ctx.framebuffer_texture2_d(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, Some(&render_target), 0);
            render_target
        };
        state.render_target = render_target;
    }
    fn clip(&mut self, start: Option<Point>, end: Option<Point>) {
        let mut state = self.state.borrow_mut();
        state.clip_start = start;
        state.clip_end = end;
    }
    fn position(&mut self, position: Point) {
        let mut state = self.state.borrow_mut();
        state.x = position.x;
        state.y = position.y;
    }
    fn new(&mut self, bounds: Rect) -> Box<dyn Frame> {
        let mut state = self.state.borrow_mut();

        let child = {
            let ctx = state.context.borrow();
            let framebuffer = ctx.create_framebuffer().unwrap();
            ctx.bind_framebuffer(gl::FRAMEBUFFER, Some(&framebuffer));
            let render_target = ctx.create_texture().unwrap();
            ctx.bind_texture(gl::TEXTURE_2D, Some(&render_target));
            ctx.tex_image2_d(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, bounds.w, bounds.h, 0, gl::RGBA8, gl::UNSIGNED_BYTE, None::<&TypedArray<u8>>);
            ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            ctx.framebuffer_texture2_d(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, Some(&render_target), 0);

            WebGL2Frame{
                state: Rc::new(RefCell::new(WebGL2FrameState {
                    width: bounds.w,
                    height: bounds.h,
                    x: bounds.x,
                    y: bounds.y,
                    framebuffer,
                    render_target,
                    context: state.context.clone(),
                    clip_start: None,
                    clip_end: None,
                    children: HashSet::new(),
                }))
            }
        };

        state.children.insert(child.downgrade());

        Box::new(child)
    }
}

impl RootFrame for WebGL2RootFrame { 
    fn new(&mut self, bounds: Rect) -> Box<dyn Frame> {
        let mut state = self.state.borrow_mut();

        let child = {
            let ctx = state.context.borrow();
            let framebuffer = ctx.create_framebuffer().unwrap();
            ctx.bind_framebuffer(gl::FRAMEBUFFER, Some(&framebuffer));
            let render_target = ctx.create_texture().unwrap();
            ctx.bind_texture(gl::TEXTURE_2D, Some(&render_target));
            ctx.tex_image2_d(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, bounds.w, bounds.h, 0, gl::RGBA8, gl::UNSIGNED_BYTE, None::<&TypedArray<u8>>);
            ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            ctx.framebuffer_texture2_d(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, Some(&render_target), 0);

            WebGL2Frame{
                state: Rc::new(RefCell::new(WebGL2FrameState {
                    width: bounds.w,
                    height: bounds.h,
                    x: bounds.x,
                    y: bounds.y,
                    framebuffer,
                    render_target,
                    context: state.context.clone(),
                    clip_start: None,
                    clip_end: None,
                    children: HashSet::new(),
                }))
            }
        };

        state.children.insert(child.downgrade());

        Box::new(child)
    }
}

impl WebGL2RootFrame {
    fn resize(&mut self, size: Size) {
        let mut state = self.state.borrow_mut();
        state.width = size.w;
        state.height = size.h;
        let renderbuffer = {
            let ctx = state.context.borrow();
            ctx.delete_renderbuffer(Some(&state.renderbuffer));
            let renderbuffer = ctx.create_renderbuffer().unwrap();
            ctx.bind_framebuffer(gl::FRAMEBUFFER, Some(&state.framebuffer));
            ctx.bind_renderbuffer(gl::RENDERBUFFER, Some(&renderbuffer));
            ctx.renderbuffer_storage_multisample(gl::RENDERBUFFER, 4, gl::RGBA8, size.w, size.h); 
            ctx.framebuffer_renderbuffer(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::RENDERBUFFER, Some(&renderbuffer));
            renderbuffer
        };
        state.renderbuffer = renderbuffer;
    }
    fn draw(&self) {
        let state = self.state.borrow();
        let ctx = state.context.borrow();
        for child in &state.children {
            child.upgrade().unwrap().draw(Some(&state.framebuffer));
        }
        ctx.bind_framebuffer(gl::READ_FRAMEBUFFER, Some(&state.framebuffer));
        ctx.bind_framebuffer(gl::DRAW_FRAMEBUFFER, None);
        ctx.blit_framebuffer(0, 0, state.width, state.height, 0, 0, state.width, state.height, gl::COLOR_BUFFER_BIT, gl::NEAREST);
    }
}

impl WebGL2Frame {
    fn draw(&self, target: Option<&WebGLFramebuffer>) {
        let state = self.state.borrow();
        for child in &state.children {
            child.upgrade().unwrap().draw(Some(&state.framebuffer));
        }
        let ctx = state.context.borrow();
        ctx.bind_framebuffer(gl::READ_FRAMEBUFFER, Some(&state.framebuffer));
        ctx.bind_framebuffer(gl::DRAW_FRAMEBUFFER, target);
        let (clip_x, clip_y) = match &state.clip_start {
            None => (0, 0),
            Some(clip) => (clip.x, clip.y)
        };
        let (clip_w, clip_h) = match &state.clip_end {
            None => (state.width, state.height),
            Some(clip) => (clip.x, clip.y)
        };
        ctx.blit_framebuffer(clip_x, clip_y, clip_w, clip_h, state.x, state.y, clip_w - clip_x, clip_h - clip_y, gl::COLOR_BUFFER_BIT, gl::NEAREST);
    }
    fn downgrade(&self) -> WebGL2FrameWeak {
        WebGL2FrameWeak{
            state: Rc::downgrade(&self.state)
        }
    }
}

impl Clone for WebGL2Frame {
    fn clone(&self) -> WebGL2Frame {
        WebGL2Frame{
            state: self.state.clone(),
        }
    }
}

impl Clone for WebGL2RootFrame {
    fn clone(&self) -> WebGL2RootFrame {
        WebGL2RootFrame{
            state: self.state.clone(),
        }
    }
}

impl Hash for WebGL2FrameWeak {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let interior = self.state.upgrade().unwrap();
        (&(*(interior.borrow())) as *const WebGL2FrameState).hash(state);
    }
}

impl PartialEq for WebGL2FrameWeak {
    fn eq(&self, other: &Self) -> bool {
        (&(*(self.state.upgrade().unwrap().borrow())) as *const WebGL2FrameState) == (&(*(other.state.upgrade().unwrap().borrow())) as *const WebGL2FrameState)
    }
}

impl Eq for WebGL2FrameWeak {}