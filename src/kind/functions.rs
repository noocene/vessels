use crate::{
    channel::{Channel, ForkHandle},
    kind,
    kind::{ConstructResult, DeconstructResult, Flatten, Future},
    Kind,
};

use futures::{lock::Mutex, SinkExt, StreamExt, TryFutureExt};

use std::sync::Arc;

use void::Void;

#[kind]
impl<U: Kind + Flatten> Kind for Box<dyn Fn() -> U + Send + Sync> {
    type ConstructItem = ForkHandle;
    type ConstructError = Void;
    type ConstructFuture = Future<ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = Void;
    type DeconstructFuture = Future<DeconstructResult<Self>>;

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            while let Some(()) = channel.next().await {
                channel
                    .send(channel.fork((self)()).await.unwrap())
                    .unwrap_or_else(|_| panic!())
                    .await;
            }
            Ok(())
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            let channel = Arc::new(Mutex::new(channel));
            let closure: Box<dyn Fn() -> U + Send + Sync> = Box::new(move || {
                let channel = channel.clone();
                U::flatten(async move {
                    let mut channel = channel.lock().await;
                    channel.send(()).unwrap_or_else(|_| panic!()).await;
                    let handle = channel.next().await.expect("test2");
                    channel.get_fork(handle).await.expect("test3")
                })
            });
            Ok(closure)
        })
    }
}

#[kind]
impl<U: Kind + Flatten> Kind for Box<dyn FnMut() -> U + Send + Sync> {
    type ConstructItem = ForkHandle;
    type ConstructError = Void;
    type ConstructFuture = Future<ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = Void;
    type DeconstructFuture = Future<DeconstructResult<Self>>;

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        mut self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            while let Some(()) = channel.next().await {
                channel
                    .send(channel.fork((self)()).await.unwrap())
                    .unwrap_or_else(|_| panic!())
                    .await;
            }
            Ok(())
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            let channel = Arc::new(Mutex::new(channel));
            let closure: Box<dyn FnMut() -> U + Send + Sync> = Box::new(move || {
                let channel = channel.clone();
                U::flatten(async move {
                    let mut channel = channel.lock().await;
                    channel.send(()).unwrap_or_else(|_| panic!()).await;
                    let handle = channel.next().await.expect("test2");
                    channel.get_fork(handle).await.expect("test3")
                })
            });
            Ok(closure)
        })
    }
}

#[kind]
impl<U: Kind + Flatten> Kind for Box<dyn FnOnce() -> U + Send + Sync> {
    type ConstructItem = ForkHandle;
    type ConstructError = Void;
    type ConstructFuture = Future<ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = Void;
    type DeconstructFuture = Future<DeconstructResult<Self>>;

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            channel.next().await.unwrap();
            channel
                .send(channel.fork((self)()).await.unwrap())
                .unwrap_or_else(|_| panic!())
                .await;
            Ok(())
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        mut channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            let closure: Box<dyn FnOnce() -> U + Send + Sync> = Box::new(move || {
                U::flatten(async move {
                    channel.send(()).unwrap_or_else(|_| panic!()).await;
                    let handle = channel.next().await.expect("test2");
                    channel.get_fork(handle).await.expect("test3")
                })
            });
            Ok(closure)
        })
    }
}

#[kind]
impl<U: Kind + Flatten> Kind for Arc<Box<dyn Fn() -> U + Send + Sync>> {
    type ConstructItem = ForkHandle;
    type ConstructError = Void;
    type ConstructFuture = Future<ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = Void;
    type DeconstructFuture = Future<DeconstructResult<Self>>;

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            while let Some(()) = channel.next().await {
                channel
                    .send(channel.fork((self)()).await.unwrap())
                    .unwrap_or_else(|_| panic!())
                    .await;
            }
            Ok(())
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            let channel = Arc::new(Mutex::new(channel));
            let closure: Arc<Box<dyn Fn() -> U + Send + Sync>> = Arc::new(Box::new(move || {
                let channel = channel.clone();
                U::flatten(async move {
                    let mut channel = channel.lock().await;
                    channel.send(()).unwrap_or_else(|_| panic!()).await;
                    let handle = channel.next().await.expect("test2");
                    channel.get_fork(handle).await.expect("test3")
                })
            }));
            Ok(closure)
        })
    }
}

