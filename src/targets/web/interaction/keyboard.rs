use crate::interaction;
use crate::interaction::keyboard;
use crate::interaction::keyboard::{
    Action, Alpha, Arrow, Event, Function, Key, Location, Number, Numpad,
};

use stdweb::traits::IKeyboardEvent;
use stdweb::web::event::{IEvent, KeyDownEvent, KeyUpEvent};
use stdweb::web::{document, IEventTarget};

use std::cell::RefCell;
use std::rc::Rc;

use std::collections::HashMap;

fn parse_code(code: &str) -> Key {
    match code {
        "Escape" => Key::Escape,
        "Digit0" => Key::Number(Number::Zero),
        "Digit1" => Key::Number(Number::One),
        "Digit2" => Key::Number(Number::Two),
        "Digit3" => Key::Number(Number::Three),
        "Digit4" => Key::Number(Number::Four),
        "Digit5" => Key::Number(Number::Five),
        "Digit6" => Key::Number(Number::Six),
        "Digit7" => Key::Number(Number::Seven),
        "Digit8" => Key::Number(Number::Eight),
        "Digit9" => Key::Number(Number::Nine),
        "Minus" => Key::Dash,
        "Equal" => Key::Equal,
        "Backspace" => Key::Backspace,
        "Tab" => Key::Tab,
        "KeyQ" => Key::Alpha(Alpha::Q),
        "KeyW" => Key::Alpha(Alpha::W),
        "KeyE" => Key::Alpha(Alpha::E),
        "KeyR" => Key::Alpha(Alpha::R),
        "KeyT" => Key::Alpha(Alpha::T),
        "KeyY" => Key::Alpha(Alpha::Y),
        "KeyU" => Key::Alpha(Alpha::U),
        "KeyI" => Key::Alpha(Alpha::I),
        "KeyO" => Key::Alpha(Alpha::O),
        "KeyP" => Key::Alpha(Alpha::P),
        "KeyA" => Key::Alpha(Alpha::A),
        "KeyS" => Key::Alpha(Alpha::S),
        "KeyD" => Key::Alpha(Alpha::D),
        "KeyF" => Key::Alpha(Alpha::F),
        "KeyG" => Key::Alpha(Alpha::G),
        "KeyH" => Key::Alpha(Alpha::H),
        "KeyJ" => Key::Alpha(Alpha::J),
        "KeyK" => Key::Alpha(Alpha::K),
        "KeyL" => Key::Alpha(Alpha::L),
        "KeyZ" => Key::Alpha(Alpha::Z),
        "KeyX" => Key::Alpha(Alpha::X),
        "KeyC" => Key::Alpha(Alpha::C),
        "KeyV" => Key::Alpha(Alpha::V),
        "KeyB" => Key::Alpha(Alpha::B),
        "KeyN" => Key::Alpha(Alpha::N),
        "KeyM" => Key::Alpha(Alpha::M),
        "BracketLeft" => Key::OpenBracket,
        "BracketRight" => Key::CloseBracket,
        "Enter" => Key::Enter,
        "ControlLeft" => Key::Control(Location::Left),
        "ControlRight" => Key::Control(Location::Right),
        "Semicolon" => Key::Semicolon,
        "Quote" => Key::Quote,
        "Backquote" => Key::Backtick,
        "ShiftLeft" => Key::Shift(Location::Left),
        "ShiftRight" => Key::Shift(Location::Right),
        "Comma" => Key::Comma,
        "Period" => Key::Period,
        "Slash" => Key::Slash,
        "NumpadMultiply" => Key::Numpad(Numpad::Multiply),
        "AltLeft" => Key::Alt(Location::Left),
        "AltRight" => Key::Alt(Location::Right),
        "MetaLeft" => Key::Meta(Location::Left),
        "MetaRight" => Key::Meta(Location::Right),
        "Space" => Key::Space,
        "CapsLock" => Key::CapsLock,
        "F1" => Key::Function(Function::F1),
        "F2" => Key::Function(Function::F2),
        "F3" => Key::Function(Function::F3),
        "F4" => Key::Function(Function::F4),
        "F5" => Key::Function(Function::F5),
        "F6" => Key::Function(Function::F6),
        "F7" => Key::Function(Function::F7),
        "F8" => Key::Function(Function::F8),
        "F9" => Key::Function(Function::F9),
        "F10" => Key::Function(Function::F10),
        "F11" => Key::Function(Function::F11),
        "F12" => Key::Function(Function::F12),
        "F13" => Key::Function(Function::F13),
        "F14" => Key::Function(Function::F14),
        "F15" => Key::Function(Function::F15),
        "F16" => Key::Function(Function::F16),
        "F17" => Key::Function(Function::F17),
        "F18" => Key::Function(Function::F18),
        "F19" => Key::Function(Function::F19),
        "F20" => Key::Function(Function::F20),
        "F21" => Key::Function(Function::F21),
        "F22" => Key::Function(Function::F22),
        "F23" => Key::Function(Function::F23),
        "F24" => Key::Function(Function::F24),
        "Pause" => Key::Pause,
        "ScrollLock" => Key::ScrollLock,
        "Numpad0" => Key::Numpad(Numpad::Number(Number::Zero)),
        "Numpad1" => Key::Numpad(Numpad::Number(Number::One)),
        "Numpad2" => Key::Numpad(Numpad::Number(Number::Two)),
        "Numpad3" => Key::Numpad(Numpad::Number(Number::Three)),
        "Numpad4" => Key::Numpad(Numpad::Number(Number::Four)),
        "Numpad5" => Key::Numpad(Numpad::Number(Number::Five)),
        "Numpad6" => Key::Numpad(Numpad::Number(Number::Six)),
        "Numpad7" => Key::Numpad(Numpad::Number(Number::Seven)),
        "Numpad8" => Key::Numpad(Numpad::Number(Number::Eight)),
        "Numpad9" => Key::Numpad(Numpad::Number(Number::Nine)),
        "NumpadAdd" => Key::Numpad(Numpad::Add),
        "NumpadDecimal" => Key::Numpad(Numpad::Decimal),
        "PrintScreen" => Key::PrintScreen,
        "NumpadEqual" => Key::Numpad(Numpad::Equal),
        "NumpadEnter" => Key::Numpad(Numpad::Enter),
        "NumLock" => Key::NumLock,
        "Home" => Key::Home,
        "Insert" => Key::Insert,
        "PageUp" => Key::PageUp,
        "PageDown" => Key::PageDown,
        "ArrowUp" => Key::Arrow(Arrow::Up),
        "ArrowDown" => Key::Arrow(Arrow::Down),
        "ArrowLeft" => Key::Arrow(Arrow::Left),
        "ArrowRight" => Key::Arrow(Arrow::Right),
        "End" => Key::End,
        "Delete" => Key::Delete,
        "ContextMenu" => Key::ContextMenu,
        "Backslash" => Key::Backslash,
        _ => Key::Unknown,
    }
}

