use serde::de::Deserializer;

pub trait DeserializeSeedBounded<'de, D: Deserializer<'de>>: Sized {
    type Entity;

    fn deserialize(self, deserializer: D) -> Result<SelfValue, D::Error>;
}
