use crate::{
    channel::{Channel, ForkHandle},
    kind,
    kind::{ConstructResult, DeconstructResult, Future, WrappedError},
    reflect::{
        CallError, Cast, CastError, Erased, MethodIndex, MethodTypes, NameError, OutOfRangeError,
        Reflected, Trait,
    },
    Kind,
};

use alloc::sync::Arc;
use core::any::{Any, TypeId};
use futures::{SinkExt, StreamExt};
use std::sync::Mutex;

pub use derive::Share;

pub trait Share {
    fn share(&self) -> Self;
}

impl<T: Clone> Share for T {
    fn share(&self) -> Self {
        self.clone()
    }
}

pub struct Shared<T: Trait<T> + Reflected + ?Sized>(Arc<Mutex<Box<T>>>);

impl<T: Trait<T> + Reflected + ?Sized> Shared<T> {
    pub fn new(item: Box<T>) -> Self {
        Shared(Arc::new(Mutex::new(item)))
    }
}

impl<T: Trait<T> + Reflected + ?Sized> Share for Shared<T> {
    fn share(&self) -> Self {
        Shared(self.0.share())
    }
}

impl<T: Trait<T> + Reflected + ?Sized> Trait<T> for Shared<T> {
    fn call(
        &self,
        index: MethodIndex,
        args: Vec<Box<dyn Any + Send + Sync>>,
    ) -> Result<Box<dyn Any + Send + Sync>, CallError> {
        self.0.lock().unwrap().call(index, args)
    }
    fn call_mut(
        &mut self,
        index: MethodIndex,
        args: Vec<Box<dyn Any + Send + Sync>>,
    ) -> Result<Box<dyn Any + Send + Sync>, CallError> {
        self.0.lock().unwrap().call_mut(index, args)
    }
    fn call_move(
        self: Box<Self>,
        index: MethodIndex,
        args: Vec<Box<dyn Any + Send + Sync>>,
    ) -> Result<Box<dyn Any + Send + Sync>, CallError> {
        Arc::try_unwrap(self.0)
            .unwrap_or_else(|_| panic!())
            .into_inner()
            .unwrap()
            .call_move(index, args)
    }
    fn by_name(&self, name: &'_ str) -> Result<MethodIndex, NameError> {
        self.0.lock().unwrap().by_name(name)
    }
    fn count(&self) -> MethodIndex {
        self.0.lock().unwrap().count()
    }
    fn name_of(&self, index: MethodIndex) -> Result<String, OutOfRangeError> {
        self.0.lock().unwrap().name_of(index)
    }
    fn this(&self) -> TypeId {
        self.0.lock().unwrap().this()
    }
    fn name(&self) -> String {
        self.0.lock().unwrap().name()
    }
    fn types(&self, index: MethodIndex) -> Result<MethodTypes, OutOfRangeError> {
        self.0.lock().unwrap().types(index)
    }
    fn supertraits(&self) -> Vec<TypeId> {
        self.0.lock().unwrap().supertraits()
    }
    fn upcast_erased(self: Box<Self>, ty: TypeId) -> Result<Box<dyn Erased>, CastError> {
        Arc::try_unwrap(self.0)
            .unwrap_or_else(|_| panic!())
            .into_inner()
            .unwrap()
            .upcast_erased(ty)
    }
    fn erase(self: Box<Self>) -> Box<dyn Erased> {
        Arc::try_unwrap(self.0)
            .unwrap_or_else(|_| panic!())
            .into_inner()
            .unwrap()
            .erase()
    }
}

#[kind]
impl<T: Reflected + Trait<T> + ?Sized> Kind for Shared<T>
where
    Box<T>: Kind,
{
    type ConstructItem = ForkHandle;
    type ConstructError = WrappedError<<Box<T> as Kind>::ConstructError>;
    type ConstructFuture = Future<ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = WrappedError<<Box<T> as Kind>::DeconstructError>;
    type DeconstructFuture = Future<DeconstructResult<Self>>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            Ok(channel
                .send(
                    channel
                        .fork::<Box<T>>(Box::new(self).erase().downcast().unwrap())
                        .await?,
                )
                .await
                .map_err(WrappedError::Send)?)
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        mut channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            let handle = channel.next().await.ok_or(WrappedError::Insufficient {
                got: 0,
                expected: 1,
            })?;
            Ok(Shared::new(channel.get_fork(handle).await?))
        })
    }
}
