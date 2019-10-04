use crate::executor;
use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};
use futures::Future as Fut;
use futures::{task::AtomicTask, Async, AsyncSink, IntoFuture, Poll, Sink, StartSend, Stream};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

pub use vessels_derive::{protocol, Value};

/// The core context type used for communication by most Value implementations.
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
    /// Creates a new pair of linked contexts.
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

impl<T: Serialize + DeserializeOwned> Sink for Context<T> {
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

impl<T: Serialize + DeserializeOwned> Stream for Context<T> {
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

/// A value that can be constructed/deconstructed into a stream of serializable items for use with protocol traits.
pub trait Value {
    /// The item type sent over the internal channel by this value.
    type Item: Serialize + DeserializeOwned + Send + 'static;

    /// Constructs a value of this type from the provided context. This method may block for indefinite time until the value is ready (i.e. with concrete or derived Value types) or return immediately (i.e. with Futures etc.)
    fn construct<
        C: Sink<SinkItem = Self::Item, SinkError = ()>
            + Stream<Item = Self::Item, Error = ()>
            + Send
            + 'static,
    >(
        context: C,
    ) -> Self
    where
        Self: Sized;
    /// Deconstructs a value of this type onto the provided context.
    fn deconstruct<
        C: Sink<SinkItem = Self::Item, SinkError = ()>
            + Stream<Item = Self::Item, Error = ()>
            + Send
            + 'static,
    >(
        self,
        context: C,
    ) where
        Self: Sized;
}

/// An abstracted future that allows the remote protocol abstraction to return immediately from a method call and preserve asynchronous operation. Recommended for most return values.
pub struct Future<T: Value, E: Value> {
    future: Box<dyn futures::Future<Item = T, Error = E> + Send + 'static>,
}

impl<T: Value, E: Value> Future<T, E> {
    /// Creates a new abstracted future. `Result` and `Option` implement IntoFuture making this construction simple for concrete values.
    pub fn new<F: IntoFuture<Item = T, Error = E> + Send + 'static>(future: F) -> Self
    where
        F::Future: Send,
    {
        Future {
            future: Box::new(future.into_future()),
        }
    }
}

struct SinkStream<T, U>(T, U);

impl<T, E, S, U> Sink for SinkStream<S, U>
where
    S: Sink<SinkItem = T, SinkError = E>,
{
    type SinkItem = T;
    type SinkError = E;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.0.start_send(item)
    }
    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.0.poll_complete()
    }
}

impl<T, E, S, U> Stream for SinkStream<S, U>
where
    U: Stream<Item = T, Error = E>,
{
    type Item = T;
    type Error = E;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.1.poll()
    }
}

impl<T: Value + Send + 'static, E: Value + Send + 'static> Value for Future<T, E> {
    type Item = Result<T::Item, E::Item>;

    fn construct<
        C: Sink<SinkItem = Self::Item, SinkError = ()>
            + Stream<Item = Self::Item, Error = ()>
            + Send
            + 'static,
    >(
        context: C,
    ) -> Self
    where
        Self: Sized,
    {
        let (sender, receiver) = unbounded();
        executor::spawn(context.into_future().map_err(|_| ()).and_then(move |v| {
            let result = v.0.unwrap();
            match result {
                Ok(value) => {
                    let (sink, stream) = v.1.split();
                    let stream =
                        futures::stream::once(Ok(value)).chain(stream.map(|data| match data {
                            Ok(data) => data,
                            Err(_e) => panic!("Invalid content in protocol future stream"),
                        }));
                    let sink = sink.with(|data: T::Item| Ok(Ok(data)));
                    sender
                        .send(Ok(T::construct(SinkStream(sink, stream))))
                        .unwrap();
                }
                Err(value) => {
                    let (sink, stream) = v.1.split();
                    let stream =
                        futures::stream::once(Ok(value)).chain(stream.map(|data| match data {
                            Err(data) => data,
                            Ok(_) => panic!("Invalid content in protocol future stream"),
                        }));
                    let sink = sink.with(|data: E::Item| Ok(Err(data)));
                    sender
                        .send(Err(E::construct(SinkStream(sink, stream))))
                        .unwrap();
                }
            };
            Ok(())
        }));
        Future {
            future: Box::new(
                futures::stream::iter_result(receiver.into_iter())
                    .take(1)
                    .into_future()
                    .map_err(|v| v.0)
                    .map(|v| v.0.unwrap()),
            ),
        }
    }

    fn deconstruct<
        C: Sink<SinkItem = Self::Item, SinkError = ()>
            + Stream<Item = Self::Item, Error = ()>
            + Send
            + 'static,
    >(
        self,
        context: C,
    ) where
        Self: Sized,
    {
        executor::spawn(self.future.then(|result| {
            match result {
                Ok(value) => {
                    let ctx = context
                        .map(|item| {
                            if let Ok(item) = item {
                                item
                            } else {
                                panic!("Invalid result in future stream");
                            }
                        })
                        .with(|ok| Ok(Ok(ok)));
                    value.deconstruct(ctx);
                }
                Err(value) => {
                    let ctx = context
                        .map(|item| {
                            if let Err(item) = item {
                                item
                            } else {
                                panic!("Invalid result in future stream");
                            }
                        })
                        .with(|err| Ok(Err(err)));
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

    fn construct<
        C: Sink<SinkItem = Self::Item, SinkError = ()>
            + Stream<Item = Self::Item, Error = ()>
            + Send
            + 'static,
    >(
        context: C,
    ) -> Self {
        if let Ok(v) = context.into_future().wait() {
            v.0.unwrap()
        } else {
            panic!("panic in construction");
        }
    }
    fn deconstruct<
        C: Sink<SinkItem = Self::Item, SinkError = ()>
            + Stream<Item = Self::Item, Error = ()>
            + Send
            + 'static,
    >(
        self,
        context: C,
    ) where
        Self: Sized,
    {
        executor::spawn(context.send(self).then(|_| Ok(())));
    }
}
