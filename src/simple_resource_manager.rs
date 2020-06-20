use crate::resource::{
    hash::Algorithm, manager::ResourceManager, provider::ResourceProvider, ResourceError,
};
use anyhow::Error;
use futures::{future::ready, lock::Mutex, stream::iter, Future, FutureExt, StreamExt};
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
                            > + Send,
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

        Box::pin(
            async move {
                let providers = providers.lock().await;

                let providers = providers
                    .get(&algo)
                    .ok_or(ResourceError::<Infallible>::UnknownAlgorithm)?;

                let futures = providers
                    .iter()
                    .map(|provider| (provider)(hash()).into_stream())
                    .collect::<Vec<_>>();

                let futures = iter(futures.into_iter()).flatten();

                Ok({
                    let future: Pin<
                        Box<dyn Future<Output = Option<Result<Option<Vec<u8>>, Error>>> + Send>,
                    > = Box::pin(
                        futures
                            .skip_while(|item| {
                                ready(
                                    item.as_ref()
                                        .map(|item| item.as_ref().map(|_| false))
                                        .unwrap_or(Some(false))
                                        .unwrap_or(true),
                                )
                            })
                            .into_future()
                            .map(|(data, _)| data),
                    );
                    future
                })
            }
            .then(
                |item: Result<
                    Pin<Box<dyn Future<Output = Option<Result<Option<Vec<u8>>, Error>>> + Send>>,
                    ResourceError<Infallible>,
                >| async { Ok(item?.await.transpose()?.flatten()) },
            ),
        )
    }
}

impl SimpleResourceManager {
    pub fn new() -> Self {
        SimpleResourceManager {
            providers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_provider<A: Algorithm + Any, T: ResourceProvider<A>>(
        &mut self,
        provider: T,
    ) -> impl Future<Output = ()>
    where
        T: Sync + Sized,
        T::Fetch: Unpin + Send + 'static,
        T: Send + 'static,
        A: Send + 'static,
        Error: From<T::Error>,
    {
        let providers = self.providers.clone();

        async move {
            let mut providers = providers.lock().await;

            providers
                .entry(TypeId::of::<A>())
                .or_insert(vec![])
                .push(Box::new(move |any| {
                    let fut = provider.fetch(*Box::<dyn Any>::downcast(any).unwrap());

                    Box::pin(async move { fut.await.map_err(From::from) })
                }));
        }
    }
}
