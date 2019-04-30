use crate::interaction;
use crate::interaction::windowing::Event;

pub(crate) struct Window {}

impl interaction::Source for Window {
    type Event = Event;
    fn bind(&self, handler: Box<dyn Fn(Self::Event) + 'static>) {}
}

impl interaction::Window for Window {
    fn set_title(&mut self, title: &'_ str) {}
}

impl Window {
    pub(crate) fn new() -> Box<dyn interaction::Window> {
        Box::new(Window {})
    }
}
