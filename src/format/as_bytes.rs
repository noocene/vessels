use super::Format;

use std::marker::PhantomData;

use serde::{de::DeserializeSeed, Serialize};

use futures::Future;

pub struct AsBytes<T: Format>(PhantomData<T>);

/*impl<F: Format<Representation = String>> Format for AsBytes<F> {
    type Representation = Vec<u8>;

    fn serialize<T: Serialize>(item: T) -> Self::Representation {
        F::serialize(&item).as_bytes().to_owned()
    }

    fn deserialize<'de, T: DeserializeSeed<'de>>(
        item: Self::Representation,
        context: T,
    ) -> Box<dyn Future<Item = TValue, Error = ()> + Send>
    where
        TValue: Send + 'static,
        T: Send + 'static,
    {
        F::deserialize(String::from_utf8(item).unwrap(), context)
    }
}*/
