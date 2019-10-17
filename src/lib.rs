#[macro_use]
extern crate erased_serde;
#[macro_use]
extern crate mopa;

pub mod channel;
pub use channel::IntoStream;
use channel::{Channel, Target};
pub mod format;
pub mod value;

pub use derive::value;
use erased_serde::Serialize as ErasedSerialize;
use futures::{
    future::{ok, FutureResult},
    Future,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    any::{Any, TypeId},
    ffi::{CString, OsString},
    marker::PhantomData,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    num::{
        NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
        NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    time::{Duration, SystemTime},
};

pub(crate) struct ErasedDeserialize {
    ty: TypeId,
    func: DeserializeFn,
}

impl ErasedDeserialize {
    fn new(ty: TypeId, func: DeserializeFn) -> Self {
        ErasedDeserialize { ty, func }
    }
}

type DeserializeFn =
    fn(&mut dyn erased_serde::Deserializer) -> erased_serde::Result<Box<dyn SerdeAny>>;

inventory::collect!(ErasedDeserialize);

pub trait Value: Sized {
    type ConstructItem: Serialize + DeserializeOwned + Send + 'static;
    type ConstructFuture: Future<Item = Self> + Send + 'static;

    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture;

    type DeconstructItem: Serialize + DeserializeOwned + Send + 'static;
    type DeconstructFuture: Future<Item = ()> + Send + 'static;

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        channel: C,
    ) -> Self::DeconstructFuture;

    #[doc(hidden)]
    const DO_NOT_IMPLEMENT_THIS_TRAIT_MANUALLY: ();
}

#[value]
impl Value for () {
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

#[value]
impl<T: Send + 'static> Value for PhantomData<T> {
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
        #[value]
        impl Value for $ty {
            type ConstructItem = $ty;
            type ConstructFuture = Box<dyn Future<Item = $ty, Error = ()> + Send + 'static>;
            type DeconstructItem = ();
            type DeconstructFuture = Box<dyn Future<Item = (), Error = ()> + Send + 'static>;

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

pub(crate) trait SerdeAny: erased_serde::Serialize + mopa::Any + Send {
    fn as_any(self) -> Box<dyn Any>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

mopafy!(SerdeAny);

serialize_trait_object!(SerdeAny);

impl<T: ?Sized> SerdeAny for T where T: ErasedSerialize + mopa::Any + Send {}
