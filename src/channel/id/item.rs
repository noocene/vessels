use crate::SerdeAny;

use super::{Context, Id, IdChannel};

use serde::{
    de::{DeserializeSeed, Deserializer, MapAccess, Visitor},
    ser::{SerializeMap, SerializeSeq, Serializer},
    Serialize,
};

use std::fmt;

pub struct Item(u32, pub(crate) Box<dyn SerdeAny>);

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
            let mut seq = serializer.serialize_seq(Some(2))?;
            seq.serialize_element(&self.0)?;
            seq.serialize_element(self.1.as_ref())?;
            seq.end()
        }
    }
}

impl Item {
    pub(crate) fn new(channel: u32, content: Box<dyn SerdeAny>) -> Self {
        Item(channel, content)
    }
}

struct ItemVisitor(Context);

impl<'de> Visitor<'de> for ItemVisitor {
    type Value = Item;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a channel item")
    }

    /*fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
    }*/

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
        Ok(Item(channel, data))
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
            deserializer
                .deserialize_seq(ItemVisitor(self))
                .map_err(|e| {
                    println!("{:?}", e);
                    panic!();
                })
        }
    }
}

impl<'de> DeserializeSeed<'de> for IdChannel {
    type Value = Item;

    fn deserialize<D>(self, deserializer: D) -> Result<Item, D::Error>
    where
        D: Deserializer<'de>,
    {
        let deserializer = &mut erased_serde::Deserializer::erase(deserializer)
            as &mut dyn erased_serde::Deserializer;
        if deserializer.is_human_readable() {
            deserializer
                .deserialize_map(ItemVisitor(self.context))
                .map_err(|e| {
                    println!("{:?}", e);
                    panic!();
                })
        } else {
            deserializer
                .deserialize_seq(ItemVisitor(self.context))
                .map_err(|e| {
                    println!("{:?}", e);
                    panic!();
                })
        }
    }
}
