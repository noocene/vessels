use super::Context;

use crate::{channel::ForkHandle, Kind, SerdeAny};

use serde::{
    de::{self, DeserializeOwned, DeserializeSeed, Deserializer},
    Serialize,
};

use std::{
    any::TypeId,
    collections::HashMap,
    pin::Pin,
    sync::{Arc, Mutex, RwLock, Weak},
};

use futures::{
    executor::LocalPool,
    future::{ready, BoxFuture},
    task::{AtomicWaker, Context as FContext},
    Future, Poll,
};

use weak_table::WeakValueHashMap;

use lazy_static::lazy_static;

type DeserializeFn =
    fn(&mut dyn erased_serde::Deserializer<'_>) -> erased_serde::Result<Box<dyn SerdeAny>>;

pub(crate) struct Registry {
    items: RwLock<HashMap<TypeId, DeserializeFn>>,
    tasks: Mutex<WeakValueHashMap<TypeId, Weak<AtomicWaker>>>,
}

pub(crate) struct WaitFor {
    task: Arc<AtomicWaker>,
    registry: &'static Registry,
    id: TypeId,
}

impl Future for WaitFor {
    type Output = DeserializeFn;

    fn poll(self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Self::Output> {
        self.registry.get(self.id).map_or_else(
            || {
                self.task.register(cx.waker());
                Poll::Pending
            },
            Poll::Ready,
        )
    }
}

impl Registry {
    pub(crate) fn add_construct<K: Kind>(&self) {
        self.add_type::<K::ConstructItem>();
    }

    pub(crate) fn add_deconstruct<K: Kind>(&self) {
        self.add_type::<K::DeconstructItem>();
    }

    fn add_type<T: Serialize + DeserializeOwned + Send + 'static>(&self) {
        if !self.items.read().unwrap().contains_key(&TypeId::of::<T>()) {
            let mut items = self.items.write().unwrap();
            if !items.contains_key(&TypeId::of::<T>()) {
                items.insert(TypeId::of::<T>(), |de| {
                    <T as ::serde::Deserialize>::deserialize(de)
                        .map(|v| Box::new(v) as Box<dyn SerdeAny>)
                });
                if let Some(task) = self.tasks.lock().unwrap().get(&TypeId::of::<T>()) {
                    task.wake()
                }
            }
        }
    }

    fn get(&self, ty: TypeId) -> Option<DeserializeFn> {
        self.items.read().unwrap().get(&ty).copied()
    }

    fn wait_for(&self, ty: TypeId) -> BoxFuture<'static, DeserializeFn> {
        self.items
            .read()
            .unwrap()
            .get(&ty)
            .copied()
            .map(|item| Box::pin(ready(item)) as BoxFuture<DeserializeFn>)
            .unwrap_or_else(|| {
                Box::pin(WaitFor {
                    task: self
                        .tasks
                        .lock()
                        .unwrap()
                        .entry(ty)
                        .or_insert_with(|| Arc::new(AtomicWaker::new()))
                        .clone(),
                    registry: &REGISTRY,
                    id: ty,
                })
            })
    }
}

lazy_static! {
    pub(crate) static ref REGISTRY: Registry = Registry {
        items: RwLock::new(HashMap::new()),
        tasks: Mutex::new(WeakValueHashMap::new()),
    };
}

pub(crate) struct Id<'a>(ForkHandle, &'a mut Context);

impl<'de, 'a> DeserializeSeed<'de> for Id<'a> {
    type Value = Box<dyn SerdeAny>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut pool = LocalPool::new();
        let ty = pool.run_until(self.1.wait_for(self.0));
        let mut deserializer = erased_serde::Deserializer::erase(deserializer);
        (pool.run_until(REGISTRY.wait_for(ty)))(&mut deserializer).map_err(de::Error::custom)
    }
}

impl<'a> Id<'a> {
    pub(crate) fn new(channel: ForkHandle, context: &'a mut Context) -> Self {
        Id(channel, context)
    }
}
