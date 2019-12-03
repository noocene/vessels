use crate::{
    kind::{Future, Stream},
    object,
    replicate::{Reactive, Share},
    Kind,
};

use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    future::ready,
};

#[object]
pub trait List<T: Kind + Share> {
    fn len(&self) -> Future<u32>;
    fn is_empty(&self) -> Future<bool>;
    fn index(&self, index: u32) -> Future<Option<T>>;
    fn push(&mut self, item: T) -> Future<()>;
    fn pop(&mut self) -> Future<Option<T>>;
    fn remove(&mut self, index: u32) -> Future<bool>;
}

impl<T: Kind + Share> List<T> for Vec<T> {
    fn len(&self) -> Future<u32> {
        Box::pin(ready(self.len() as u32))
    }
    fn is_empty(&self) -> Future<bool> {
        Box::pin(ready(self.is_empty()))
    }
    fn index(&self, index: u32) -> Future<Option<T>> {
        Box::pin(ready(self.get(index as usize).map(Share::share)))
    }
    fn push(&mut self, item: T) -> Future<()> {
        self.push(item);
        Box::pin(ready(()))
    }
    fn pop(&mut self) -> Future<Option<T>> {
        Box::pin(ready(self.pop()))
    }
    fn remove(&mut self, index: u32) -> Future<bool> {
        let removed;
        let index = index as usize;
        if index < self.len() {
            removed = true;
            self.remove(index);
        } else {
            removed = false;
        }
        Box::pin(ready(removed))
    }
}

#[derive(Kind)]
pub enum Mutation<T: Kind + Share> {
    Pop,
    Push(T),
    Remove(u32),
}

impl<T: Kind + Share> Share for Mutation<T> {
    fn share(&self) -> Self {
        match self {
            Mutation::Pop => Mutation::Pop,
            Mutation::Push(item) => Mutation::Push(item.share()),
            Mutation::Remove(index) => Mutation::Remove(*index),
        }
    }
}

struct Observed<T: Kind + Share>(Box<dyn List<T>>, UnboundedSender<Mutation<T>>);

impl<T: Kind + Share> Observed<T> {
    fn new(item: Box<dyn List<T>>) -> (Self, Stream<Mutation<T>>) {
        let (sender, receiver) = unbounded();
        (Observed(item, sender), Box::pin(receiver))
    }
}

impl<T: Kind + Share> List<T> for Observed<T> {
    fn len(&self) -> Future<u32> {
        self.0.len()
    }
    fn is_empty(&self) -> Future<bool> {
        self.0.is_empty()
    }
    fn index(&self, index: u32) -> Future<Option<T>> {
        self.0.index(index)
    }
    fn push(&mut self, item: T) -> Future<()> {
        self.1.start_send(Mutation::Push(item.share())).unwrap();
        self.0.push(item)
    }
    fn pop(&mut self) -> Future<Option<T>> {
        self.1.start_send(Mutation::Pop).unwrap();
        self.0.pop()
    }
    fn remove(&mut self, index: u32) -> Future<bool> {
        self.1.start_send(Mutation::Remove(index)).unwrap();
        self.0.remove(index)
    }
}

impl<T: Kind + Share> Reactive for Box<dyn List<T>> {
    type Mutation = Mutation<T>;

    fn react(self) -> (Self, Stream<Self::Mutation>) {
        let (item, stream) = Observed::new(self);
        (Box::new(item), stream)
    }
}
