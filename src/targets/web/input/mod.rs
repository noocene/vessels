use crate::input::{
    keyboard::{self as keyboard_mod, Event as KeyboardEvent},
    mouse::{self, Event as MouseEvent},
    windowing::Event as WindowingEvent,
    Event, Input as IInput,
};
use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};
use futures::{task::AtomicTask, Async, Poll, Stream};
use std::sync::Arc;

use wasm_bindgen::{prelude::*, JsCast};

mod keyboard;

#[derive(Clone)]
pub(crate) struct Input {
    receiver: Receiver<Event>,
    sender: Sender<Event>,
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
    pub(crate) fn send(&self, event: Event) {
        if Arc::strong_count(&self.task) != 1 {
            let _ = self.sender.send(event);
            self.task.notify();
        }
    }
    pub(crate) fn new() -> Input {
        let (sender, receiver) = unbounded();
        let task = Arc::new(AtomicTask::new());
        let (resize_sender, resize_task) = (sender.clone(), task.clone());
        let window = web_sys::window().unwrap();
        window.set_onresize(Some(
            Closure::wrap(Box::new(move || {
                if Arc::strong_count(&resize_task) == 1 {
                    return;
                }
                let _ = resize_sender.send(Event::Windowing(WindowingEvent::Resize));
                resize_task.notify();
            }) as Box<dyn Fn()>)
            .as_ref()
            .unchecked_ref(),
        ));
        let (mouse_up_sender, mouse_up_task) = (sender.clone(), task.clone());
        window.set_onmouseup(Some(
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                if Arc::strong_count(&mouse_up_task) == 1 {
                    return;
                }
                event.prevent_default();
                let _ = mouse_up_sender.send(Event::Mouse(MouseEvent::Up(match event.button() {
                    0 => mouse::Button::Left,
                    2 => mouse::Button::Right,
                    1 => mouse::Button::Middle,
                    button => mouse::Button::Auxiliary((button - 3) as u8),
                })));
                mouse_up_task.notify();
            }) as Box<dyn FnMut(_)>)
            .as_ref()
            .unchecked_ref(),
        ));
        let (mouse_down_sender, mouse_down_task) = (sender.clone(), task.clone());
        window.set_onmousedown(Some(
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                if Arc::strong_count(&mouse_down_task) == 1 {
                    return;
                }
                event.prevent_default();
                let _ =
                    mouse_down_sender.send(Event::Mouse(MouseEvent::Up(match event.button() {
                        0 => mouse::Button::Left,
                        2 => mouse::Button::Right,
                        1 => mouse::Button::Middle,
                        button => mouse::Button::Auxiliary((button - 3) as u8),
                    })));
                mouse_down_task.notify();
            }) as Box<dyn FnMut(_)>)
            .as_ref()
            .unchecked_ref(),
        ));
        let (mouse_move_sender, mouse_move_task) = (sender.clone(), task.clone());
        window.set_onmousemove(Some(
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                if Arc::strong_count(&mouse_move_task) == 1 {
                    return;
                }
                event.prevent_default();
                let _ = mouse_move_sender.send(Event::Mouse(MouseEvent::Move(
                    (f64::from(event.client_x()), f64::from(event.client_y())).into(),
                )));
                mouse_move_task.notify();
            }) as Box<dyn FnMut(_)>)
            .as_ref()
            .unchecked_ref(),
        ));
        let (mouse_wheel_sender, mouse_wheel_task) = (sender.clone(), task.clone());
        window.set_onwheel(Some(
            Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
                if Arc::strong_count(&mouse_wheel_task) == 1 {
                    return;
                }
                let _ = mouse_wheel_sender.send(Event::Mouse(MouseEvent::Scroll(
                    (event.delta_x(), event.delta_y()).into(),
                )));
                mouse_wheel_task.notify();
            }) as Box<dyn FnMut(_)>)
            .as_ref()
            .unchecked_ref(),
        ));
        let (key_down_sender, key_down_task) = (sender.clone(), task.clone());
        window.set_onkeydown(Some(
            Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                if Arc::strong_count(&key_down_task) == 1 {
                    return;
                }
                e.prevent_default();
                let key = e.key();
                let k = keyboard::parse_code(e.code().as_str());
                let _ = key_down_sender.send(Event::Keyboard(KeyboardEvent {
                    action: keyboard_mod::Action::Down(k),
                    printable: if key.len() == 1 {
                        Some(key.chars().take(1).collect::<Vec<char>>()[0])
                    } else {
                        None
                    },
                }));
                key_down_task.notify();
            }) as Box<dyn FnMut(_)>)
            .as_ref()
            .unchecked_ref(),
        ));
        let (key_up_sender, key_up_task) = (sender.clone(), task.clone());
        window.set_onkeyup(Some(
            Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                if Arc::strong_count(&key_up_task) == 1 {
                    return;
                }
                e.prevent_default();
                let key = e.key();
                let k = keyboard::parse_code(e.code().as_str());
                let _ = key_up_sender.send(Event::Keyboard(KeyboardEvent {
                    action: keyboard_mod::Action::Down(k),
                    printable: if key.len() == 1 {
                        Some(key.chars().take(1).collect::<Vec<char>>()[0])
                    } else {
                        None
                    },
                }));
                key_up_task.notify();
            }) as Box<dyn FnMut(_)>)
            .as_ref()
            .unchecked_ref(),
        ));
        Input {
            receiver,
            task,
            sender,
        }
    }
}
