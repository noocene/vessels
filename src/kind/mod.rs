pub mod future;
pub use future::Future;
mod option;
mod result;
pub mod serde;
pub use self::serde::Serde;

use futures::{
    future::{ok, FutureResult},
    Future as IFuture,
};

use std::{
    ffi::{CString, OsString},
    marker::PhantomData,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    num::{
        NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
        NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    time::{Duration, SystemTime},
};

use crate::{channel::Channel, Kind};

pub trait IntoKind<K: Kind> {
    fn into_kind(self) -> K;
}

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

macro_rules! primitive_impl {
    ($($ty:ident)+) => {$(
        impl Kind for $ty {
            type ConstructItem = $ty;
            type ConstructFuture = Box<dyn IFuture<Item = $ty, Error = ()> + Send + 'static>;
            type DeconstructItem = ();
            type DeconstructFuture = Box<dyn IFuture<Item = (), Error = ()> + Send + 'static>;

            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                self,
                channel: C,
            ) -> Self::DeconstructFuture {
                Box::new(channel.send(self).then(|_| Ok(())))
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                channel: C,
            ) -> Self::ConstructFuture
            {
                Box::new(
                    channel
                        .into_future()
                        .map_err(|e| panic!(e))
                        .map(|v| v.0.unwrap()),
                )
            }
        }
    )+};
}

primitive_impl!(bool isize i8 i16 i32 i64 usize u8 u16 u32 u64 f32 f64 char CString String Ipv4Addr SocketAddrV4 SocketAddrV6 SocketAddr SystemTime OsString Ipv6Addr Duration NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroUsize NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroIsize);
