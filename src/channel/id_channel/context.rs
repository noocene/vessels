use std::{
    any::TypeId,
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

use crate::{channel::ForkHandle, Kind};

use futures::channel::oneshot::{channel, Receiver};

struct ContextState {
    channel_types: HashMap<ForkHandle, TypeId>,
    unused_indices: Vec<u32>,
    next_index: u32,
}

#[derive(Clone)]
pub struct Context {
    state: Arc<RwLock<ContextState>>,
}

impl Context {
    pub(crate) fn get(&self, id: &ForkHandle) -> Option<TypeId> {
        let state = self.state.read().unwrap();
        state.channel_types.get(id).cloned()
    }

    pub(crate) fn new() -> Self {
        Context {
            state: Arc::new(RwLock::new(ContextState {
                channel_types: HashMap::new(),
                next_index: 0,
                unused_indices: vec![],
            })),
        }
    }

    pub(crate) fn create<K: Kind>(&self) -> (ForkHandle, Receiver<()>) {
        let mut state = self.state.write().unwrap();
        let d = TypeId::of::<K::DeconstructItem>();
        let (sender, receiver) = channel();
        if let Some(id) = state.unused_indices.pop() {
            let handle = ForkHandle(id, Arc::new(Mutex::new(Some(sender))));
            state.channel_types.insert(handle.hash_clone(), d);
            (handle, receiver)
        } else {
            let id = state.next_index;
            state.next_index += 1;
            let handle = ForkHandle(id, Arc::new(Mutex::new(Some(sender))));
            state.channel_types.insert(handle.hash_clone(), d);
            (handle, receiver)
        }
    }

    pub(crate) fn add<K: Kind>(&self, handle: ForkHandle) {
        let mut state = self.state.write().unwrap();
        let c = TypeId::of::<K::ConstructItem>();
        state.channel_types.insert(handle, c);
    }
}
