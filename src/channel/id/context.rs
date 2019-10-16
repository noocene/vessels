use std::{
    any::TypeId,
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::Value;

struct ContextState {
    channel_types: HashMap<u32, (TypeId, TypeId)>,
    unused_indices: Vec<u32>,
    next_index: u32,
}

#[derive(Clone)]
pub struct Context {
    state: Arc<Mutex<ContextState>>,
}

impl Context {
    pub(crate) fn new<V: Value>() -> Self {
        let mut channel_types = HashMap::new();

        channel_types.insert(
            0,
            (
                TypeId::of::<V::ConstructItem>(),
                TypeId::of::<V::DeconstructItem>(),
            ),
        );

        Context {
            state: Arc::new(Mutex::new(ContextState {
                channel_types,
                next_index: 1,
                unused_indices: vec![],
            })),
        }
    }

    pub(crate) fn get(&self, channel: &'_ u32) -> Option<(TypeId, TypeId)> {
        self.state
            .lock()
            .unwrap()
            .channel_types
            .get(channel)
            .map(|c| *c)
    }

    pub(crate) fn add(&self, construct: TypeId, deconstruct: TypeId) -> u32 {
        let mut state = self.state.lock().unwrap();

        if let Some(id) = state.unused_indices.pop() {
            state.channel_types.insert(id, (construct, deconstruct));
            id
        } else {
            let id = state.next_index;
            state.next_index += 1;
            state.channel_types.insert(id, (construct, deconstruct));
            id
        }
    }
}
