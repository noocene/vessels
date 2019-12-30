use alloc::{collections::VecDeque, rc::Rc};
use core::cell::{Cell, RefCell};
#[cfg(feature = "core")]
use js_sys::Promise;
#[cfg(feature = "core")]
use wasm_bindgen::{closure::Closure, JsValue};

#[cfg(not(feature = "core"))]
extern "C" {
    fn _EXPORT_enqueue();
}

#[cfg(not(feature = "core"))]
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
    #[cfg(feature = "core")]
    promise: Promise,
    #[cfg(feature = "core")]
    closure: Closure<dyn FnMut(JsValue)>,
}

impl Queue {
    pub(crate) fn push_task(&self, task: Rc<super::task::Task>) {
        self.state.tasks.borrow_mut().push_back(task);
        if !self.state.is_spinning.replace(true) {
            #[cfg(not(feature = "core"))]
            unsafe {
                _EXPORT_enqueue()
            };
            #[cfg(feature = "core")]
            self.promise.then(&self.closure);
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

        Queue {
            #[cfg(feature = "core")]
            promise: Promise::resolve(&JsValue::undefined()),
            #[cfg(feature = "core")]
            closure: {
                let state = Rc::clone(&state);
                Closure::wrap(Box::new(move |_| state.run_all()))
            },
            state,
        }
    }
}

thread_local! {
    pub(crate) static QUEUE: Queue = Queue::new();
}
