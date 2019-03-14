use crate::input;
use crate::input::mouse;
use crate::input::mouse::{Button, Event};

use stdweb::web::event::*;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::IEventTarget;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Mouse {
    target: CanvasElement,
}

impl input::Source<mouse::Event> for Mouse {
    fn bind<F>(&self, handler: F)
    where
        F: Fn(input::mouse::Event) + 'static,
    {
        let down_handler = Rc::new(RefCell::new(handler));
        let up_handler = down_handler.clone();
        let move_handler = down_handler.clone();
        self.target
            .add_event_listener(move |event: MouseDownEvent| {
                event.prevent_default();
                down_handler.borrow()(Event::Down(match event.button() {
                    MouseButton::Left => Button::Left,
                    MouseButton::Right => Button::Right,
                    MouseButton::Wheel => Button::Middle,
                    MouseButton::Button4 => Button::Auxiliary(0),
                    MouseButton::Button5 => Button::Auxiliary(1),
                }));
            });
        self.target.add_event_listener(move |event: MouseUpEvent| {
            event.prevent_default();
            up_handler.borrow()(Event::Up(match event.button() {
                MouseButton::Left => Button::Left,
                MouseButton::Right => Button::Right,
                MouseButton::Wheel => Button::Middle,
                MouseButton::Button4 => Button::Auxiliary(0),
                MouseButton::Button5 => Button::Auxiliary(1),
            }));
        });
        self.target
            .add_event_listener(move |event: MouseMoveEvent| {
                event.prevent_default();
                move_handler.borrow()(Event::Move((f64::from(event.movement_x()), f64::from(event.movement_y())).into())));
    }
}

impl input::Mouse for Mouse {}

impl Mouse {
    pub fn new(target: CanvasElement) -> Mouse {
        Mouse { target }
    }
}
