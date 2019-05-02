use crate::interaction;
use crate::interaction::keyboard;
use crate::interaction::keyboard::{Action, Alpha, Event, Key, Location, Number};

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

fn parse_code(scancode: u32) -> Key {
    Key::Unknown
}

pub(crate) struct KeyboardState {
    handlers: Vec<Box<dyn Fn(Event) + Send + Sync>>,
    keys: HashMap<Key, bool>,
}

pub(crate) struct Keyboard {
    state: Arc<RwLock<KeyboardState>>,
}

impl interaction::Source for Keyboard {
    type Event = Event;
    fn bind(&self, handler: Box<dyn Fn(Self::Event) + 'static + Sync + Send>) {
        self.state.write().unwrap().handlers.push(handler);
    }
}

impl keyboard::Keyboard for Keyboard {}

impl keyboard::State for Keyboard {
    fn poll(&mut self, key: Key) -> bool {
        let mut state = self.state.write().unwrap();
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
            state: Arc::new(RwLock::new(KeyboardState {
                handlers: vec![],
                keys: HashMap::new(),
            })),
        };
        keyboard.initialize(event_handler);
        Box::new(keyboard)
    }
    fn initialize(&self, event_handler: Box<dyn interaction::Source<Event = glutin::Event>>) {
        let state = self.state.clone();
        event_handler.bind(Box::new(move |event: glutin::Event| {
            let my_state = state.clone();
            let send_state = state.clone();
            let mut state = my_state.write().unwrap();
            if let glutin::Event::WindowEvent { event, .. } = event {
                if let glutin::WindowEvent::KeyboardInput { device_id, input } = event {
                    let key = parse_code(input.scancode);
                    match input.state {
                        glutin::ElementState::Pressed => state.keys.insert(key, true),
                        glutin::ElementState::Released => state.keys.insert(key, false),
                    };
                    let send_event = Event {
                        action: match input.state {
                            glutin::ElementState::Pressed => Action::Down(key),
                            glutin::ElementState::Released => Action::Up(key),
                        },
                        state: send_state,
                        //temp none
                        printable: None,
                    };
                    state
                        .handlers
                        .iter()
                        .for_each(|handler| handler(send_event.clone()));
                }
            }
        }));
    }
}
