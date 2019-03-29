use crate::graphics_2d::*;
use crate::input::*;
use crate::path::*;
use crate::targets::native;
use crate::text::*;
use crate::util::ObserverCell;

use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::{Arc, RwLock, Mutex};

use glutin::{ContextTrait, ControlFlow};

use cairo::{Format, ImageSurface};

use glib::SendValue;

type CairoSurface = SendValue;

struct CairoImage(Arc<Mutex<CairoSurface>>);

type CairoContext = SendValue;

impl ImageRepresentation for CairoImage {
    fn get_size(&self) -> Vector {
        (self.get_width() as f64, self.get_height() as f64).into()
    }

    fn box_clone(&self) -> Box<dyn ImageRepresentation> {
        Box::new(self.clone())
    }

    fn as_texture(&self) -> Image<Color, Texture2D> {
        Image {
            pixels: vec![],
            format: Texture2D {
                height: 0,
                width: 0,
            },
        }
    }

    fn from_texture(texture: Image<Color, Texture2D>) -> CairoImage {
        ImageSurface::create(
            Format::ARgb32,
            texture.format.width as i32,
            texture.format.height as i32,
        )
        .unwrap()
    }
}

struct CairoFrameState {
    context: SendValue,
    surface: CairoImage,
    contents: Vec<CairoObject>,
    viewport: Rect,
    size: Vector,
}

struct CairoFrame {
    state: Arc<Mutex<CairoFrameState>>,
}

impl CairoFrame {
    fn new() -> CairoFrame {
        let size = Vector::default();
        let surface = ImageSurface::create(Format::ARgb32, size.x as i32, size.y as i32).unwrap();
        CairoFrame {
            state: Arc::new(RwLock::new(CairoFrameState {
                context: cairo::Context::new(&surface),
                surface: surface,
                contents: vec![],
                size: Cell::from(size),
                viewport: Cell::from(Rect {
                    size: Vector::default(),
                    position: (0., 0.).into(),
                }),
            })),
        }
    }

    fn draw(&self) {}
}

impl Clone for CairoFrame {
    fn clone(&self) -> Self {
        CairoFrame {
            state: self.state.clone(),
        }
    }
}

impl Frame for CairoFrame {
    type Image = CairoImage;
    type Object = CairoObject;

    fn add<T, U>(&mut self, rasterizable: T, orientation: U) -> Box<dyn Object>
    where
        T: Into<Rasterizable>,
        U: Into<Transform>,
    {
        let object = CairoObject::new(rasterizable.into(), orientation.into());
        let mut state = self.state.write().unwrap();
        state.contents.push(object.clone());
        Box::new(object)
    }

    fn set_viewport(&self, viewport: Rect) {
        let state = self.state.borrow();
        state.viewport.set(viewport);
    }

    fn resize<T>(&self, size: T)
    where
        T: Into<Vector>,
    {
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

struct CairoObjectState {
    orientation: Transform,
    content: Rasterizable,
}

#[derive(Clone)]
struct CairoObject {
    state: Arc<RwLock<CairoObjectState>>,
}

impl CairoObject {
    fn new(content: Rasterizable, orientation: Transform) -> CairoObject {
        CairoObject {
            state: Arc::new(RwLock::new(CairoObjectState {
                orientation,
                content,
            })),
        }
    }
}

impl Object for CairoObject {
    fn get_transform(&self) -> Transform {
        self.state.read().unwrap().orientation
    }
    fn apply_transform(&mut self, transform: Transform) {
        self.state.write().unwrap().orientation.transform(transform);
    }
    fn set_transform(&mut self, transform: Transform) {
        self.state.write().unwrap().orientation = transform;
    }
    fn update(&mut self, input: Rasterizable) {
        self.state.write().unwrap().content = input;
    }
}

struct EventHandler {
    handlers: Vec<Box<dyn Fn(glutin::Event) + Send + Sync>>,
}

impl EventHandler {
    fn new() -> EventHandler {
        EventHandler { handlers: vec![] }
    }

    fn bind_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(glutin::Event) + Send + Sync + 'static,
    {
        self.handlers.push(Box::new(handler));
    }
}

struct Window {
    state: Arc<RwLock<WindowState>>,
}

struct WindowState {
    root_frame: Option<CairoFrame>,
    event_handler: EventHandler,
    size: ObserverCell<Vector>,
}

impl Ticker for Window {
    fn bind<F>(&mut self, handler: F)
    where
        F: FnMut(f64) + 'static + Send + Sync
    {}
}

impl Rasterizer for Window {
    type Image = CairoImage;

    //todo pepega, make sure to update with dpr
    fn rasterize<T>(&self, input: T, size: Vector) -> Box<dyn ImageRepresentation>
    where
        T: Into<Rasterizable>,
    {
        let input = input.into();
        let surface = match input {
            Rasterizable::Text(input) => {
                let mut lines: Vec<String> = input
                    .content
                    .split('\n')
                    .map(std::borrow::ToOwned::to_owned)
                    .collect();
                let height = (f64::from(input.line_height)) as i32
                    * ((lines.len() - 1).max(0) as i32)
                    + (f64::from(input.size)) as i32;
                let width = match input.max_width {
                    None => {
                        //todo pepega
                        5
                    }
                    Some(max_width) => (f64::from(max_width)) as i32,
                };
                ImageSurface::create(Format::ARgb32, height, width).unwrap()
            }
        };
        Box::new(surface)
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

impl ContextGraphics for Window {
    fn run(self) {}
}

impl ContextualGraphics for Window {
    type Context = Window;
    fn start(self, root: CairoFrame) -> Self::Context {
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
    let mut el = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new();
    let windowed_context = glutin::ContextBuilder::new()
        .build_windowed(wb, &el)
        .unwrap();
    unsafe { windowed_context.make_current().unwrap() }

    let mut running = true;
    while running {
        el.poll_events(|event| {
            //temporary event handling
            println!("{:?}", event);
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = windowed_context.get_hidpi_factor();
                        windowed_context.resize(logical_size.to_physical(dpi_factor));
                    }
                    _ => (),
                },
                _ => (),
            }
        });
    }
    let event_handler = EventHandler::new();

    let window = Window {
        state: Arc::new(RwLock::new(WindowState {
            size: ObserverCell::new((5.0, 10.0).into()),
            event_handler,
            root_frame: None,
        })),
    };

    window
}
