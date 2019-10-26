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
        let mut deserializer = erased_serde::Deserializer::erase(deserializer);
        self.1
            .get(&self.0)
            .map(|ty| (REGISTRY.get(&ty.0).unwrap())(&mut deserializer).map_err(de::Error::custom))
            .unwrap()
        /*.unwrap_or_else(move || {
            Ok(Content::Eventual(Box::new(
                self.1.wait_for(self.0).and_then(|ty| {
                    Ok((REGISTRY.get(&ty.0).unwrap())(&mut deserializer).unwrap())
                }),
            )))
        })*/
    }
}

impl<'a> Id<'a> {
    pub(crate) fn new(channel: u32, context: &'a mut Context) -> Self {
        Id(channel, context)
    }
}
