use crate::interaction;
use crate::interaction::windowing::{Action, Event};

use std::cell::RefCell;
use std::rc::Rc;

use stdweb::web::event::ResizeEvent;
use stdweb::web::{document, window, IEventTarget, INode, IParentNode};

pub(crate) struct WindowState {
    handlers: Vec<Box<dyn Fn(Event)>>,
}

pub(crate) struct Window {
    state: Rc<RefCell<WindowState>>,
}

impl Window {
    #[allow(clippy::new_ret_no_self)]
    pub(crate) fn new() -> Box<dyn interaction::Window> {
        let window = Window {
            state: Rc::new(RefCell::new(WindowState { handlers: vec![] })),
        };
        window.initialize();
        Box::new(window)
    }
    fn initialize(&self) {
        let state = self.state.clone();
        window().add_event_listener(move |_: ResizeEvent| {
            let state = state.borrow();
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
    fn bind(&self, handler: Box<dyn Fn(Event) + 'static + Sync + Send>) {
        self.state.borrow_mut().handlers.push(handler);
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
