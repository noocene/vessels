use crate::{
    channel::{Channel, ForkHandle},
    Kind,
};

extern crate arrayvec;

use arrayvec::ArrayVec;

use futures::{future::join_all, Future};

macro_rules! arr_impls {
($($len:expr => ($($n:tt $nn:ident)+))+) => {
        $(

impl<T> Kind for [T; $len]
where
T: Kind
{
    type ConstructItem = [ForkHandle; $len];
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
            .and_then(|handles_vec| {
                let handles: ArrayVec<Self::ConstructItem> = handles_vec.into_iter().collect();
                channel
                    .send(handles.into_inner().unwrap())
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
                        vec![
                            $(channel.get_fork::<T>(item.unwrap()[$n])),+
                        ]
                    ).map(|items_vec| {
                        let memes: ArrayVec<[T; $len]> = items_vec.into_iter().collect::<ArrayVec<[T; $len]>>();
                        memes.into_inner().unwrap_or_else(|_| panic!("lol"))
                        })
                })
        )
    }
}
)+
}
}

arr_impls! {
    1 => (0 a)
    2 => (0 a 1 b)
    3 => (0 a 1 b 2 c)
}
