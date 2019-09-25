use crate::interaction;
use crate::interaction::keyboard;
use crate::interaction::keyboard::{Action, Event, Key};

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[cfg(target_os = "macos")]
mod scancode_macos {
    use crate::interaction::keyboard::{Alpha, Arrow, Function, Key, Location, Number, Numpad};

    pub(crate) static MAP: [Key; 127] = [
        Key::Alpha(Alpha::A),
        Key::Alpha(Alpha::S),
        Key::Alpha(Alpha::D),
        Key::Alpha(Alpha::F),
        Key::Alpha(Alpha::H),
        Key::Alpha(Alpha::G),
        Key::Alpha(Alpha::Z),
        Key::Alpha(Alpha::X),
        Key::Alpha(Alpha::C),
        Key::Alpha(Alpha::V),
        Key::Unknown, //Labelled as NonUsBackslash, not sure what it is
        Key::Alpha(Alpha::B),
        Key::Alpha(Alpha::Q),
        Key::Alpha(Alpha::W),
        Key::Alpha(Alpha::E),
        Key::Alpha(Alpha::R),
        Key::Alpha(Alpha::Y),
        Key::Alpha(Alpha::T),
        Key::Number(Number::One),
        Key::Number(Number::Two),
        Key::Number(Number::Three),
        Key::Number(Number::Four),
        Key::Number(Number::Six),
        Key::Number(Number::Five),
        Key::Equal,
        Key::Number(Number::Nine),
        Key::Number(Number::Seven),
        Key::Dash,
        Key::Number(Number::Eight),
        Key::Number(Number::Zero),
        Key::CloseBracket,
        Key::Alpha(Alpha::O),
        Key::Alpha(Alpha::U),
        Key::OpenBracket,
        Key::Alpha(Alpha::I),
        Key::Alpha(Alpha::P),
        Key::Enter,
        Key::Alpha(Alpha::L),
        Key::Alpha(Alpha::J),
        Key::Quote,
        Key::Alpha(Alpha::K),
        Key::Semicolon,
        Key::Backslash,
        Key::Comma,
        Key::Slash,
        Key::Alpha(Alpha::N),
        Key::Alpha(Alpha::M),
        Key::Period,
        Key::Tab,
        Key::Space,
        Key::Backtick,
        Key::Backspace,
        Key::Numpad(Numpad::Enter),
        Key::Escape,
        Key::Meta(Location::Right),
        Key::Meta(Location::Left),
        Key::Shift(Location::Left),
        Key::CapsLock,
        Key::Alt(Location::Left),
        Key::Control(Location::Left),
        Key::Shift(Location::Right),
        Key::Alt(Location::Right),
        Key::Control(Location::Right),
        Key::Unknown,
        Key::Unknown,
        Key::Numpad(Numpad::Decimal),
        Key::Unknown,
        Key::Numpad(Numpad::Multiply),
        Key::Unknown,
        Key::Numpad(Numpad::Add),
        Key::Unknown,
        Key::NumLock,
        Key::Unknown, //volumeup key
        Key::Unknown, //volumedown key
        Key::Unknown, //mute key
        Key::Numpad(Numpad::Divide),
        Key::Numpad(Numpad::Enter),
        Key::Unknown,
        Key::Numpad(Numpad::Subtract),
        Key::Unknown,
        Key::Unknown,
        Key::Numpad(Numpad::Equal),
        Key::Numpad(Numpad::Number(Number::Zero)),
        Key::Numpad(Numpad::Number(Number::One)),
        Key::Numpad(Numpad::Number(Number::Two)),
        Key::Numpad(Numpad::Number(Number::Three)),
        Key::Numpad(Numpad::Number(Number::Four)),
        Key::Numpad(Numpad::Number(Number::Five)),
        Key::Numpad(Numpad::Number(Number::Six)),
        Key::Numpad(Numpad::Number(Number::Seven)),
        Key::Unknown,
        Key::Numpad(Numpad::Number(Number::Eight)),
        Key::Numpad(Numpad::Number(Number::Nine)),
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Function(Function::F5),
        Key::Function(Function::F6),
        Key::Function(Function::F7),
        Key::Function(Function::F3),
        Key::Function(Function::F8),
        Key::Function(Function::F9),
        Key::Unknown,
        Key::Function(Function::F11),
        Key::Unknown,
        Key::Function(Function::F13),
        Key::Pause,
        Key::PrintScreen,
        Key::Unknown,
        Key::Function(Function::F10),
        Key::Unknown,
        Key::Function(Function::F12),
        Key::Unknown,
        Key::ScrollLock,
        Key::Insert,
        Key::Home,
        Key::PageUp,
        Key::Delete,
        Key::Function(Function::F4),
        Key::End,
        Key::Function(Function::F2),
        Key::PageDown,
        Key::Function(Function::F1),
        Key::Arrow(Arrow::Left),
        Key::Arrow(Arrow::Right),
        Key::Arrow(Arrow::Down),
        Key::Arrow(Arrow::Up),
    ];
}

