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
    fn bind<F>(&self, handler: F)
    where
        F: Fn(Event) + 'static,
    {
        self.state.borrow_mut().handlers.push(Box::new(handler));
    }
}

impl input::Mouse for Mouse {
    fn position(&self) -> Vector {
        self.state.borrow().position
    }
}

impl Mouse {
    pub(crate) fn new() -> Mouse {
        let mouse = Mouse {
            state: Rc::new(RefCell::new(MouseState {
                handlers: vec![],
                position: Vector::default(),
            })),
        };
        mouse.initialize();
        mouse
    }
    fn initialize(&self) {
    }
}