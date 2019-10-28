use std::marker::PhantomData;

use crate::{channel::Channel, Kind};

use futures::future::{ok, FutureResult};

impl<T: Send + 'static> Kind for PhantomData<T> {
    type ConstructItem = ();
    type ConstructFuture = FutureResult<PhantomData<T>, ()>;
    type DeconstructItem = ();
    type DeconstructFuture = FutureResult<(), ()>;

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
