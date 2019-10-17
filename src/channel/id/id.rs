use super::Context;

use crate::{SerdeAny, REGISTRY};

use serde::de::{DeserializeSeed, Deserializer};

pub(crate) struct Id<'a>(u32, &'a mut Context);

impl<'de, 'a> DeserializeSeed<'de> for Id<'a> {
    type Value = Box<dyn SerdeAny>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ty = self.1.get(&self.0).unwrap();
        let deserializer = &mut erased_serde::Deserializer::erase(deserializer)
            as &mut dyn erased_serde::Deserializer;
        (REGISTRY.get(&ty.0).unwrap())(deserializer).map_err(|_| panic!())
    }
}

impl<'a> Id<'a> {
    pub(crate) fn new(channel: u32, context: &'a mut Context) -> Self {
        Id(channel, context)
    }
}