use crate::{channel::Channel, Kind};

use futures::future::{ok, FutureResult};

impl Kind for () {
    type ConstructItem = ();
    type DeconstructItem = ();
    type ConstructFuture = FutureResult<(), ()>;
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
        ok(())
    }
}