#[cfg(target_os = "windows")]
mod scancode_windows {
    use crate::interaction::keyboard::{Alpha, Arrow, Function, Key, Location, Number, Numpad};

    pub(crate) static MAP: [Key; 127] = [
        Key::Unknown,
        Key::Escape,
        Key::Number(Number::One),
        Key::Number(Number::Two),
        Key::Number(Number::Three),
        Key::Number(Number::Four),
        Key::Number(Number::Five),
        Key::Number(Number::Six),
        Key::Number(Number::Seven),
        Key::Number(Number::Eight),
        Key::Number(Number::Nine),
        Key::Number(Number::Zero),
        Key::Dash,
        Key::Equal,
        Key::Backspace,
        Key::Tab,
        Key::Alpha(Alpha::Q),
        Key::Alpha(Alpha::W),
        Key::Alpha(Alpha::E),
        Key::Alpha(Alpha::R),
        Key::Alpha(Alpha::T),
        Key::Alpha(Alpha::Y),
        Key::Alpha(Alpha::U),
        Key::Alpha(Alpha::I),
        Key::Alpha(Alpha::O),
        Key::Alpha(Alpha::P),
        Key::OpenBracket,
        Key::CloseBracket,
        Key::Enter,
        Key::Control(Location::Left),
        Key::Alpha(Alpha::A),
        Key::Alpha(Alpha::S),
        Key::Alpha(Alpha::D),
        Key::Alpha(Alpha::F),
        Key::Alpha(Alpha::G),
        Key::Alpha(Alpha::H),
        Key::Alpha(Alpha::J),
        Key::Alpha(Alpha::K),
        Key::Alpha(Alpha::L),
        Key::Semicolon,
        Key::Quote,
        Key::Backtick,
        Key::Shift(Location::Left),
        Key::Backslash,
        Key::Alpha(Alpha::Z),
        Key::Alpha(Alpha::X),
        Key::Alpha(Alpha::C),
        Key::Alpha(Alpha::V),
        Key::Alpha(Alpha::B),
        Key::Alpha(Alpha::N),
        Key::Alpha(Alpha::M),
        Key::Comma,
        Key::Period,
        Key::Slash,
        Key::Shift(Location::Right),
        Key::Numpad(Numpad::Multiply),
        Key::Alt(Location::Left),
        Key::Space,
        Key::CapsLock,
        Key::Function(Function::F1),
        Key::Function(Function::F2),
        Key::Function(Function::F3),
        Key::Function(Function::F4),
        Key::Function(Function::F5),
        Key::Function(Function::F6),
        Key::Function(Function::F7),
        Key::Function(Function::F8),
        Key::Function(Function::F9),
        Key::Function(Function::F10),
        Key::NumLock,
        Key::ScrollLock,
        Key::Numpad(Numpad::Number(Number::Seven)),
        Key::Numpad(Numpad::Number(Number::Eight)),
        Key::Numpad(Numpad::Number(Number::Nine)),
        Key::Numpad(Numpad::Subtract),
        Key::Numpad(Numpad::Number(Number::Four)),
        Key::Numpad(Numpad::Number(Number::Five)),
        Key::Numpad(Numpad::Number(Number::Six)),
        Key::Numpad(Numpad::Add),
        Key::Numpad(Numpad::Number(Number::One)),
        Key::Numpad(Numpad::Number(Number::Two)),
        Key::Numpad(Numpad::Number(Number::Three)),
        Key::Numpad(Numpad::Number(Number::Zero)),
        Key::Numpad(Numpad::Decimal),
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Function(Function::F11),
        Key::Function(Function::F12),
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Numpad(Numpad::Enter),
        Key::Control(Location::Right),
        Key::Numpad(Numpad::Divide),
        Key::PrintScreen,
        Key::Alt(Location::Right),
        Key::Unknown,
        Key::Home,
        Key::Arrow(Arrow::Up),
        Key::PageUp,
        Key::Arrow(Arrow::Left),
        Key::Arrow(Arrow::Right),
        Key::End,
        Key::Arrow(Arrow::Down),
        Key::PageDown,
        Key::Insert,
        Key::Delete,
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Numpad(Numpad::Equal),
        Key::Unknown,
        Key::Pause,
        Key::Numpad(Numpad::Comma),
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Meta(Location::Left),
        Key::Meta(Location::Right),
    ];
}

