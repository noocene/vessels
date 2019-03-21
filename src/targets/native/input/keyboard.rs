use crate::input;
use crate::input::keyboard;
use crate::input::keyboard::{Event, Key};

use std::cell::RefCell;
use std::rc::Rc;

use std::collections::HashMap;

pub(crate) struct KeyboardState {
    handlers: Vec<Box<dyn Fn(Event)>>,
    keys: HashMap<Key, bool>,
}

pub(crate) struct Keyboard {
    state: Rc<RefCell<KeyboardState>>,
}

impl input::Source for Keyboard {
    type Event = Event;
    fn bind<F>(&self, handler: F)
    where
        F: Fn(Event) + 'static,
    {
        self.state.borrow_mut().handlers.push(Box::new(handler));
    }
}

impl keyboard::Keyboard for Keyboard {}

impl keyboard::State for Keyboard {
    fn poll(&mut self, key: Key) -> bool {
        let mut state = self.state.borrow_mut();
        let entry = state.keys.entry(key).or_insert(false);
        *entry
    }
}

impl keyboard::State for KeyboardState {
    fn poll(&mut self, key: Key) -> bool {
        *self.keys.entry(key).or_insert(false)
    }
}

impl Keyboard {
    pub(crate) fn new() -> Keyboard {
        let keyboard = Keyboard {
            state: Rc::new(RefCell::new(KeyboardState {
                handlers: vec![],
                keys: HashMap::new(),
            })),
        };
        keyboard.initialize();
        keyboard
    }
    fn initialize(&self) {
    }
}