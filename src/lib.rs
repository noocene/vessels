#[macro_use]
extern crate erased_serde;

pub mod channel;
use channel::{Channel, Fork, ForkHandle, Target};
pub mod format;
pub mod value;

pub use derive::value;
use erased_serde::Serialize as ErasedSerialize;
use failure::Error;
use futures::{future::ok, Future as IFuture};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    any::{Any, TypeId},
    ffi::{CString, OsString},
    marker::PhantomData,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    num::{
        NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
        NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    ops::Deref,
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

pub trait Value: Send + 'static {
    type ConstructItem: Serialize + DeserializeOwned + Send + 'static;
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Box<dyn IFuture<Item = Self, Error = Error> + Send + 'static>
    where
        Self: Sized;

    type DeconstructItem: Serialize + DeserializeOwned + Send + 'static;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        channel: C,
    ) -> Box<dyn IFuture<Item = (), Error = ()> + Send + 'static>;

    #[doc(hidden)]
    const DO_NOT_IMPLEMENT_THIS_TRAIT_MANUALLY: ();

    fn stream<T: Target>(
        self,
    ) -> Box<dyn IFuture<Item = T, Error = <T as Target>::Error> + Send + 'static>
    where
        Self: Sized,
    {
        T::new_with(self)
    }
}

#[value]
impl Value for () {
    type ConstructItem = ();
    type DeconstructItem = ();

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        _: C,
    ) -> Box<dyn IFuture<Item = (), Error = ()> + Send + 'static> {
        Box::new(ok(()))
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        _: C,
    ) -> Box<dyn IFuture<Item = Self, Error = Error> + Send + 'static> {
        Box::new(ok(()))
    }
}

#[value]
impl<T: Send + 'static> Value for PhantomData<T> {
    type ConstructItem = ();
    type DeconstructItem = ();

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        _: C,
    ) -> Box<dyn IFuture<Item = (), Error = ()> + Send + 'static> {
        Box::new(ok(()))
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        _: C,
    ) -> Box<dyn IFuture<Item = Self, Error = Error> + Send + 'static> {
        Box::new(ok(PhantomData))
    }
}

macro_rules! primitive_impl {
    ($($ty:ident)+) => {$(
        #[value]
        impl Value for $ty {
            type ConstructItem = $ty;
            type DeconstructItem = ();

            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                self,
                channel: C,
            ) -> Box<dyn IFuture<Item = (), Error = ()> + Send + 'static> {
                Box::new(channel.send(self).then(|_| Ok(())))
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                channel: C,
            ) -> Box<dyn IFuture<Item = Self, Error = Error> + Send + 'static>
            where
                Self: Sized,
            {
                Box::new(
                    channel
                        .into_future()
                        .map_err(|_| failure::err_msg("test"))
                        .map(|v| v.0.unwrap()),
                )
            }
        }
    )+};
}

primitive_impl!(bool isize i8 i16 i32 i64 usize u8 u16 u32 u64 f32 f64 char CString String Ipv4Addr SocketAddrV4 SocketAddrV6 SocketAddr SystemTime OsString Ipv6Addr Duration NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroUsize NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroIsize);

pub(crate) trait SerdeAny: erased_serde::Serialize + Any + Send {}

serialize_trait_object!(SerdeAny);

impl<T: ?Sized> SerdeAny for T where T: ErasedSerialize + Any + Send {}