#[cfg(target_os = "linux")]
mod scancode_linux {
    use crate::interaction::keyboard::{Alpha, Arrow, Function, Key, Location, Number, Numpad};

    pub(crate) static MAP: [Key; 127] = [
        Key::Unknown,
        Key::Escape,
        Key::Number(Number::One),
        Key::Number(Number::Two),
        Key::Number(Number::Three),
        Key::Number(Number::Four),
        Key::Number(Number::Five),
        Key::Number(Number::Six),
        Key::Number(Number::Seven),
        Key::Number(Number::Eight),
        Key::Number(Number::Nine),
        Key::Number(Number::Zero),
        Key::Dash,
        Key::Equal,
        Key::Backspace,
        Key::Tab,
        Key::Alpha(Alpha::Q),
        Key::Alpha(Alpha::W),
        Key::Alpha(Alpha::E),
        Key::Alpha(Alpha::R),
        Key::Alpha(Alpha::T),
        Key::Alpha(Alpha::Y),
        Key::Alpha(Alpha::U),
        Key::Alpha(Alpha::I),
        Key::Alpha(Alpha::O),
        Key::Alpha(Alpha::P),
        Key::OpenBracket,
        Key::CloseBracket,
        Key::Enter,
        Key::Control(Location::Left),
        Key::Alpha(Alpha::A),
        Key::Alpha(Alpha::S),
        Key::Alpha(Alpha::D),
        Key::Alpha(Alpha::F),
        Key::Alpha(Alpha::G),
        Key::Alpha(Alpha::H),
        Key::Alpha(Alpha::J),
        Key::Alpha(Alpha::K),
        Key::Alpha(Alpha::L),
        Key::Semicolon,
        Key::Quote,
        Key::Backtick,
        Key::Shift(Location::Left),
        Key::Backslash,
        Key::Alpha(Alpha::Z),
        Key::Alpha(Alpha::X),
        Key::Alpha(Alpha::C),
        Key::Alpha(Alpha::V),
        Key::Alpha(Alpha::B),
        Key::Alpha(Alpha::N),
        Key::Alpha(Alpha::M),
        Key::Comma,
        Key::Period,
        Key::Slash,
        Key::Shift(Location::Right),
        Key::Numpad(Numpad::Multiply),
        Key::Alt(Location::Left),
        Key::Space,
        Key::CapsLock,
        Key::Function(Function::F1),
        Key::Function(Function::F2),
        Key::Function(Function::F3),
        Key::Function(Function::F4),
        Key::Function(Function::F5),
        Key::Function(Function::F6),
        Key::Function(Function::F7),
        Key::Function(Function::F8),
        Key::Function(Function::F9),
        Key::Function(Function::F10),
        Key::NumLock,
        Key::ScrollLock,
        Key::Numpad(Numpad::Number(Number::Seven)),
        Key::Numpad(Numpad::Number(Number::Eight)),
        Key::Numpad(Numpad::Number(Number::Nine)),
        Key::Numpad(Numpad::Subtract),
        Key::Numpad(Numpad::Number(Number::Four)),
        Key::Numpad(Numpad::Number(Number::Five)),
        Key::Numpad(Numpad::Number(Number::Six)),
        Key::Numpad(Numpad::Add),
        Key::Numpad(Numpad::Number(Number::One)),
        Key::Numpad(Numpad::Number(Number::Two)),
        Key::Numpad(Numpad::Number(Number::Three)),
        Key::Numpad(Numpad::Number(Number::Zero)),
        Key::Numpad(Numpad::Decimal),
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Function(Function::F11),
        Key::Function(Function::F12),
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Numpad(Numpad::Enter),
        Key::Control(Location::Right),
        Key::Numpad(Numpad::Divide),
        Key::PrintScreen,
        Key::Alt(Location::Right),
        Key::Unknown,
        Key::Home,
        Key::Arrow(Arrow::Up),
        Key::PageUp,
        Key::Arrow(Arrow::Left),
        Key::Arrow(Arrow::Right),
        Key::End,
        Key::Arrow(Arrow::Down),
        Key::PageDown,
        Key::Insert,
        Key::Delete,
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Numpad(Numpad::Equal),
        Key::Unknown,
        Key::Pause,
        Key::Numpad(Numpad::Comma),
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Unknown,
        Key::Meta(Location::Left),
        Key::Meta(Location::Right),
    ];
}

