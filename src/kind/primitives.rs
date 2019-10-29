use std::{
    ffi::{CString, OsString},
    net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    num::{
        NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
        NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    time::{Duration, SystemTime},
};

use crate::{channel::Channel, Kind};

use futures::{
    future::{ok, BoxFuture},
    stream::once,
    FutureExt, SinkExt, StreamExt, TryFutureExt,
};

macro_rules! primitive_impl {
    ($($ty:ident)+) => {$(
        impl Kind for $ty {
            type ConstructItem = $ty;
            type Error = ();
            type ConstructFuture = BoxFuture<'static, Result<$ty, Self::Error>>;
            type DeconstructItem = ();
            type DeconstructFuture = BoxFuture<'static, ()>;

            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                self,
                channel: C,
            ) -> Self::DeconstructFuture {
                let channel = channel.sink_map_err(|_| panic!());
                Box::pin(
                    once(ok(self))
                        .forward(channel)
                        .unwrap_or_else(|_| panic!()),
                )
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                channel: C,
            ) -> Self::ConstructFuture
            {
                Box::pin(
                    channel
                        .into_future()
                        .map(|v| v.0.unwrap())
                        .unit_error(),
                )
            }
        }
    )+};
}

primitive_impl!(bool isize i8 i16 i32 i64 usize u8 u16 u32 u64 f32 f64 char CString String Ipv4Addr SocketAddrV4 SocketAddrV6 SocketAddr SystemTime OsString Ipv6Addr Duration NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroUsize NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroIsize);
