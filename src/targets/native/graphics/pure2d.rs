use crate::graphics_2d::*;
use crate::input::*;
use crate::text::*;
use crate::path::*;
use crate::util::ObserverCell;
use crate::targets::native;

use std::cell::{RefCell, Cell};
use std::rc::Rc;
use std::sync::Arc;
use std::borrow::Cow;
use std::thread;
use std::collections::VecDeque;

use glutin::{ContextTrait, ControlFlow};

use cairo::{ImageSurface, Format};

type CairoImage = ImageSurface;

impl ImageRepresentation for CairoImage {
    fn get_size(&self) -> Vector {
        (self.get_width() as f64, self.get_height() as f64).into()
    }

    fn box_clone(&self) -> Box<dyn ImageRepresentation> {
        Box::new(self.clone())
    }

    fn as_texture(&self) -> Image<RGBA8, Texture2D> {
        Image {
            pixels: vec![],
            format: Texture2D {
                height: 0,
                width: 0,
            },
        }
    }
    
    fn from_texture(texture: Image<RGBA8, Texture2D>) -> CairoImage {
        ImageSurface::create(Format::ARgb32, texture.format.width as i32, texture.format.height as i32).unwrap()
    }
}

struct CairoFrameState {
    context: cairo::Context,
    surface: ImageSurface,
    contents: Vec<Object>,
    viewport: Cell<Rect>,
    size: Cell<Vector>,
}

struct CairoFrame {
    state: Rc<RefCell<CairoFrameState>>
}

impl CairoFrame {
    fn new() -> CairoFrame {
        let size = Vector::default();
        let surface = ImageSurface::create(Format::ARgb32, size.x as i32, size.y as i32).unwrap();
        CairoFrame {
            state: Rc::new(RefCell::new(CairoFrameState {
                context: cairo::Context::new(&surface),
                surface: surface,
                contents: vec![],
                size: Cell::from(size),
                viewport: Cell::from(Rect {
                    size: Vector::default(),
                    position: (0., 0.).into(),
            }),
            }))
        }
    }

    fn draw(&self) {

    }
}

impl Clone for CairoFrame {
    fn clone(&self) -> Self {
        CairoFrame {
            state: self.state.clone()
        }
    }
}

impl DynamicObject for CairoFrame {
    fn orientation(&self) -> Transform {
        Transform::default()
    }

    fn render(&self) -> Cow<'_, [Path]> {
        let state = self.state.borrow();
        self.draw();
        let size = state.size.get();
        Cow::from(vec![Path {
            orientation: Transform::default(),
            fill: Some(Fill {
                content: Texture::Image(Box::new(state.surface.clone())),
            }),
            shadow: None,
            stroke: None,
            closed: true,
            segments: vec![
                Segment::LineTo((0., 0.).into()),
                Segment::LineTo((0., size.y).into()),
                Segment::LineTo(size),
                Segment::LineTo((size.x, 0.).into()),
            ],
        }])
    }
}

impl Frame for CairoFrame {
    type Image = CairoImage;

    fn add<U>(&mut self, object: U) where U: Into<Object> {
        let mut state = self.state.borrow_mut();
        state.contents.push(object.into());
    }

    fn set_viewport(&self, viewport: Rect) {
        let state = self.state.borrow();
        state.viewport.set(viewport);
    }

    fn resize<T>(&self, size: T) where T: Into<Vector> {
        let state = self.state.borrow();
        let size = size.into();
        state.size.set(size);
        //TODO: Actual resizing
    }
    fn get_size(&self) -> Vector {
        let state = self.state.borrow();
        state.size.get()
    }
    fn to_image(&self) -> Box<CairoImage> {
        let state = self.state.borrow();
        self.draw();
        Box::new(state.surface.clone())
    }
}

struct EventHandler {
    handlers: Vec<Box<dyn Fn(glutin::Event) + Send + Sync>>,
    event_queue: Arc<VecDeque<glutin::Event>>,
}

impl EventHandler {
    fn new() -> EventHandler {
        EventHandler {
            handlers: vec![],
            event_queue: Arc::new(VecDeque::new()),
        }
    }

    fn event_queue_new(&self) -> Arc<VecDeque<glutin::Event>> {
        self.event_queue.clone()
    }

    fn bind_event_handler<F>(&self, handler: F)
    where
        F: Fn(glutin::Event) + Send + Sync + 'static,
    {
        self.handlers.push(Box::new(handler));
    }
}

struct Window {
    state: Rc<RefCell<WindowState>>,
}

struct WindowState {
    root_frame: Option<CairoFrame>,
    windowed_context: glutin::WindowedContext,
    event_handler: EventHandler,
    event_loop: glutin::EventsLoop,
    size: ObserverCell<Vector>,
}

impl Rasterizer for Window {
    type Image = CairoImage;

    //todo pepega, make sure to update with dpr
    fn rasterize<'a, T>(&self, input: T) -> Self::Image where T: Into<Rasterizable<'a>> {
        let input = input.into(); 
        let surface = match input {
            Rasterizable::Text(input) => {
                let mut lines: Vec<String> = input.content.split('\n').map(std::borrow::ToOwned::to_owned).collect();
                let height = (f64::from(input.line_height)) as i32 * ((lines.len() - 1).max(0) as i32) + (f64::from(input.size)) as i32;
                let width =  match input.max_width {
                    None => {
                        //todo pepega
                        5
                    }
                    Some(max_width) => {
                        (f64::from(max_width)) as i32
                    }
                };
                ImageSurface::create(Format::ARgb32, height, width).unwrap()
            }
        }; 
        surface
    }
}

impl Context for Window {
    type Mouse = native::input::Mouse;
    type Keyboard = native::input::Keyboard;
    fn mouse(&self) -> Self::Mouse {
        native::input::Mouse::new()
    }
    fn keyboard(&self) -> Self::Keyboard {
        native::input::Keyboard::new()
    }
}

impl ContextGraphics for Window {}

impl ContextualGraphics for Window {
    type Context = Window;

    fn run(self, root: CairoFrame) -> Self::Context {
        {
            let state = self.state.borrow();
            let event_queue = state.event_handler.event_queue_new();
            thread::spawn(move || {
                state.event_loop.run_forever(move |event| {
                    event_queue.push_back(event);
                    ControlFlow::Continue
                });
            });
        }
        self
    }
}

impl Graphics for Window {
    type Frame = CairoFrame;
    fn frame(&self) -> CairoFrame {
        CairoFrame::new()
    }
}

impl Clone for Window {
    fn clone(&self) -> Window {
        Window {
            state: self.state.clone(),
        }
    }
}

pub(crate) fn new() -> impl ContextualGraphics {
    let el = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new();
    let windowed_context = glutin::ContextBuilder::new()
        .build_windowed(wb, &el)
        .unwrap();

    unsafe { windowed_context.make_current().unwrap() }

    let window = Window {
        state: Rc::new(RefCell::new(WindowState {
            size: ObserverCell::new((5.0, 10.0).into()),
            windowed_context,
            event_handler: EventHandler::new(),
            event_loop: el,
            root_frame: None,
        })),
    };

    window
}