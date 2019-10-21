use crate::{SerdeAny, REGISTRY};

use super::{Context, Id};

use serde::{
    de::{DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor},
    ser::{SerializeMap, SerializeSeq, Serializer},
    Serialize,
};

use std::fmt;

pub struct Item(pub(crate) u32, pub(crate) Box<dyn SerdeAny>, Context);

impl Serialize for Item {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry("channel", &self.0)?;
            map.serialize_entry("data", self.1.as_ref())?;
            map.end()
        } else {
            if self.2.len() == 1 {
                self.1.serialize(serializer)
            } else {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(&self.0)?;
                seq.serialize_element(self.1.as_ref())?;
                seq.end()
            }
        }
    }
}

impl Item {
    pub(crate) fn new(channel: u32, content: Box<dyn SerdeAny>, context: Context) -> Self {
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
        let channel = seq.next_element()?.unwrap();
        let data = seq
            .next_element_seed(Id::new(channel, &mut self.0))?
            .unwrap();
        Ok(Item(channel, data, self.0))
    }

    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut channel: Option<u32> = None;
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
                _ => panic!(),
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
        let deserializer = &mut erased_serde::Deserializer::erase(deserializer)
            as &mut dyn erased_serde::Deserializer;
        if human_readable {
            deserializer
                .deserialize_map(ItemVisitor(self))
                .map_err(|e| {
                    println!("{:?}", e);
                    panic!();
                })
        } else {
            if let Some((idx, ty)) = self.only() {
                Ok(Item::new(
                    idx,
                    (REGISTRY.get(&ty.0).unwrap())(deserializer).map_err(|_| panic!())?,
                    self.clone(),
                ))
            } else {
                deserializer
                    .deserialize_seq(ItemVisitor(self))
                    .map_err(|e| {
                        println!("{:?}", e);
                        panic!();
                    })
            }
        }
    }
}
