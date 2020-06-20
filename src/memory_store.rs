use crate::{
    resource::{
        hash::{Algorithm, Hasher},
        provider::ResourceProvider,
        Rehydrate,
    },
    Resource,
};
use core_error::Error;
use futures::{lock::Mutex, Future};
use std::{collections::HashMap, hash::Hash, pin::Pin, sync::Arc};

pub struct MemoryStore<A: Algorithm> {
    data: Arc<Mutex<HashMap<A::Hash, Vec<u8>>>>,
}

impl<A: Algorithm> Clone for MemoryStore<A> {
    fn clone(&self) -> Self {
        MemoryStore {
            data: self.data.clone(),
        }
    }
}

impl<A: Algorithm> MemoryStore<A> {
    pub fn new() -> Self {
        MemoryStore {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn intern<H: Hasher<A>, T, U: Rehydrate<T>>(
        &mut self,
        item: T,
    ) -> impl Future<Output = Result<Resource<T, U, A>, Box<dyn Error + Send>>>
    where
        A::Hash: Eq + Hash + Clone,
        U::DumpError: Error + Send + 'static,
    {
        let data = self.data.clone();

        async move {
            let mut data = data.lock().await;

            let item = U::dump(item)
                .await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;

            let mut hasher = H::new();

            hasher.write(&item);

            let hash = hasher.hash();

            data.insert(hash.clone(), item);

            Ok(Resource::new(hash))
        }
    }
}

impl<A: Algorithm> ResourceProvider<A> for MemoryStore<A>
where
    A::Hash: Hash + Eq + Send + 'static,
{
    type Fetch =
        Pin<Box<dyn Future<Output = Result<Option<Vec<u8>>, Box<dyn Error + Send>>> + Send>>;

    fn fetch(&self, hash: A::Hash) -> Self::Fetch {
        let data = self.data.clone();

        Box::pin(async move {
            let data = data.lock().await;

            Ok(data.get(&hash).cloned())
        })
    }
}
