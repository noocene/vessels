use crate::graphics_2d::*;
use crate::input::*;
use crate::text::*;
use crate::path::*;
use crate::util::ObserverCell;
use crate::targets::native;

use std::cell::{RefCell, Cell};
use std::rc::Rc;

use glutin::ContextTrait;

type CairoImage = cairo::Context;

/*
impl ImageRepresentation for CairoImage {
    fn get_size(&self) -> Vector {
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
    }
}
*/

struct CairoFrameState {
    cairo_context: cairo::Context,
    contents: Vec<Object>,
    viewport: Cell<Rect>,
    size: Cell<Vector>,
}

impl Drop for CairoFrameState {
    fn drop(&mut self) {
    }
}

struct CairoFrame {
    state: Rc<RefCell<CairoFrameState>>
}

impl Clone for CairoFrame {
    fn clone(&self) -> Self {
        CairoFrame {
            state: self.state.clone()
        }
    }
}
/*
impl DynamicObject for CairoFrame {

}

impl Frame for CairoFrame {

}
*/
struct Window {
    state: Rc<RefCell<WindowState>>,
}

struct WindowState {
    root_frame: Option<CairoFrame>,
    windowed_context: glutin::WindowedContext,
    event_loop: RefCell<glutin::EventsLoop>,
    size: ObserverCell<Vector>,
}

/*
impl Rasterizer for Window {
    type Image = CairoImage;
}
*/

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
/*
impl ContextGraphics for Window {}

impl ContextualGraphics for Window {
    type Context = Window;
    fn run(self, root: CairoFrame) -> Self::Context {

    }
}

impl Graphics for Window {
    type Frame = CairoFrame;
    fn frame(&self) -> CairoFrame {
        CairoFrame::new()
    }
}
*/
impl Clone for Window {
    fn clone(&self) -> Window {
        Window {
            state: self.state.clone(),
        }
    }
}

pub(crate) fn new() { // -> Window {
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
            event_loop: RefCell::new(el),
            root_frame: None,
        })),
    };

    let mut running = true;
    while running {
        let state = window.state.borrow();
        state.event_loop.borrow_mut().poll_events(|event| {
            println!("{:?}", event);
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = state.windowed_context.get_hidpi_factor();
                        state.windowed_context
                            .resize(logical_size.to_physical(dpi_factor));
                    }
                    _ => (),
                },
                _ => (),
            }
        });
    }
    //window
}