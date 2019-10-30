use std::marker::PhantomData;

use crate::{channel::Channel, ConstructResult, Kind};

use futures::future::{ok, ready, Ready};

impl<T: Send + 'static> Kind for PhantomData<T> {
    type ConstructItem = ();
    type Error = ();
    type ConstructFuture = Ready<ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructFuture = Ready<()>;

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        _: C,
    ) -> Self::DeconstructFuture {
        ready(())
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        _: C,
    ) -> Self::ConstructFuture {
        ok(PhantomData)
    }
}
