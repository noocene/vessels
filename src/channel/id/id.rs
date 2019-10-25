use super::Context;

use crate::REGISTRY;

use super::item::Content;

use serde::de::{self, DeserializeSeed, Deserializer};

pub(crate) struct Id<'a>(u32, &'a mut Context);

impl<'de, 'a> DeserializeSeed<'de> for Id<'a> {
    type Value = Content;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut deserializer = erased_serde::Deserializer::erase(deserializer);
        self.1
            .get(&self.0)
            .map(|ty| {
                let deserializer = &mut deserializer as &mut dyn erased_serde::Deserializer;
                (REGISTRY.get(&ty.0).unwrap())(deserializer)
                    .map_err(de::Error::custom)
                    .map(|item| Content::Concrete(item))
            })
            .unwrap()
        /*.unwrap_or_else(move || {
            let deserializer =
                &mut deserializer as &mut (dyn erased_serde::Deserializer + Send);
            Ok(Content::Eventual(Box::new(
                self.1
                    .wait_for(self.0)
                    .and_then(|ty| Ok((REGISTRY.get(&ty.0).unwrap())(deserializer).unwrap())),
            )))
        })*/
    }
}

impl<'a> Id<'a> {
    pub(crate) fn new(channel: u32, context: &'a mut Context) -> Self {
        Id(channel, context)
    }
}
