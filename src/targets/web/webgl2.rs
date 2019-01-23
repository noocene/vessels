pub struct WebGL2 {
    state: Rc<RefCell<WebGL2State>>,
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
        doc.head()
            .unwrap()
            .append_html(
                r#"
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
            overflow: hidden;
            height: 100%;
        }
        </style>
        "#,
            )
            .unwrap();
        let win = window();
        let dpr = win.device_pixel_ratio();
        let canvas: CanvasElement = doc.create_element("canvas").unwrap().try_into().unwrap();
        let ctx: gl = js!(
            return @{&canvas}.getContext("webgl2", {
                antialias: false,
                alpha: false,
            });
        )
        .try_into()
        .unwrap();
        let body = doc.body().unwrap();
        body.append_child(&canvas);
        let (width, height) = (
            (f64::from(canvas.offset_width()) * dpr) as i32,
            ((f64::from(canvas.offset_height()) * dpr) as i32),
        );
        canvas.set_width(width as u32);
        canvas.set_height(height as u32);
        let framebuffer = ctx.create_framebuffer().unwrap();
        ctx.bind_framebuffer(gl::FRAMEBUFFER, Some(&framebuffer));
        let renderbuffer = ctx.create_renderbuffer().unwrap();
        ctx.bind_renderbuffer(gl::RENDERBUFFER, Some(&renderbuffer));
        ctx.renderbuffer_storage(gl::RENDERBUFFER, gl::RGB8, width, height);
        ctx.framebuffer_renderbuffer(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::RENDERBUFFER,
            Some(&renderbuffer),
        );
        let context = Rc::new(RefCell::new(ctx));
        let root_frame_state = Rc::new(RefCell::new(WebGL2RootFrameState {
            width,
            height,
            framebuffer,
            renderbuffer,
            context: context.clone(),
            children: PtrWeakHashSet::new(),
        }));
        let root_frame = WebGL2RootFrame {
            state: root_frame_state,
        };
        let state = Rc::new(RefCell::new(WebGL2State {
            width,
            height,
            root_frame,
            context,
            canvas,
            resized: false,
            dpr,
        }));
        WebGL2 { state }
    }
    fn run(&self) {
        let win = window();

        {
            let state = self.state.clone();
            win.add_event_listener(move |_: ResizeEvent| {
                let mut x = state.borrow_mut();
                x.resized = true;
            });
        }

        let state = self.state.clone();

        win.request_animation_frame(move |_time| {
            let rc = state.clone();
            let mut state = state.borrow_mut();
            state.context.borrow().viewport(
                0,
                0,
                state.canvas.width() as i32,
                state.canvas.height() as i32,
            );
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
            let (w, h) = (
                (f64::from(self.canvas.offset_width()) * self.dpr) as i32,
                ((f64::from(self.canvas.offset_height()) * self.dpr) as i32),
            );
            self.canvas.set_width(w as u32);
            self.canvas.set_height(h as u32);
            ctx.viewport(0, 0, w, h);
            self.width = w;
            self.height = h;
            self.root_frame.resize(Size { w, h });
            self.resized = false;
        }

        ctx.bind_framebuffer(gl::FRAMEBUFFER, None);
        ctx.clear_color(0., 0., 0., 0.);
        ctx.clear(gl::COLOR_BUFFER_BIT);

        self.root_frame.draw();

        window().request_animation_frame(move |_time| {
            let mut state = rc.borrow_mut();
            let rc = rc.clone();
            state.draw(rc);
        });
    }
}