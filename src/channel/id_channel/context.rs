use std::{
    any::TypeId,
    collections::HashMap,
    sync::{Arc, RwLock},
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
}

impl Context {
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
        }
    }

    pub(crate) fn create<K: Kind>(&self) -> ForkHandle {
        let mut state = self.state.write().unwrap();
        let d = TypeId::of::<K::DeconstructItem>();
        if let Some(id) = state.unused_indices.pop() {
            state.channel_types.insert(id, d);
            id
        } else {
            let id = state.next_index;
            state.next_index = ForkHandle(state.next_index.0 + 1);
            state.channel_types.insert(id, d);
            id
        }
    }

    pub(crate) fn add<K: Kind>(&self, handle: ForkHandle) {
        let mut state = self.state.write().unwrap();
        let c = TypeId::of::<K::ConstructItem>();
        state.channel_types.insert(handle, c);
    }
}
