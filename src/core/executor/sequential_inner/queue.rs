use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;

extern "C" {
    fn _EXPORT_enqueue();
}

#[no_mangle]
pub extern "C" fn _EXPORT_handle() {
    QUEUE.with(|queue| queue.run_all())
}

struct QueueState {
    tasks: RefCell<VecDeque<Rc<super::task::Task>>>,
    is_spinning: Cell<bool>,
}

impl QueueState {
    fn run_all(&self) {
        debug_assert!(self.is_spinning.get());
        loop {
            let task = match self.tasks.borrow_mut().pop_front() {
                Some(task) => task,
                None => break,
            };
            task.run();
        }
        self.is_spinning.set(false);
    }
}

pub(crate) struct Queue {
    state: Rc<QueueState>,
}

impl Queue {
    pub(crate) fn push_task(&self, task: Rc<super::task::Task>) {
        self.state.tasks.borrow_mut().push_back(task);
        if !self.state.is_spinning.replace(true) {
            unsafe { _EXPORT_enqueue() };
        }
    }
    fn run_all(&self) {
        self.state.run_all()
    }
}

impl Queue {
    fn new() -> Self {
        let state = Rc::new(QueueState {
            is_spinning: Cell::new(false),
            tasks: RefCell::new(VecDeque::new()),
        });

        Queue { state }
    }
}

thread_local! {
    pub(crate) static QUEUE: Queue = Queue::new();
}
