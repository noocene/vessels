pub mod id_channel;
pub use id_channel::IdChannel;

use serde::{
    de::{DeserializeOwned, DeserializeSeed},
    ser::{SerializeTupleStruct, Serializer},
    Deserialize, Serialize,
};

use std::{
    cmp::PartialEq,
    hash::{Hash, Hasher},
    marker::Unpin,
    sync::{Arc, Mutex},
};

use crate::Kind;

use futures::{channel::oneshot::Sender, future::BoxFuture, Sink, Stream};

#[derive(Deserialize, Clone)]
pub struct ForkHandle(
    pub(crate) u32,
    #[serde(skip)] pub Arc<Mutex<Option<Sender<()>>>>,
);

impl ForkHandle {
    pub fn new(idx: u32) -> Self {
        ForkHandle(idx, Arc::new(Mutex::new(None)))
    }
    pub fn hash_clone(&self) -> Self {
        ForkHandle(self.0, Arc::new(Mutex::new(None)))
    }
    pub fn send(&self) {
        if let Some(sender) = self.1.lock().unwrap().take() {
            let _ = sender.send(());
        }
    }
}

impl Serialize for ForkHandle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.send();
        let mut ts = serializer.serialize_tuple_struct("ForkHandle", 1)?;
        ts.serialize_field(&self.0)?;
        ts.end()
    }
}

impl PartialEq for ForkHandle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for ForkHandle {}

impl Hash for ForkHandle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

pub trait Fork: Send + 'static {
    fn fork<K: Kind>(&self, kind: K)
        -> BoxFuture<'static, Result<ForkHandle, K::DeconstructError>>;
    fn get_fork<K: Kind>(
        &self,
        fork_ref: ForkHandle,
    ) -> BoxFuture<'static, Result<K, K::ConstructError>>;
}

pub trait Channel<
    I: Serialize + DeserializeOwned + Send + 'static,
    O: Serialize + DeserializeOwned + Send + 'static,
>: Stream<Item = I> + Sink<O> + Fork + Send + Sync + Unpin
{
}

pub trait Shim<'a, T: Target<'a, K>, K: Kind>:
    Context<'a, Item = <T as Context<'a>>::Item>
{
    fn complete<
        C: Send + Stream<Item = <T as Context<'a>>::Item> + Sink<<T as Context<'a>>::Item> + 'static,
    >(
        self,
        input: C,
    ) -> BoxFuture<'static, Result<K, K::ConstructError>>;
}

pub trait Target<'a, K: Kind>: Context<'a> + Sized {
    type Shim: Shim<'a, Self, K>;

    fn new_with(kind: K) -> BoxFuture<'static, Self>
    where
        K::DeconstructFuture: Send;

    fn new_shim() -> Self::Shim;
}

pub trait Context<'de> {
    type Item: Serialize + 'static;
    type Target: DeserializeSeed<'de, Value = Self::Item> + Clone + Send + 'static;

    fn context(&self) -> Self::Target;
}

pub trait OnTo: Kind {
    fn on_to<'a, T: Target<'a, Self>>(self) -> BoxFuture<'static, T>
    where
        Self: Send + 'static,
        Self::DeconstructFuture: Send;
}

impl<K: Kind> OnTo for K {
    fn on_to<'a, T: Target<'a, Self>>(self) -> BoxFuture<'static, T>
    where
        Self: Send + 'static,
        Self::DeconstructFuture: Send,
    {
        T::new_with(self)
    }
}
