use crate::interaction;
use crate::interaction::keyboard;
use crate::interaction::keyboard::{Action, Alpha, Event, Key, Location, Number};

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

fn parse_code(scancode: u32) -> Key {
    match scancode {
        1 => Key::Escape,
        2 => Key::Number(Number::One),
        3 => Key::Number(Number::Two),
        4 => Key::Number(Number::Three),
        5 => Key::Number(Number::Four),
        6 => Key::Number(Number::Five),
        7 => Key::Number(Number::Six),
        8 => Key::Number(Number::Seven),
        9 => Key::Number(Number::Eight),
        10 => Key::Number(Number::Nine),
        11 => Key::Number(Number::Zero),
        12 => Key::Dash,
        13 => Key::Equal,
        14 => Key::Backspace,
        15 => Key::Tab,
        16 => Key::Alpha(Alpha::Q),
        17 => Key::Alpha(Alpha::W),
        18 => Key::Alpha(Alpha::E),
        19 => Key::Alpha(Alpha::R),
        20 => Key::Alpha(Alpha::T),
        21 => Key::Alpha(Alpha::Y),
        22 => Key::Alpha(Alpha::U),
        23 => Key::Alpha(Alpha::I),
        24 => Key::Alpha(Alpha::O),
        25 => Key::Alpha(Alpha::P),
        26 => Key::OpenBracket,
        27 => Key::CloseBracket,
        28 => Key::Enter,
        29 => Key::Control(Location::Left),
        _ => Key::Unknown,
    }
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
