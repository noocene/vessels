use futures::future::{ok, Ready};

use std::{default::Default as IDefault, ops::Deref};

use crate::{channel::Channel, ConstructResult, DeconstructResult, Kind};

use super::{using, AsKind};

use void::Void;

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

impl<T: IDefault + Sync + Send + 'static> AsKind<using::Default> for T {
    type Kind = Default<T>;

    fn into_kind(self) -> Default<T> {
        Default::new()
    }
    fn from_kind(_: Self::Kind) -> Self {
        T::default()
    }
}

impl<T: IDefault> Default<T> {
    pub fn new() -> Self {
        Default(T::default())
    }
}

impl<T: IDefault + Sync + Send + 'static> Kind for Default<T> {
    type ConstructItem = ();
    type ConstructError = Void;
    type ConstructFuture = Ready<ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = Void;
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
