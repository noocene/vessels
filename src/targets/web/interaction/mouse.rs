use crate::graphics_2d::Vector;
use crate::interaction;
use crate::interaction::mouse::{Action, Button, Event};

use stdweb::web::event::{
    IEvent, IMouseEvent, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, MouseWheelEvent,
};
use stdweb::web::{document, IEventTarget};

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
    fn bind(&self, handler: Box<dyn Fn(Event) + 'static>) {
        self.state.borrow_mut().handlers.push(handler);
    }
}

impl interaction::Mouse for Mouse {
    fn position(&self) -> Vector {
        self.state.borrow().position
    }
}

impl Mouse {
    #[allow(clippy::new_ret_no_self)]
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
    fn initialize(&self) {
        let state = self.state.clone();
        let up_state = state.clone();
        let move_state = up_state.clone();
        let scroll_state = move_state.clone();
        let body = document().body().unwrap();
        body.add_event_listener(move |event: MouseDownEvent| {
            event.prevent_default();
            let state = state.borrow();
            state.handlers.iter().for_each(|handler| {
                handler(Event {
                    action: Action::Down(match event.button() {
                        MouseButton::Left => Button::Left,
                        MouseButton::Right => Button::Right,
                        MouseButton::Wheel => Button::Middle,
                        MouseButton::Button4 => Button::Auxiliary(0),
                        MouseButton::Button5 => Button::Auxiliary(1),
                    }),
                    position: state.position,
                })
            });
        });
        body.add_event_listener(move |event: MouseUpEvent| {
            event.prevent_default();
            let state = up_state.borrow();
            state.handlers.iter().for_each(|handler| {
                handler(Event {
                    action: Action::Up(match event.button() {
                        MouseButton::Left => Button::Left,
                        MouseButton::Right => Button::Right,
                        MouseButton::Wheel => Button::Middle,
                        MouseButton::Button4 => Button::Auxiliary(0),
                        MouseButton::Button5 => Button::Auxiliary(1),
                    }),
                    position: state.position,
                })
            });
        });
        body.add_event_listener(move |event: MouseMoveEvent| {
            let mut state = move_state.borrow_mut();
            event.prevent_default();
            state.position = (f64::from(event.client_x()), f64::from(event.client_y())).into();
            state.handlers.iter().for_each(|handler| {
                handler(Event {
                    action: Action::Move(
                        (f64::from(event.movement_x()), f64::from(event.movement_y())).into(),
                    ),
                    position: state.position,
                })
            })
        });
        body.add_event_listener(move |event: MouseWheelEvent| {
            let state = scroll_state.borrow();
            event.prevent_default();
            state.handlers.iter().for_each(|handler| {
                handler(Event {
                    action: Action::Scroll((event.delta_x(), event.delta_y()).into()),
                    position: state.position,
                })
            })
        });
    }
}
