use crate::{kind::Future, object, replicate::Share, Kind};
use futures::future::ready;

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
