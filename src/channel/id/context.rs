use std::{
    any::TypeId,
    collections::HashMap,
    pin::Pin,
    sync::{Arc, Mutex, RwLock, Weak},
};

use weak_table::WeakValueHashMap;

use futures::{
    task::{AtomicWaker, Context as FContext},
    Future, Poll,
};

use crate::{channel::ForkHandle, Kind};

struct ContextState {
    channel_types: HashMap<ForkHandle, (TypeId, TypeId)>,
    unused_indices: Vec<ForkHandle>,
    next_index: ForkHandle,
}

#[derive(Clone)]
pub struct Context {
    state: Arc<RwLock<ContextState>>,
    tasks: Arc<Mutex<WeakValueHashMap<ForkHandle, Weak<AtomicWaker>>>>,
}

pub(crate) struct WaitFor {
    task: Arc<AtomicWaker>,
    context: Context,
    id: ForkHandle,
}

impl Future for WaitFor {
    type Output = (TypeId, TypeId);

    fn poll(self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Self::Output> {
        self.context.get(self.id).map_or_else(
            || {
                self.task.register(cx.waker());
                Poll::Pending
            },
            Poll::Ready,
        )
    }
}

impl Context {
    pub(crate) fn wait_for(&self, id: ForkHandle) -> WaitFor {
        let mut tasks = self.tasks.lock().unwrap();
        let task = tasks.get(&id).unwrap_or_else(|| {
            let task = Arc::new(AtomicWaker::new());
            tasks.insert(id, task.clone());
            task
        });
        WaitFor {
            task,
            context: self.clone(),
            id,
        }
    }

    pub(crate) fn new_with<K: Kind>() -> Self {
        let mut channel_types = HashMap::new();

        channel_types.insert(
            ForkHandle(0),
            (
                TypeId::of::<K::ConstructItem>(),
                TypeId::of::<K::DeconstructItem>(),
            ),
        );

        Context {
            state: Arc::new(RwLock::new(ContextState {
                channel_types,
                next_index: ForkHandle(1),
                unused_indices: vec![],
            })),
            tasks: Arc::new(Mutex::new(WeakValueHashMap::new())),
        }
    }

    pub(crate) fn get(&self, channel: ForkHandle) -> Option<(TypeId, TypeId)> {
        self.state
            .read()
            .unwrap()
            .channel_types
            .get(&channel)
            .copied()
    }

    pub(crate) fn create<K: Kind>(&self) -> ForkHandle {
        let mut state = self.state.write().unwrap();
        let c = TypeId::of::<K::ConstructItem>();
        let d = TypeId::of::<K::DeconstructItem>();

        let id = if let Some(id) = state.unused_indices.pop() {
            state.channel_types.insert(id, (c, d));
            id
        } else {
            let id = state.next_index;
            state.next_index = ForkHandle(state.next_index.0 + 1);
            state.channel_types.insert(id, (c, d));
            id
        };
        if let Some(task) = self.tasks.lock().unwrap().get(&id) {
            task.wake()
        }
        id
    }

    pub(crate) fn add<K: Kind>(&self, handle: ForkHandle) {
        let mut state = self.state.write().unwrap();
        let c = TypeId::of::<K::ConstructItem>();
        let d = TypeId::of::<K::DeconstructItem>();
        state.channel_types.insert(handle, (c, d));
        if let Some(task) = self.tasks.lock().unwrap().get(&handle) {
            task.wake()
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.state.read().unwrap().channel_types.len()
    }

    pub(crate) fn only(&self) -> Option<(ForkHandle, (TypeId, TypeId))> {
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
