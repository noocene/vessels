use super::Context;

use crate::{channel::ForkHandle, SerdeAny};

use serde::{
    de::{self, DeserializeOwned, DeserializeSeed, Deserializer},
    Serialize,
};

use std::{
    any::TypeId,
    collections::HashMap,
    sync::{Arc, Mutex, RwLock, Weak},
};

use futures::{future::ok, task::AtomicTask, Async, Future, Poll};

use weak_table::WeakValueHashMap;

use lazy_static::lazy_static;

type DeserializeFn =
    fn(&mut dyn erased_serde::Deserializer<'_>) -> erased_serde::Result<Box<dyn SerdeAny>>;

pub(crate) struct Registry {
    items: RwLock<HashMap<TypeId, DeserializeFn>>,
    tasks: Mutex<WeakValueHashMap<TypeId, Weak<AtomicTask>>>,
}

pub(crate) struct WaitFor {
    task: Arc<AtomicTask>,
    registry: &'static Registry,
    id: TypeId,
}

impl Future for WaitFor {
    type Item = DeserializeFn;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(self.registry.get(self.id).map_or_else(
            || {
                self.task.register();
                Async::NotReady
            },
            Async::Ready,
        ))
    }
}

impl Registry {
    pub(crate) fn add<T: Serialize + DeserializeOwned + Send + 'static>(&self) {
        if !self.items.read().unwrap().contains_key(&TypeId::of::<T>()) {
            self.items.write().unwrap().insert(TypeId::of::<T>(), |de| {
                <T as ::serde::Deserialize>::deserialize(de)
                    .map(|v| Box::new(v) as Box<dyn SerdeAny>)
            });
            if let Some(task) = self.tasks.lock().unwrap().get(&TypeId::of::<T>()) {
                task.notify()
            }
        }
    }

    fn get(&self, ty: TypeId) -> Option<DeserializeFn> {
        self.items.read().unwrap().get(&ty).copied()
    }

    fn wait_for(&self, ty: TypeId) -> Box<dyn Future<Item = DeserializeFn, Error = ()> + Send> {
        self.items
            .read()
            .unwrap()
            .get(&ty)
            .copied()
            .map(|item| {
                Box::new(ok(item)) as Box<dyn Future<Item = DeserializeFn, Error = ()> + Send>
            })
            .unwrap_or_else(|| {
                Box::new(WaitFor {
                    task: self
                        .tasks
                        .lock()
                        .unwrap()
                        .entry(ty)
                        .or_insert_with(|| Arc::new(AtomicTask::new()))
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
        let ty = self.1.wait_for(self.0).wait().unwrap();
        let mut deserializer = erased_serde::Deserializer::erase(deserializer);
        (REGISTRY.wait_for(ty.0).wait().unwrap())(&mut deserializer).map_err(de::Error::custom)
    }
}

impl<'a> Id<'a> {
    pub(crate) fn new(channel: ForkHandle, context: &'a mut Context) -> Self {
        Id(channel, context)
    }
}
