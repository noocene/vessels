use std::{
    ffi::{CString, OsString},
    net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    num::{
        NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
        NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    time::{Duration, SystemTime},
};

use crate::{channel::Channel, ConstructResult, DeconstructResult, Kind};

use futures::{future::BoxFuture, SinkExt, StreamExt};

use void::Void;

macro_rules! primitive_impl {
    ($($ty:ident),+) => {$(
        impl Kind for $ty {
            type ConstructItem = $ty;
            type ConstructError = Void;
            type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
            type DeconstructItem = ();
            type DeconstructError = Void;
            type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;

            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                self,
                mut channel: C,
            ) -> Self::DeconstructFuture {
                Box::pin(async move {
                    channel.send(self).await.map_err(|_| panic!())
                })
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                mut channel: C,
            ) -> Self::ConstructFuture
            {
                Box::pin(async move {
                    Ok(channel.next().await.unwrap())
                })
            }
        }
    )+};
}

primitive_impl!(
    bool,
    isize,
    i8,
    i16,
    i32,
    i64,
    i128,
    usize,
    u8,
    u16,
    u32,
    u64,
    u128,
    f32,
    f64,
    char,
    CString,
    String,
    Ipv4Addr,
    SocketAddrV4,
    SocketAddrV6,
    SocketAddr,
    SystemTime,
    OsString,
    Ipv6Addr,
    Duration,
    NonZeroU8,
    NonZeroU16,
    NonZeroU32,
    NonZeroU64,
    NonZeroUsize,
    NonZeroI8,
    NonZeroI16,
    NonZeroI32,
    NonZeroI64,
    NonZeroIsize
);
