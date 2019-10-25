use crate::SerdeAny;

use super::{Context, Id};

use serde::{
    de::{self, Deserializer, MapAccess, SeqAccess, Visitor},
    ser::{SerializeMap, SerializeSeq, Serializer},
    Serialize,
};

use crate::channel::DeserializeSeed;

use futures::{future::ok, Future};
use std::fmt;

pub(crate) enum Content {
    Concrete(Box<dyn SerdeAny>),
    Eventual(Box<dyn Future<Item = Box<dyn SerdeAny>, Error = ()> + Send>),
}

impl Content {
    fn unwrap_concrete_ref(&self) -> &dyn SerdeAny {
        if let Content::Concrete(item) = self {
            item
        } else {
            panic!("Unwrapped eventual content as concrete")
        }
    }
    pub(crate) fn unwrap_eventual(
        self,
    ) -> Box<dyn Future<Item = Box<dyn SerdeAny>, Error = ()> + Send> {
        match self {
            Content::Concrete(item) => Box::new(ok::<_, ()>(item)),
            Content::Eventual(item) => item,
        }
    }
}

impl Serialize for Content {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.unwrap_concrete_ref().serialize(serializer)
    }
}

pub struct Item(pub(crate) u32, pub(crate) Content, Context);

impl Serialize for Item {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry("channel", &self.0)?;
            map.serialize_entry("data", &self.1)?;
            map.end()
        } else {
            let mut seq = serializer.serialize_seq(Some(2))?;
            seq.serialize_element(&self.0)?;
            seq.serialize_element(&self.1)?;
            seq.end()
        }
    }
}

impl Item {
    pub(crate) fn new(channel: u32, content: Content, context: Context) -> Self {
        Item(channel, content, context)
    }
}

struct ItemVisitor(Context);

impl<'de> Visitor<'de> for ItemVisitor {
    type Value = Item;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a channel item")
    }

    fn visit_seq<A>(mut self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let channel = seq
            .next_element()?
            .ok_or(de::Error::invalid_length(0, &"two elements"))?;
        let data = seq
            .next_element_seed(Id::new(channel, &mut self.0))?
            .ok_or(de::Error::invalid_length(1, &"two elements"))?;
        Ok(Item(channel, data, self.0))
    }

    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut channel = None;
        let mut data = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_ref() {
                "channel" => {
                    if channel.is_some() {
                        return Err(serde::de::Error::duplicate_field("channel"));
                    }
                    channel = Some(map.next_value()?);
                }
                "data" => {
                    if data.is_some() {
                        return Err(serde::de::Error::duplicate_field("data"));
                    }
                    data = Some(map.next_value_seed(Id::new(channel.unwrap(), &mut self.0))?);
                }
                name => {
                    return Err(de::Error::unknown_field(name, &["data", "channel"]));
                }
            }
        }
        let channel = channel.ok_or_else(|| serde::de::Error::missing_field("channel"))?;
        let data = data.ok_or_else(|| serde::de::Error::missing_field("data"))?;
        Ok(Item(channel, data, self.0))
    }
}

impl<'de> DeserializeSeed<'de> for Context {
    type Value = Item;

    fn deserialize<D>(self, deserializer: D) -> Result<Item, D::Error>
    where
        D: Deserializer<'de>,
    {
        let human_readable = deserializer.is_human_readable();
        if human_readable {
            Ok(deserializer.deserialize_map(ItemVisitor(self))?)
        } else {
            Ok(deserializer.deserialize_seq(ItemVisitor(self))?)
        }
    }
}