macro_rules! functions_impl {
    ($($len:expr => ($($n:tt $name:ident $nn:ident)+))+) => {$(
        #[allow(non_snake_case)]
        #[kind]
        impl<U: Kind + Flatten, $($name),+> Kind for Box<dyn Fn($($name),+) -> U + Send + Sync>
            where $($name: Kind),+
        {
            type ConstructItem = ForkHandle;
            type ConstructError = Void;
            type ConstructFuture = Future<ConstructResult<Self>>;
            type DeconstructItem = Vec<ForkHandle>;
            type DeconstructError = Void;
            type DeconstructFuture = Future<DeconstructResult<Self>>;

            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                self,
                mut channel: C,
            ) -> Self::DeconstructFuture {
                Box::pin(async move {
                    while let Some(handles) = channel.next().await {
                        channel
                            .send(
                                channel
                                    .fork((self)($(channel.get_fork::<$name>(handles[$n as usize]).await.unwrap()),+))
                                    .await
                                    .unwrap(),
                            )
                            .unwrap_or_else(|_| panic!())
                            .await;
                    }
                    Ok(())
                })
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                channel: C,
            ) -> Self::ConstructFuture {
                Box::pin(async move {
                    let channel = Arc::new(Mutex::new(channel));
                    let closure: Box<dyn Fn($($name),+) -> U + Send + Sync> =
                        Box::new(move |$($name),+| {
                            let channel = channel.clone();
                            U::flatten(async move {
                                let mut channel = channel.lock().await;
                                let handles = vec![
                                    $(channel.fork::<$name>($name).await.unwrap()),+
                                ];
                                channel.send(handles).unwrap_or_else(|_| panic!()).await;
                                let handle = channel.next().await.expect("test2");
                                channel.get_fork(handle).await.expect("test3")
                            })
                        });
                    Ok(closure)
                })
            }
        }
        #[allow(non_snake_case)]
        #[kind]
        impl<U: Kind + Flatten, $($name),+> Kind for Box<dyn FnMut($($name),+) -> U + Send + Sync>
            where $($name: Kind),+
        {
            type ConstructItem = ForkHandle;
            type ConstructError = Void;
            type ConstructFuture = Future<ConstructResult<Self>>;
            type DeconstructItem = Vec<ForkHandle>;
            type DeconstructError = Void;
            type DeconstructFuture = Future<DeconstructResult<Self>>;

            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                mut self,
                mut channel: C,
            ) -> Self::DeconstructFuture {
                Box::pin(async move {
                    while let Some(handles) = channel.next().await {
                        channel
                            .send(
                                channel
                                    .fork((self)($(channel.get_fork::<$name>(handles[$n as usize]).await.unwrap()),+))
                                    .await
                                    .unwrap(),
                            )
                            .unwrap_or_else(|_| panic!())
                            .await;
                    }
                    Ok(())
                })
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                channel: C,
            ) -> Self::ConstructFuture {
                Box::pin(async move {
                    let channel = Arc::new(Mutex::new(channel));
                    let closure: Box<dyn FnMut($($name),+) -> U + Send + Sync> =
                        Box::new(move |$($name),+| {
                            let channel = channel.clone();
                            U::flatten(async move {
                                let mut channel = channel.lock().await;
                                let handles = vec![
                                    $(channel.fork::<$name>($name).await.unwrap()),+
                                ];
                                channel.send(handles).unwrap_or_else(|_| panic!()).await;
                                let handle = channel.next().await.expect("test2");
                                channel.get_fork(handle).await.expect("test3")
                            })
                        });
                    Ok(closure)
                })
            }
        }
        #[allow(non_snake_case)]
        #[kind]
        impl<U: Kind + Flatten, $($name),+> Kind for Box<dyn FnOnce($($name),+) -> U + Send + Sync>
            where $($name: Kind),+
        {
            type ConstructItem = ForkHandle;
            type ConstructError = Void;
            type ConstructFuture = Future<ConstructResult<Self>>;
            type DeconstructItem = Vec<ForkHandle>;
            type DeconstructError = Void;
            type DeconstructFuture = Future<DeconstructResult<Self>>;

            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                self,
                mut channel: C,
            ) -> Self::DeconstructFuture {
                Box::pin(async move {
                    let handles = channel.next().await.unwrap();
                    channel
                        .send(
                            channel
                                .fork((self)($(channel.get_fork::<$name>(handles[$n as usize]).await.unwrap()),+))
                                .await
                                .unwrap(),
                        )
                        .unwrap_or_else(|_| panic!())
                        .await;
                    Ok(())
                })
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                mut channel: C,
            ) -> Self::ConstructFuture {
                Box::pin(async move {
                    let closure: Box<dyn FnOnce($($name),+) -> U + Send + Sync> =
                        Box::new(move |$($name),+| {
                            U::flatten(async move {
                                let handles = vec![
                                    $(channel.fork::<$name>($name).await.unwrap()),+
                                ];
                                channel.send(handles).unwrap_or_else(|_| panic!()).await;
                                let handle = channel.next().await.expect("test2");
                                channel.get_fork(handle).await.expect("test3")
                            })
                        });
                    Ok(closure)
                })
            }
        }
        #[allow(non_snake_case)]
        #[kind]
        impl<U: Kind + Flatten, $($name),+> Kind for Arc<Box<dyn Fn($($name),+) -> U + Send + Sync>>
            where $($name: Kind),+
        {
            type ConstructItem = ForkHandle;
            type ConstructError = Void;
            type ConstructFuture = Future<ConstructResult<Self>>;
            type DeconstructItem = Vec<ForkHandle>;
            type DeconstructError = Void;
            type DeconstructFuture = Future<DeconstructResult<Self>>;

            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                self,
                mut channel: C,
            ) -> Self::DeconstructFuture {
                Box::pin(async move {
                    let handles = channel.next().await.unwrap();
                    channel
                        .send(
                            channel
                                .fork((self)($(channel.get_fork::<$name>(handles[$n as usize]).await.unwrap()),+))
                                .await
                                .unwrap(),
                        )
                        .unwrap_or_else(|_| panic!())
                        .await;
                    Ok(())
                })
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                channel: C,
            ) -> Self::ConstructFuture {
                Box::pin(async move {
                    let channel = Arc::new(Mutex::new(channel));
                    let closure: Arc<Box<dyn Fn($($name),+) -> U + Send + Sync>> =
                        Arc::new(Box::new(move |$($name),+| {
                            let channel = channel.clone();
                            U::flatten(async move {
                                let mut channel = channel.lock().await;
                                let handles = vec![
                                    $(channel.fork::<$name>($name).await.unwrap()),+
                                ];
                                channel.send(handles).unwrap_or_else(|_| panic!()).await;
                                let handle = channel.next().await.expect("test2");
                                channel.get_fork(handle).await.expect("test3")
                            })
                        }));
                    Ok(closure)
                })
            }
        })+
    }
}

functions_impl! {
    1 => (0 T0 a)
    2 => (0 T0 a 1 T1 b)
    3 => (0 T0 a 1 T1 b 2 T2 c)
    4 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d)
    5 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e)
    6 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f)
    7 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g)
    8 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h)
    9 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i)
    10 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i 9 T9 j)
    11 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i 9 T9 j 10 T10 k)
    12 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i 9 T9 j 10 T10 k 11 T11 l)
    13 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i 9 T9 j 10 T10 k 11 T11 l 12 T12 m)
    14 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i 9 T9 j 10 T10 k 11 T11 l 12 T12 m 13 T13 n)
    15 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i 9 T9 j 10 T10 k 11 T11 l 12 T12 m 13 T13 n 14 T14 o)
    16 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i 9 T9 j 10 T10 k 11 T11 l 12 T12 m 13 T13 n 14 T14 o 15 T15 p)
}
