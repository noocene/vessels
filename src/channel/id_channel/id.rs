use super::Context;

use crate::{channel::ForkHandle, Kind, SerdeAny};

use serde::{
    de::{DeserializeOwned, DeserializeSeed, Deserializer, Error},
    Serialize,
};

use std::{collections::HashMap, sync::RwLock};

use core::any::TypeId;

use lazy_static::lazy_static;

type DeserializeFn =
    fn(&mut dyn erased_serde::Deserializer<'_>) -> erased_serde::Result<Box<dyn SerdeAny>>;

pub(crate) struct Registry {
    items: RwLock<HashMap<TypeId, DeserializeFn>>,
}

impl Registry {
    pub(crate) fn add_construct<K: Kind>(&self) {
        self.add_type::<K::ConstructItem>();
    }

    pub(crate) fn add_deconstruct<K: Kind>(&self) {
        self.add_type::<K::DeconstructItem>();
    }

    fn add_type<T: Serialize + DeserializeOwned + Sync + Send + 'static>(&self) {
        if !self.items.read().unwrap().contains_key(&TypeId::of::<T>()) {
            let mut items = self.items.write().unwrap();
            if !items.contains_key(&TypeId::of::<T>()) {
                items.insert(TypeId::of::<T>(), |de| {
                    <T as serde::Deserialize>::deserialize(de)
                        .map(|v| Box::new(v) as Box<dyn SerdeAny>)
                });
            }
        }
    }

    fn get(&self, ty: TypeId) -> Option<DeserializeFn> {
        self.items.read().unwrap().get(&ty).copied()
    }
}

lazy_static! {
    pub(crate) static ref REGISTRY: Registry = Registry {
        items: RwLock::new(HashMap::new()),
    };
}

pub(crate) struct Id<'a>(ForkHandle, &'a mut Context);

impl<'de, 'a> DeserializeSeed<'de> for Id<'a> {
    type Value = Box<dyn SerdeAny>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut deserializer = erased_serde::Deserializer::erase(deserializer);
        (REGISTRY
            .get(
                self.1
                    .get(self.0)
                    .ok_or(Error::custom(format!("ASYNC_WAIT {} ", (self.0).0)))?,
            )
            .ok_or(Error::custom("No deserializer in registry"))?)(&mut deserializer)
        .map_err(Error::custom)
    }
}

impl<'a> Id<'a> {
    pub(crate) fn new(channel: ForkHandle, context: &'a mut Context) -> Self {
        Id(channel, context)
    }
}
