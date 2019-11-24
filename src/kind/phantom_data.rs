use std::marker::PhantomData;

use crate::{channel::Channel, ConstructResult, DeconstructResult, Kind};

use futures::future::{ok, Ready};

use void::Void;

impl<T: Unpin + Sync + Send + 'static> Kind for PhantomData<T> {
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
        ok(PhantomData)
    }
}
