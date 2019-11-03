use super::Format;

use serde::{de::DeserializeSeed, Serialize};

use futures::{
    channel::oneshot::{channel, Receiver},
    future::BoxFuture,
    TryFutureExt,
};

/// A format implementing JavaScript Object Notation.
///
/// JSON is a human readable object
/// serialization format used in a diverse range of applications.
/// This wraps functionality provided by the `serde_json` crate.
///
/// For this format to be used the `json` feature must be enabled.
pub struct Json;

impl Format for Json {
    type Representation = String;
    type Error = serde_json::Error;

    fn serialize<T: Serialize>(item: T) -> Self::Representation {
        serde_json::to_string(&item).unwrap()
    }

    fn deserialize<'de, T: DeserializeSeed<'de>>(
        item: Self::Representation,
        context: T,
    ) -> BoxFuture<'static, Result<T::Value, Self::Error>>
    where
        T::Value: Send + 'static,
        T: Send + 'static,
    {
        let (sender, receiver): (_, Receiver<Result<T::Value, Self::Error>>) = channel();
        std::thread::spawn(move || {
            let mut deserializer = serde_json::Deserializer::from_reader(item.as_bytes());
            sender
                .send(context.deserialize(&mut deserializer))
                .map_err(|e| panic!(e))
                .unwrap();
        });
        Box::pin(receiver.unwrap_or_else(|e| panic!(e)))
    }
}
