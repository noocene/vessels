use crate::resource::{
    hash::Algorithm,
    manager::{ResourceManager, ResourceRegistrant},
    provider::ResourceProvider,
    ResourceError,
};
use anyhow::Error;
use futures::{lock::Mutex, stream::FuturesUnordered, Future, StreamExt};
use protocol::allocated::ProtocolError;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    convert::Infallible,
    pin::Pin,
    sync::Arc,
};

#[derive(Clone)]
pub struct SimpleResourceManager {
    providers: Arc<
        Mutex<
            HashMap<
                TypeId,
                Vec<
                    Box<
                        dyn Fn(
                                Box<dyn Any + Send>,
                            ) -> Pin<
                                Box<dyn Future<Output = Result<Option<Vec<u8>>, Error>> + Send>,
                            > + Sync
                            + Send,
                    >,
                >,
            >,
        >,
    >,
}

impl ResourceManager for SimpleResourceManager {
    type Fetch =
        Pin<Box<dyn Future<Output = Result<Option<Vec<u8>>, ResourceError<Infallible>>> + Send>>;

    fn fetch(
        &self,
        algo: TypeId,
        mut hash: Box<dyn FnMut() -> Box<dyn Any + Send> + Send>,
    ) -> Self::Fetch {
        let providers = self.providers.clone();

        Box::pin(async move {
            let providers = providers.lock().await;

            let providers = providers
                .get(&algo)
                .ok_or(ResourceError::UnknownAlgorithm)?
                .as_slice();

            let mut fetch = providers
                .iter()
                .map(|provider| (provider)(hash()))
                .collect::<FuturesUnordered<_>>();

            while let Some(item) = fetch.next().await {
                if let Some(item) = item? {
                    return Ok(Some(item));
                }
            }

            Ok(None)
        })
    }
}

impl<A, T> ResourceRegistrant<A, T> for SimpleResourceManager
where
    T: ResourceProvider<A> + Send + Sync + Sized + 'static,
    T::Fetch: Unpin + Send + 'static,
    A: Algorithm + Send + 'static,
    Error: From<T::Error>,
{
    type Register = Pin<Box<dyn Future<Output = Result<(), ProtocolError>> + Send>>;

    fn register_provider(&mut self, provider: T) -> Self::Register {
        let providers = self.providers.clone();

        Box::pin(async move {
            let mut providers = providers.lock().await;

            providers
                .entry(TypeId::of::<A>())
                .or_insert(vec![])
                .push(Box::new(move |any| {
                    let fut = provider.fetch(*Box::<dyn Any>::downcast(any).unwrap());

                    Box::pin(async move { fut.await.map_err(From::from) })
                }));
            Ok(())
        })
    }
}

impl SimpleResourceManager {
    pub fn new() -> Self {
        SimpleResourceManager {
            providers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
