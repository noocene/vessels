use crate::graphics_2d::Vector;
use crate::interaction;
use crate::interaction::mouse::Event;

use std::cell::RefCell;
use std::rc::Rc;

pub(crate) struct MouseState {
    handlers: Vec<Box<dyn Fn(Event)>>,
    position: Vector,
}

pub(crate) struct Mouse {
    state: Rc<RefCell<MouseState>>,
}

impl interaction::Source for Mouse {
    type Event = Event;
    fn bind(&self, handler: Box<dyn Fn(Self::Event) + 'static>) {
        self.state.borrow_mut().handlers.push(handler);
    }
}

impl interaction::Mouse for Mouse {
    fn position(&self) -> Vector {
        self.state.borrow().position
    }
}

impl Mouse {
    pub(crate) fn new() -> Box<dyn interaction::Mouse> {
        let mouse = Mouse {
            state: Rc::new(RefCell::new(MouseState {
                handlers: vec![],
                position: Vector::default(),
            })),
        };
        mouse.initialize();
        Box::new(mouse)
    }
    fn initialize(&self) {}
}
