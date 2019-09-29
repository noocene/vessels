use crate::input::{
    keyboard::{self as keyboard_mod, Event as KeyboardEvent},
    mouse::{self, Event as MouseEvent},
    windowing::Event as WindowingEvent,
    Event, Input as IInput,
};
use crossbeam_channel::{unbounded, Receiver, TryRecvError};
use futures::{task::AtomicTask, Async, Poll, Stream};
use std::sync::Arc;

use stdweb::traits::{IEvent, IEventTarget, IKeyboardEvent};
use stdweb::web::{
    document,
    event::{
        IMouseEvent, KeyDownEvent, KeyUpEvent, MouseButton, MouseDownEvent, MouseMoveEvent,
        MouseUpEvent, MouseWheelEvent, ResizeEvent,
    },
    window,
};

mod keyboard;

#[derive(Clone)]
pub(crate) struct Input {
    receiver: Receiver<Event>,
    task: Arc<AtomicTask>,
}

impl Stream for Input {
    type Item = Event;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.receiver.try_recv() {
            Ok(event) => Ok(Async::Ready(Some(event))),
            Err(err) => match err {
                TryRecvError::Disconnected => panic!("Input channel disconnected!"),
                TryRecvError::Empty => {
                    self.task.register();
                    Ok(Async::NotReady)
                }
            },
        }
    }
}

impl IInput for Input {
    fn box_clone(&self) -> Box<dyn IInput> {
        Box::new(self.clone())
    }
}

impl Input {
    pub(crate) fn new() -> Box<dyn IInput> {
        let (sender, receiver) = unbounded();
        let task = Arc::new(AtomicTask::new());
        let (resize_sender, resize_task) = (sender.clone(), task.clone());
        window().add_event_listener(move |_: ResizeEvent| {
            resize_sender.send(Event::Windowing(WindowingEvent::Resize));
            resize_task.notify();
        });
        let body = document().body().unwrap();
        let (mouse_up_sender, mouse_up_task) = (sender.clone(), task.clone());
        body.add_event_listener(move |event: MouseUpEvent| {
            event.prevent_default();
            mouse_up_sender.send(Event::Mouse(MouseEvent::Up(match event.button() {
                MouseButton::Left => mouse::Button::Left,
                MouseButton::Right => mouse::Button::Right,
                MouseButton::Wheel => mouse::Button::Middle,
                MouseButton::Button4 => mouse::Button::Auxiliary(0),
                MouseButton::Button5 => mouse::Button::Auxiliary(1),
            })));
            mouse_up_task.notify();
        });
        let (mouse_down_sender, mouse_down_task) = (sender.clone(), task.clone());
        body.add_event_listener(move |event: MouseDownEvent| {
            event.prevent_default();
            mouse_down_sender.send(Event::Mouse(MouseEvent::Down(match event.button() {
                MouseButton::Left => mouse::Button::Left,
                MouseButton::Right => mouse::Button::Right,
                MouseButton::Wheel => mouse::Button::Middle,
                MouseButton::Button4 => mouse::Button::Auxiliary(0),
                MouseButton::Button5 => mouse::Button::Auxiliary(1),
            })));
            mouse_down_task.notify();
        });
        let (mouse_move_sender, mouse_move_task) = (sender.clone(), task.clone());
        body.add_event_listener(move |event: MouseMoveEvent| {
            event.prevent_default();

            mouse_move_sender.send(Event::Mouse(MouseEvent::Move(
                (f64::from(event.movement_x()), f64::from(event.movement_y())).into(),
            )));
            mouse_move_task.notify();
        });
        let (mouse_wheel_sender, mouse_wheel_task) = (sender.clone(), task.clone());
        body.add_event_listener(move |event: MouseWheelEvent| {
            mouse_wheel_sender.send(Event::Mouse(MouseEvent::Scroll(
                (event.delta_x(), event.delta_y()).into(),
            )));
            mouse_wheel_task.notify();
        });
        let (key_down_sender, key_down_task) = (sender.clone(), task.clone());
        body.add_event_listener(move |e: KeyDownEvent| {
            e.prevent_default();
            let key = e.key();
            let k = keyboard::parse_code(e.code().as_str());
            key_down_sender.send(Event::Keyboard(KeyboardEvent {
                action: keyboard_mod::Action::Down(k),
                printable: if key.len() == 1 {
                    Some(key.chars().take(1).collect::<Vec<char>>()[0])
                } else {
                    None
                },
            }));
            key_down_task.notify();
        });
        let (key_up_sender, key_up_task) = (sender.clone(), task.clone());
        body.add_event_listener(move |e: KeyUpEvent| {
            e.prevent_default();
            let key = e.key();
            let k = keyboard::parse_code(e.code().as_str());
            key_up_sender.send(Event::Keyboard(KeyboardEvent {
                action: keyboard_mod::Action::Up(k),
                printable: if key.len() == 1 {
                    Some(key.chars().take(1).collect::<Vec<char>>()[0])
                } else {
                    None
                },
            }));
            key_up_task.notify();
        });
        Box::new(Input { receiver, task })
    }
}
