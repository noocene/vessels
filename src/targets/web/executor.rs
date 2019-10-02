use futures::executor::{self, Notify, Spawn};
use futures::future::{ExecuteError, Executor, Future};
use futures::Async;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::result::Result as StdResult;
use wasm_bindgen::{prelude::*, JsValue};

unsafe fn clone_raw<T>(ptr: *const T) -> Rc<T> {
    let result = Rc::from_raw(ptr);
    ::std::mem::forget(result.clone());
    result
}

type BoxedFuture = Box<dyn Future<Item = (), Error = ()> + 'static>;

struct SpawnedTask {
    is_queued: Cell<bool>,
    spawn: RefCell<Option<Spawn<BoxedFuture>>>,
}

impl SpawnedTask {
    fn new<F>(future: F) -> Rc<Self>
    where
        F: Future<Item = (), Error = ()> + 'static,
    {
        Rc::new(Self {
            is_queued: Cell::new(false),
            spawn: RefCell::new(Some(executor::spawn(Box::new(future) as BoxedFuture))),
        })
    }

    fn poll(&self) {
        let mut spawn = self.spawn.borrow_mut();
        if let Some(mut spawn_future) = spawn.take() {
            self.is_queued.set(false);
            if spawn_future.poll_future_notify(&&Core, self as *const _ as usize)
                == Ok(Async::NotReady)
            {
                *spawn = Some(spawn_future);
            }
        }
    }

    fn notify(spawned: Rc<SpawnedTask>) {
        if !spawned.is_queued.replace(true) {
            js_sys::Promise::resolve(&JsValue::NULL)
                .then(&Closure::wrap(
                    Box::new(move |_| spawned.poll()) as Box<dyn FnMut(JsValue)>
                ));
        }
    }
}

struct Core;

impl<F> Executor<F> for Core
where
    F: Future<Item = (), Error = ()> + Send + 'static,
{
    fn execute(&self, future: F) -> StdResult<(), ExecuteError<F>> {
        SpawnedTask::notify(SpawnedTask::new(future));
        Ok(())
    }
}

impl Notify for Core {
    fn notify(&self, spawned_id: usize) {
        SpawnedTask::notify(unsafe { clone_raw(spawned_id as *const _) })
    }

    fn clone_id(&self, id: usize) -> usize {
        unsafe { Rc::into_raw(clone_raw(id as *const SpawnedTask)) as usize }
    }

    fn drop_id(&self, id: usize) {
        unsafe { Rc::from_raw(id as *const SpawnedTask) };
    }
}

pub(crate) fn spawn<F>(future: F)
where
    F: Future<Item = (), Error = ()> + Send + 'static,
{
    Core.execute(future).unwrap();
}
