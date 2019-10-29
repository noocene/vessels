use crate::{channel::Channel, Kind};

use futures::future::{ok, ready, Ready};

impl Kind for () {
    type ConstructItem = ();
    type Error = ();
    type ConstructFuture = Ready<Result<(), ()>>;
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
        ok(())
    }
}
