use crate::executor;
use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};
use futures::{task::AtomicTask, Async, AsyncSink, Future, Poll, Sink, StartSend, Stream};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

pub use vitruvia_derive::protocol;

#[derive(Clone)]
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

    fn construct(context: Context<Self::Item>) -> Option<Self>
    where
        Self: Sized,
    {
        executor::spawn(context.for_each(|i| {println!("i"); Ok(())}));
        None
    }
    fn deconstruct(self, context: Context<Self::Item>)
    where
        Self: Sized;
}

/*pub struct Future<T: Value, E: Value> {
    item: PhantomData<T>,
    error: PhantomData<E>,
}

impl<T: Value, E: Value> Value for Future<T, E> {
    type Item = ();
}

impl<T: Value, E: Value> futures::Future for Future<T, E> {
    type Item = T;
    type Error = E;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(Async::NotReady)
    }
}

pub struct Object<T: ?Sized> {
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

    fn deconstruct(self, context: Context<Self::Item>)
    where
        Self: Sized,
    {
        executor::spawn(context.send(self).then(|_| Ok(())));
    }
}
