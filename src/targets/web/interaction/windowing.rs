use crate::interaction;
use crate::interaction::windowing::{Action, Event};

use std::sync::{Arc, RwLock};

use stdweb::web::event::ResizeEvent;
use stdweb::web::{document, window, IEventTarget, INode, IParentNode};

pub(crate) struct WindowState {
    handlers: Vec<Box<dyn Fn(Event)>>,
}

#[derive(Clone)]
pub(crate) struct Window {
    state: Arc<RwLock<WindowState>>,
}

impl Window {
    #[allow(clippy::new_ret_no_self)]
    pub(crate) fn new() -> Box<dyn interaction::Window> {
        let window = Window {
            state: Arc::new(RwLock::new(WindowState { handlers: vec![] })),
        };
        window.initialize();
        Box::new(window)
    }
    fn initialize(&self) {
        let state = self.state.clone();
        window().add_event_listener(move |_: ResizeEvent| {
            let state = state.read().unwrap();
            state.handlers.iter().for_each(|handler| {
                handler(Event {
                    action: Action::Resize,
                })
            });
        });
    }
}

impl interaction::Source for Window {
    type Event = Event;
    fn bind(&self, handler: Box<dyn Fn(Event) + 'static + Send + Sync>) {
        self.state.write().unwrap().handlers.push(handler);
    }
    fn box_clone(&self) -> Box<dyn interaction::Source<Event=Event>> {
        Box::new(self.clone())
    }
}

impl interaction::Window for Window {
    fn set_title(&mut self, title: &'_ str) {
        document()
            .head()
            .unwrap()
            .query_selector("title")
            .unwrap()
            .unwrap()
            .set_text_content(title);
    }
}
