use std::{
    any::TypeId,
    collections::HashMap,
    pin::Pin,
    sync::{Arc, Mutex, RwLock, Weak},
};

use weak_table::PtrWeakHashSet;

use futures::{
    task::{AtomicWaker, Context as FContext},
    Future, Poll,
};

use crate::{channel::ForkHandle, Kind};

struct ContextState {
    channel_types: HashMap<ForkHandle, TypeId>,
    unused_indices: Vec<ForkHandle>,
    next_index: ForkHandle,
}

#[derive(Clone)]
pub struct Context {
    state: Arc<RwLock<ContextState>>,
    tasks: Arc<Mutex<HashMap<ForkHandle, PtrWeakHashSet<Weak<AtomicWaker>>>>>,
}

pub(crate) struct WaitFor {
    task: Arc<AtomicWaker>,
    context: Context,
    id: ForkHandle,
}

impl Drop for WaitFor {
    fn drop(&mut self) {
        let mut tasks = self.context.tasks.lock().unwrap();
        if tasks.get(&self.id).unwrap().len() == 1 {
            tasks.remove(&self.id);
        }
    }
}

impl Future for WaitFor {
    type Output = TypeId;

    fn poll(self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Self::Output> {
        let state = self.context.state.read().unwrap();
        let output = state.channel_types.get(&self.id).copied().map_or_else(
            || {
                self.task.register(cx.waker());
                Poll::Pending
            },
            Poll::Ready,
        );
        drop(state);
        output
    }
}

impl Context {
    pub(crate) fn wait_for(&self, id: ForkHandle) -> WaitFor {
        let mut tasks = self.tasks.lock().unwrap();
        let task = Arc::new(AtomicWaker::new());
        tasks
            .entry(id)
            .or_insert_with(|| PtrWeakHashSet::new())
            .insert(task.clone());
        drop(tasks);
        WaitFor {
            task,
            context: self.clone(),
            id,
        }
    }

    pub(crate) fn get(&self, id: ForkHandle) -> Option<TypeId> {
        let state = self.state.read().unwrap();
        state.channel_types.get(&id).cloned()
    }

    pub(crate) fn new() -> Self {
        Context {
            state: Arc::new(RwLock::new(ContextState {
                channel_types: HashMap::new(),
                next_index: ForkHandle(0),
                unused_indices: vec![],
            })),
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub(crate) fn create<K: Kind>(&self) -> ForkHandle {
        let mut state = self.state.write().unwrap();
        let tasks = self.tasks.lock().unwrap();
        let d = TypeId::of::<K::DeconstructItem>();
        let id = if let Some(id) = state.unused_indices.pop() {
            state.channel_types.insert(id, d);
            id
        } else {
            let id = state.next_index;
            state.next_index = ForkHandle(state.next_index.0 + 1);
            state.channel_types.insert(id, d);
            id
        };
        if let Some(tasks) = tasks.get(&id) {
            tasks.iter().for_each(|task| task.wake())
        }
        drop(tasks);
        id
    }

    pub(crate) fn add<K: Kind>(&self, handle: ForkHandle) {
        let mut state = self.state.write().unwrap();
        let tasks = self.tasks.lock().unwrap();
        let c = TypeId::of::<K::ConstructItem>();
        state.channel_types.insert(handle, c);
        if let Some(tasks) = tasks.get(&handle) {
            tasks.iter().for_each(|task| task.wake())
        }
        drop(tasks);
        drop(state);
    }

    pub(crate) fn len(&self) -> usize {
        self.state.read().unwrap().channel_types.len()
    }

    pub(crate) fn only(&self) -> Option<(ForkHandle, TypeId)> {
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
