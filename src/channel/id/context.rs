use std::{
    any::TypeId,
    collections::HashMap,
    marker::PhantomData,
    sync::{Arc, Mutex, RwLock},
};

use futures::{task::AtomicTask, Async, Future, Poll};

use crate::Value;

struct ContextState {
    channel_types: HashMap<u32, (TypeId, TypeId)>,
    unused_indices: Vec<u32>,
    next_index: u32,
}

#[derive(Clone)]
pub struct Context {
    state: Arc<RwLock<ContextState>>,
    tasks: Arc<Mutex<HashMap<u32, Arc<AtomicTask>>>>,
}

pub(crate) struct WaitFor<E> {
    task: Arc<AtomicTask>,
    context: Context,
    id: u32,
    _marker: PhantomData<E>,
}

impl<E> Future for WaitFor<E> {
    type Item = (TypeId, TypeId);
    type Error = E;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(self.context.get(&self.id).map_or_else(
            || {
                self.task.register();
                Async::NotReady
            },
            |item| {
                self.context.complete(self.id);
                Async::Ready(item)
            },
        ))
    }
}

impl Context {
    pub(crate) fn wait_for<E>(&self, id: u32) -> WaitFor<E> {
        let mut tasks = self.tasks.lock().unwrap();
        let task = tasks.get(&id).map(Clone::clone).unwrap_or_else(|| {
            let task = Arc::new(AtomicTask::new());
            tasks.insert(id, task.clone());
            task
        });
        WaitFor {
            task,
            context: self.clone(),
            id,
            _marker: PhantomData,
        }
    }

    fn complete(&self, id: u32) {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.remove(&id);
    }

    pub(crate) fn new_with<V: Value>() -> Self {
        let mut channel_types = HashMap::new();

        channel_types.insert(
            0,
            (
                TypeId::of::<V::ConstructItem>(),
                TypeId::of::<V::DeconstructItem>(),
            ),
        );

        Context {
            state: Arc::new(RwLock::new(ContextState {
                channel_types,
                next_index: 1,
                unused_indices: vec![],
            })),
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub(crate) fn new() -> Self {
        Context {
            state: Arc::new(RwLock::new(ContextState {
                channel_types: HashMap::new(),
                next_index: 1,
                unused_indices: vec![],
            })),
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub(crate) fn get(&self, channel: &'_ u32) -> Option<(TypeId, TypeId)> {
        self.state
            .read()
            .unwrap()
            .channel_types
            .get(channel)
            .map(|c| *c)
    }

    pub(crate) fn create<V: Value>(&self) -> u32 {
        let mut state = self.state.write().unwrap();
        let c = TypeId::of::<V::ConstructItem>();
        let d = TypeId::of::<V::DeconstructItem>();

        let id = if let Some(id) = state.unused_indices.pop() {
            state.channel_types.insert(id, (c, d));
            id
        } else {
            let id = state.next_index;
            state.next_index += 1;
            state.channel_types.insert(id, (c, d));
            id
        };
        self.tasks
            .lock()
            .unwrap()
            .get(&id)
            .map(|task| task.notify());
        id
    }

    pub(crate) fn add<V: Value>(&self, handle: u32) {
        let mut state = self.state.write().unwrap();
        let c = TypeId::of::<V::ConstructItem>();
        let d = TypeId::of::<V::DeconstructItem>();
        state.channel_types.insert(handle, (c, d));
        self.tasks
            .lock()
            .unwrap()
            .get(&handle)
            .map(|task| task.notify());
    }

    pub(crate) fn len(&self) -> usize {
        self.state.read().unwrap().channel_types.len()
    }

    pub(crate) fn only(&self) -> Option<(u32, (TypeId, TypeId))> {
        let state = self.state.read().unwrap();
        if state.channel_types.len() == 1 {
            state
                .channel_types
                .iter()
                .next()
                .map(|item| (*item.0, *item.1))
        } else {
            None
        }
    }
}