pub(crate) struct KeyboardState {
    handlers: Vec<Box<dyn Fn(Event)>>,
    keys: HashMap<Key, bool>,
}

pub(crate) struct Keyboard {
    state: Rc<RefCell<KeyboardState>>,
}

impl interaction::Source for Keyboard {
    type Event = Event;
    fn bind(&self, handler: Box<dyn Fn(Event) + 'static + Sync + Send>) {
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
    #[allow(clippy::new_ret_no_self)]
    pub(crate) fn new() -> Box<dyn interaction::Keyboard> {
        let keyboard = Keyboard {
            state: Rc::new(RefCell::new(KeyboardState {
                handlers: vec![],
                keys: HashMap::new(),
            })),
        };
        keyboard.initialize();
        Box::new(keyboard)
    }
    fn initialize(&self) {
        let state = self.state.clone();
        let up_state = state.clone();
        let body = document().body().unwrap();
        body.add_event_listener(move |e: KeyDownEvent| {
            let send_state = state.clone();
            let mut state = state.borrow_mut();
            e.prevent_default();
            let key = e.key();
            let k = parse_code(e.code().as_str());
            let entry = state.keys.entry(k).or_insert(true);
            *entry = true;
            let event = Event {
                action: Action::Down(k),
                state: send_state,
                printable: if key.len() == 1 {
                    Some(key.chars().take(1).collect::<Vec<char>>()[0])
                } else {
                    None
                },
            };
            state.handlers.iter().for_each(|handler| {
                handler(event.clone());
            });
        });
        body.add_event_listener(move |e: KeyUpEvent| {
            let send_state = up_state.clone();
            let mut state = up_state.borrow_mut();
            e.prevent_default();
            let key = e.key();
            let k = parse_code(e.code().as_str());
            let entry = state.keys.entry(k).or_insert(false);
            *entry = false;
            let event = Event {
                action: Action::Up(k),
                state: send_state,
                printable: if key.len() == 1 {
                    Some(key.chars().take(1).collect::<Vec<char>>()[0])
                } else {
                    None
                },
            };
            state.handlers.iter().for_each(|handler| {
                handler(event.clone());
            });
        });
    }
}
