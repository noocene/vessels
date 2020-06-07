use core::cell::{Cell, RefCell};
use std::{
    collections::VecDeque,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

pub static ACTIVE_TASKS: AtomicUsize = AtomicUsize::new(0);

extern "C" {
    fn _vessel_wake();
}

#[no_mangle]
pub extern "C" fn _vessel_poll() -> usize {
    QUEUE.with(|queue| {
        queue.run_all();
        ACTIVE_TASKS.load(Ordering::SeqCst)
    })
}

struct QueueState {
    tasks: RefCell<VecDeque<Rc<super::task::Task>>>,
    is_spinning: Cell<bool>,
}

impl QueueState {
    fn run_all(&self) {
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
            unsafe { _vessel_wake() };
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
