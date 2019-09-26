use crate::graphics::Vector;
use crate::interaction;
use crate::interaction::mouse::{Action, Button, Event};

use std::sync::{Arc, RwLock};

pub(crate) struct MouseState {
    handlers: Vec<Box<dyn Fn(Event) + Send + Sync>>,
    position: Vector,
}

#[derive(Clone)]
pub(crate) struct Mouse {
    state: Arc<RwLock<MouseState>>,
}

impl interaction::Source for Mouse {
    type Event = Event;
    fn bind(&self, handler: Box<dyn Fn(Self::Event) + 'static + Sync + Send>) {
        self.state.write().unwrap().handlers.push(handler);
    }
}

impl interaction::Mouse for Mouse {
    fn position(&self) -> Vector {
        self.state.read().unwrap().position
    }
}

impl Mouse {
    #[allow(clippy::new_ret_no_self)]
    pub(crate) fn new(
        event_handler: Box<dyn interaction::Source<Event = glutin::Event>>,
    ) -> Box<dyn interaction::Mouse> {
        let mouse = Mouse {
            state: Arc::new(RwLock::new(MouseState {
                handlers: vec![],
                position: Vector::default(),
            })),
        };
        mouse.initialize(event_handler);
        Box::new(mouse)
    }
    fn initialize(&self, event_handler: Box<dyn interaction::Source<Event = glutin::Event>>) {
        let state = self.state.clone();
        event_handler.bind(Box::new(move |event: glutin::Event| {
            let my_state = state.clone();
            let mut state = my_state.write().unwrap();
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::CursorMoved { position, .. } => {
                        let movement: Vector =
                            Vector::from((position.x, position.y)) - state.position;
                        state.position = (position.x, position.y).into();
                        state.handlers.iter().for_each(|handler| {
                            handler(Event {
                                action: Action::Move(movement),
                                position: state.position,
                            })
                        })
                    }
                    glutin::WindowEvent::CursorEntered { .. } => (),
                    glutin::WindowEvent::CursorLeft { .. } => (),
                    glutin::WindowEvent::MouseInput {
                        state: element_state,
                        button,
                        ..
                    } => state.handlers.iter().for_each(|handler| {
                        handler(Event {
                            action: match element_state {
                                glutin::ElementState::Pressed => Action::Down(match button {
                                    glutin::MouseButton::Left => Button::Left,
                                    glutin::MouseButton::Right => Button::Right,
                                    glutin::MouseButton::Middle => Button::Middle,
                                    glutin::MouseButton::Other(x) => Button::Auxiliary(x),
                                }),
                                glutin::ElementState::Released => Action::Up(match button {
                                    glutin::MouseButton::Left => Button::Left,
                                    glutin::MouseButton::Right => Button::Right,
                                    glutin::MouseButton::Middle => Button::Middle,
                                    glutin::MouseButton::Other(x) => Button::Auxiliary(x),
                                }),
                            },
                            position: state.position,
                        })
                    }),
                    glutin::WindowEvent::MouseWheel { delta, .. } => {
                        let pixel_delta: Vector = match delta {
                            glutin::MouseScrollDelta::LineDelta(_x, _y) => {
                                println!("LineDelta is not handled");
                                (0., 0.).into()
                            }
                            glutin::MouseScrollDelta::PixelDelta(p) => (p.x, p.y).into(),
                        };
                        state.handlers.iter().for_each(|handler| {
                            handler(Event {
                                action: Action::Scroll(pixel_delta),
                                position: state.position,
                            })
                        })
                    }
                    _ => (),
                }
            }
        }));
    }
}
