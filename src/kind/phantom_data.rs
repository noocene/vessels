use std::marker::PhantomData;

use crate::{channel::Channel, ConstructResult, DeconstructResult, Kind};

use futures::future::{ok, Ready};

impl<T: Send + 'static> Kind for PhantomData<T> {
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
        ok(PhantomData)
    }
}
