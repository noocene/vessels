use crate::{
    channel::{Channel, ForkHandle},
    ConstructResult, Kind,
};

use futures::{
    future::{join_all, ok, BoxFuture},
    stream::once,
    FutureExt, SinkExt, StreamExt, TryFutureExt,
};

use std::{mem::MaybeUninit, ptr};

macro_rules! array_impl {
    ($($len:expr => ($($n:tt $nn:ident)+))+) => {$(
        impl<T> Kind for [T; $len]
            where T: Kind
        {
            type ConstructItem = Vec<ForkHandle>;
            type Error = ();
            type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
            type DeconstructItem = ();
            type DeconstructFuture = BoxFuture<'static, ()>;
            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                self,
                channel: C,
            ) -> Self::DeconstructFuture {
                let [$($nn),+] = self;
                Box::pin(
                    join_all(
                        vec![
                            $(channel.fork::<T>($nn)),+
                        ]
                    )
                    .then(move |handles| {
                        let channel = channel.sink_map_err(|_| panic!());
                        Box::pin(
                            once(ok(handles))
                                .forward(channel)
                                .unwrap_or_else(|_| panic!()),
                        )
                    }),
                )
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                channel: C,
            ) -> Self::ConstructFuture {
                Box::pin(
                    channel
                        .into_future()
                        .then(move |(item, channel)| {
                            join_all(
                                item.unwrap().into_iter().map(move |item| channel.get_fork::<T>(item).unwrap_or_else(|_| panic!()))
                            ).map(|items| {
                                let len = items.len();
                                if len != $len {
                                    panic!("expected data with {} elements, got {}", $len, len)
                                }
                                let mut arr = MaybeUninit::uninit();
                                for (i, item) in items.into_iter().enumerate() {
                                    unsafe { ptr::write((arr.as_mut_ptr() as *mut T).add(i), item) };
                                }
                                unsafe { arr.assume_init() }
                            })
                        })
                        .unit_error()
                )
            }
        })+
    }
}

array_impl! {
    1 => (0 a)
    2 => (0 a 1 b)
    3 => (0 a 1 b 2 c)
}
