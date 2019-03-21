use crate::graphics_2d::*;
use crate::input::*;
use crate::text::*;
use crate::path::*;
use crate::util::ObserverCell;
use crate::targets::native;

use std::cell::{RefCell, Cell};
use std::rc::Rc;

use glutin::ContextTrait;

struct CairoFrameState {
    cairo_context: cairo::Context,
    contents: Vec<Object>,
    viewport: Cell<Rect>,
    size: Cell<Vector>,
}

impl Drop for CairoFrameState {
    fn drop(&mut self) {
        self.canvas.remove();
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

impl DynamicObject for CairoFrame {

}

impl Frame for CairoFrame {

}

struct Window {
    state: Rc<RefCell<WindowState>>,
}

struct WindowState {
    root_frame: Option<CairoFrame>,
    windowed_context: glutin::WindowedContext,
    event_loop: glutin::EventsLoop,
    size: ObserverCell<Vector>,
}

impl Rasterizer for Window {

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

    let window = Window {
        state: Rc::new(RefCell::new(WindowState {
            size: ObserverCell::new(),
            root_frame: None,
        })),
    };
    window
}