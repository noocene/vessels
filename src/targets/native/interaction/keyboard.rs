use crate::interaction;
use crate::interaction::keyboard;
use crate::interaction::keyboard::{Event, Key};

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

impl interaction::Source for Keyboard {
    type Event = Event;
    fn bind(&self, handler: Box<dyn Fn(Self::Event) + 'static + Sync + Send>) {
        self.state.borrow_mut().handlers.push(handler);
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
    pub(crate) fn new(
        event_handler: Box<dyn interaction::Source<Event = glutin::Event>>,
    ) -> Box<dyn interaction::Keyboard> {
        let keyboard = Keyboard {
            state: Rc::new(RefCell::new(KeyboardState {
                handlers: vec![],
                keys: HashMap::new(),
            })),
        };
        keyboard.initialize(event_handler);
        Box::new(keyboard)
    }
    fn initialize(&self, event_handler: Box<dyn interaction::Source<Event = glutin::Event>>) {
        event_handler.bind(Box::new(move |event: glutin::Event| match event {
            _ => (),
        }));
    }
}
