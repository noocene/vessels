use futures::future::{ok, Ready};

use std::{default::Default as IDefault, ops::Deref};

use crate::{channel::Channel, ConstructResult, DeconstructResult, Kind};

use super::{using, AsKind};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Default<T: IDefault>(T);

impl<T: IDefault> Deref for Default<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: IDefault> From<T> for Default<T> {
    fn from(_: T) -> Self {
        Default::new()
    }
}

impl<T: IDefault + Send + 'static> AsKind<using::Default> for T {
    type Kind = Default<T>;
    type ConstructFuture = Ready<Result<T, ()>>;

    fn into_kind(self) -> Default<T> {
        Default::new()
    }
    fn from_kind(_: <Default<T> as Kind>::ConstructFuture) -> Self::ConstructFuture {
        ok(T::default())
    }
}

impl<T: IDefault> Default<T> {
    pub fn new() -> Self {
        Default(T::default())
    }
}

impl<T: IDefault + Send + 'static> Kind for Default<T> {
    type ConstructItem = ();
    type ConstructError = ();
    type ConstructFuture = Ready<ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = ();
    type DeconstructFuture = Ready<DeconstructResult<Self>>;

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        _: C,
    ) -> Self::DeconstructFuture {
        ok(())
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        _: C,
    ) -> Self::ConstructFuture {
        ok(Default::new())
    }
}
