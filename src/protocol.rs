use crate::executor;
use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};
use futures::{task::AtomicTask, Async, AsyncSink, Poll, Sink, StartSend, Stream, IntoFuture};
use futures::Future as Fut;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

pub use vitruvia_derive::protocol;

#[derive(Clone, Debug)]
pub struct Context<T: Serialize + DeserializeOwned> {
    sender: Sender<T>,
    o_sender: Sender<T>,
    o_receiver: Receiver<T>,
    receiver: Receiver<T>,
    task: Arc<AtomicTask>,
    other_task: Arc<AtomicTask>,
}

impl<T: Serialize + DeserializeOwned> Context<T> {
    pub fn new() -> (Self, Self) {
        let (s0, r1) = unbounded();
        let (s1, r0) = unbounded();
        let task0 = Arc::new(AtomicTask::new());
        let task1 = Arc::new(AtomicTask::new());
        (
            Context {
                sender: s0.clone(),
                o_sender: s1.clone(),
                o_receiver: r1.clone(),
                receiver: r0.clone(),
                task: task0.clone(),
                other_task: task1.clone(),
            },
            Context {
                sender: s1,
                o_sender: s0,
                o_receiver: r0,
                receiver: r1,
                task: task1,
                other_task: task0,
            },
        )
    }
}

impl<T: Serialize + DeserializeOwned> futures::Sink for Context<T> {
    type SinkError = ();
    type SinkItem = T;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.sender.send(item).unwrap();
        self.other_task.notify();
        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Ok(Async::Ready(()))
    }
}

impl<T: Serialize + DeserializeOwned> futures::Stream for Context<T> {
    type Error = ();
    type Item = T;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.receiver.try_recv() {
            Ok(data) => Ok(Async::Ready(Some(data))),
            Err(err) => match err {
                TryRecvError::Disconnected => panic!("Context channel disconnected!"),
                TryRecvError::Empty => {
                    self.task.register();
                    Ok(Async::NotReady)
                }
            },
        }
    }
}

pub trait Value {
    type Item: Serialize + DeserializeOwned + Send + 'static;

    fn construct<C: Sink<SinkItem = Self::Item, SinkError = ()> + Stream<Item = Self::Item, Error = ()> + Send + 'static>(context: C) -> Self
    where
        Self: Sized;
    fn deconstruct<C: Sink<SinkItem = Self::Item, SinkError = ()> + Stream<Item = Self::Item, Error = ()> + Send + 'static>(self, context: C)
    where
        Self: Sized;
}

pub struct Future<T: Value, E: Value> {
    future: Box<dyn futures::Future<Item = T, Error = E> + Send + 'static>,
}

impl<T: Value, E: Value> Future<T, E> {
    fn new<F: IntoFuture<Item = T, Error = E> + Send + 'static>(future: F) -> Self where F::Future: Send {
        Future {
            future: Box::new(future.into_future()),
        }
    }
}

fn future_with_ok<T: Value, E: Value>(ok: T::Item) -> Result<Result<T::Item, E::Item>, ()> {
    Ok(Ok(ok))
}

fn future_with_err<T: Value, E: Value>(item: E::Item) -> Result<Result<T::Item, E::Item>, ()> {
    Ok(Err(item))
}

impl<T: Value + Send + 'static, E: Value + Send + 'static> Value for Future<T, E> {
    type Item = Result<T::Item, E::Item>;

    fn construct<C: Sink<SinkItem = Self::Item, SinkError = ()> + Stream<Item = Self::Item, Error = ()> + Send + 'static>(context: C) -> Self where Self: Sized {
        executor::spawn(context.for_each(|v| {println!("{}", serde_json::to_string(&v).unwrap()); Ok(())}));
        Future {
            future: Box::new(futures::future::done(Ok(T::construct(Context::new().0)))),
        }
    }

    fn deconstruct<C: Sink<SinkItem = Self::Item, SinkError = ()> + Stream<Item = Self::Item, Error = ()> + Send + 'static>(self, context: C) where Self: Sized {
        executor::spawn(self.future.then(|result| {
            match result {
                Ok(value) => {
                    let ctx = context.map(|item| {
                        if let Ok(item) = item {
                            return item;
                        } else {
                            panic!("Invalid result in future stream");
                        }
                    }).with(future_with_ok::<T, E>);
                    value.deconstruct(ctx);
                }
                Err(value) => {
                    let ctx = context.map(|item| {
                        if let Err(item) = item {
                            return item;
                        } else {
                            panic!("Invalid result in future stream");
                        }
                    }).with(future_with_err::<T, E>);
                    value.deconstruct(ctx);
                }
            }
            Ok(())
        }))
    }
}

impl<T: Value, E: Value> futures::Future for Future<T, E> {
    type Item = T;
    type Error = E;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.future.poll()
    }
}

/*pub struct Object<T: ?Sized> {
    inner: Box<T>,
}

impl<T: ?Sized> Value for Object<T> {
    type Item = ();
}*/

impl<T> Value for T
where
    T: Serialize + DeserializeOwned + Send + 'static,
{
    type Item = T;

    fn construct<C: Sink<SinkItem = Self::Item, SinkError = ()> + Stream<Item = Self::Item, Error = ()> + Send + 'static>(context: C) -> Self {
        if let Ok(v) = context.into_future().wait() {
            return v.0.unwrap();
        } else {
            panic!("panic in construction");
        }
    }
    fn deconstruct<C: Sink<SinkItem = Self::Item, SinkError = ()> + Stream<Item = Self::Item, Error = ()> + Send + 'static>(self, context: C)
    where
        Self: Sized,
    {
        executor::spawn(context.send(self).then(|_| Ok(())));
    }
}
