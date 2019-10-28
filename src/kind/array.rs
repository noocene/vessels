use crate::{
    channel::{Channel, ForkHandle},
    Kind,
};

use futures::{future::join_all, Future};

use std::{mem::MaybeUninit, ptr};

macro_rules! array_impl {
    ($($len:expr => ($($n:tt $nn:ident)+))+) => {$(
        impl<T> Kind for [T; $len]
        where
        T: Kind
        {
            type ConstructItem = Vec<ForkHandle>;
            type ConstructFuture = Box<dyn Future<Item = Self, Error = ()> + Send>;
            type DeconstructItem = ();
            type DeconstructFuture = Box<dyn Future<Item = (), Error = ()> + Send>;
            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                self,
                channel: C,
            ) -> Self::DeconstructFuture {
                let [$($nn),+] = self;
                Box::new(
                    join_all(
                        vec![
                            $(channel.fork::<T>($nn)),+
                        ]
                    )
                    .map_err(|_| panic!("lol"))
                    .and_then(|handles| {
                        channel
                            .send(handles)
                            .and_then(|_| Ok(()))
                            .map_err(|_| panic!())
                    }),
                )
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                channel: C,
            ) -> Self::ConstructFuture {
                Box::new(
                    channel
                        .into_future()
                        .map_err(|_| panic!("lol"))
                        .and_then(|(item, channel)| {
                            join_all(
                                item.unwrap().into_iter().map(move |item| channel.get_fork::<T>(item))
                            ).map(|items| -> [T; $len] {
                                let len = items.len();
                                if len != $len {
                                    panic!("expected data with {} elements, got {}", $len, len)
                                }
                                let mut arr: MaybeUninit<[T; $len]> = MaybeUninit::uninit();
                                for (i, item) in items.into_iter().enumerate() {
                                    unsafe { ptr::write((arr.as_mut_ptr() as *mut T).add(i) , item) };
                                }
                                unsafe { arr.assume_init() }
                            })
                        })
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
