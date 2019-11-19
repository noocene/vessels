use futures::{
    future::{try_join_all, BoxFuture},
    SinkExt, StreamExt,
};

use crate::{
    channel::{Channel, ForkHandle},
    ConstructResult, DeconstructResult, Kind,
};

use super::{using, AsKind, WrappedError};

use std::{iter::FromIterator, ops::Deref};

#[derive(Clone, Debug, Copy, Hash, Eq, Ord, PartialOrd, PartialEq, Default)]
pub struct Iterator<T: Send + IntoIterator + FromIterator<<T as IntoIterator>::Item> + 'static>(
    pub T,
)
where
    <T as IntoIterator>::Item: Kind,
    T::IntoIter: Send;

impl<T: Send + IntoIterator + FromIterator<<T as IntoIterator>::Item> + 'static> Iterator<T>
where
    <T as IntoIterator>::Item: Kind,
    T::IntoIter: Send,
{
    pub fn new(item: T) -> Self {
        Iterator(item)
    }
}

impl<T: Send + IntoIterator + FromIterator<<T as IntoIterator>::Item> + 'static> Deref
    for Iterator<T>
where
    <T as IntoIterator>::Item: Kind,
    T::IntoIter: Send,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Send + IntoIterator + FromIterator<<T as IntoIterator>::Item> + 'static> From<T>
    for Iterator<T>
where
    <T as IntoIterator>::Item: Kind,
    T::IntoIter: Send,
{
    fn from(item: T) -> Self {
        Iterator(item)
    }
}

impl<T: Send + IntoIterator + FromIterator<<T as IntoIterator>::Item> + 'static>
    FromIterator<<T as IntoIterator>::Item> for Iterator<T>
where
    <T as IntoIterator>::Item: Kind,
    T::IntoIter: Send,
{
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item = <T as IntoIterator>::Item>,
    {
        Iterator(iter.into_iter().collect())
    }
}

impl<T: Send + IntoIterator + FromIterator<<T as IntoIterator>::Item> + 'static>
    AsKind<using::Iterator> for T
where
    <T as IntoIterator>::Item: Kind,
    T::IntoIter: Send,
{
    type Kind = Iterator<T>;

    fn into_kind(self) -> Iterator<T> {
        Iterator(self)
    }
    fn from_kind(kind: Self::Kind) -> Self {
        kind.0
    }
}

impl<T: Send + IntoIterator + FromIterator<<T as IntoIterator>::Item> + 'static> IntoIterator
    for Iterator<T>
where
    <T as IntoIterator>::Item: Kind,
    T::IntoIter: Send,
{
    type Item = <T as IntoIterator>::Item;
    type IntoIter = <T as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: Send + IntoIterator + FromIterator<<T as IntoIterator>::Item> + 'static> Kind
    for Iterator<T>
where
    <T as IntoIterator>::Item: Kind,
    T::IntoIter: Send,
{
    type ConstructItem = Vec<ForkHandle>;
    type ConstructError = WrappedError<<<T as IntoIterator>::Item as Kind>::ConstructError>;
    type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = WrappedError<<<T as IntoIterator>::Item as Kind>::DeconstructError>;
    type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            channel
                .send(
                    try_join_all(
                        self.0
                            .into_iter()
                            .map(|entry| channel.fork::<<T as IntoIterator>::Item>(entry)),
                    )
                    .await?,
                )
                .await
                .map_err(From::from)
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        mut channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            let handles = channel.next().await.ok_or(WrappedError::Insufficient {
                got: 0,
                expected: 1,
            })?;
            Ok(Iterator(
                try_join_all(
                    handles
                        .into_iter()
                        .map(|entry| channel.get_fork::<<T as IntoIterator>::Item>(entry)),
                )
                .await?
                .into_iter()
                .collect(),
            ))
        })
    }
}
