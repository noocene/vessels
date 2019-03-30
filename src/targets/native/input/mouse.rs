use crate::graphics_2d::Vector;
use crate::input;
use crate::input::mouse::Event;

use std::cell::RefCell;
use std::rc::Rc;

pub(crate) struct MouseState {
    handlers: Vec<Box<dyn Fn(Event)>>,
    position: Vector,
}

pub(crate) struct Mouse {
    state: Rc<RefCell<MouseState>>,
}

impl input::Source for Mouse {
    type Event = Event;
    fn bind(&self, handler: Box<dyn Fn(Self::Event) + 'static>) {
        self.state.borrow_mut().handlers.push(handler);
    }
}

impl input::Mouse for Mouse {
    fn position(&self) -> Vector {
        self.state.borrow().position
    }
}

impl Mouse {
    pub(crate) fn new() -> Box<dyn input::Mouse> {
        let mouse = Mouse {
            state: Rc::new(RefCell::new(MouseState {
                handlers: vec![],
                position: Vector::default(),
            })),
        };
        mouse.initialize();
        Box::new(mouse)
    }
    fn initialize(&self) {
    }
}