mod scancode {
    #[cfg(target_os = "linux")]
    pub(crate) use super::scancode_linux::MAP;
    #[cfg(target_os = "macos")]
    pub(crate) use super::scancode_macos::MAP;
    #[cfg(target_os = "windows")]
    pub(crate) use super::scancode_windows::MAP;
}

fn parse_code(code: u32) -> Key {
    if (code as usize) < scancode::MAP.len() {
        scancode::MAP[code as usize]
    } else {
        Key::Unknown
    }
}

pub(crate) struct KeyboardState {
    handlers: Vec<Box<dyn Fn(Event) + Send + Sync>>,
    keys: HashMap<Key, bool>,
}

#[derive(Clone)]
pub(crate) struct Keyboard {
    state: Arc<RwLock<KeyboardState>>,
}

impl interaction::Source for Keyboard {
    type Event = Event;
    fn bind(&self, handler: Box<dyn Fn(Self::Event) + 'static + Sync + Send>) {
        self.state.write().unwrap().handlers.push(handler);
    }
}

impl keyboard::Keyboard for Keyboard {
    fn state(&self) -> Box<dyn keyboard::State> {
        Box::new(self.clone())
    }
}

impl keyboard::State for Keyboard {
    fn box_clone(&self) -> Box<dyn keyboard::State> {
        Box::new(self.clone())
    }
    fn poll(&mut self, key: Key) -> bool {
        let mut state = self.state.write().unwrap();
        let entry = state.keys.entry(key).or_insert(false);
        *entry
    }
}

impl Keyboard {
    #[allow(clippy::new_ret_no_self)]
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
        let state = self.clone();
        event_handler.bind(Box::new(move |event: glutin::Event| {
            let c_state = state.clone();
            let send_state = Box::new(state.clone());
            let mut state = c_state.state.write().unwrap();
            if let glutin::Event::WindowEvent { event, .. } = event {
                if let glutin::WindowEvent::KeyboardInput { input, .. } = event {
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
