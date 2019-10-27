use super::Context;

use crate::{channel::DeserializeSeed, SerdeAny, REGISTRY};

use serde::de::{self, Deserializer};

use futures::Future;

pub(crate) struct Id<'a>(u32, &'a mut Context);

impl<'de, 'a> DeserializeSeed<'de> for Id<'a> {
    type Value = Box<dyn SerdeAny>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ty = self.1.wait_for(self.0).wait().unwrap();
        let mut deserializer = erased_serde::Deserializer::erase(deserializer);
        (REGISTRY.get(&ty.0).unwrap())(&mut deserializer).map_err(de::Error::custom)
    }
}

impl<'a> Id<'a> {
    pub(crate) fn new(channel: u32, context: &'a mut Context) -> Self {
        Id(channel, context)
    }
}
