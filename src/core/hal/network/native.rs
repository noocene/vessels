use super::{ListenError, Server as IServer};

use crate::{
    channel::{IdChannel, OnTo},
    core,
    core::{executor::Spawn, Executor},
    format::{ApplyEncode, Cbor},
    kind::Future,
    Kind,
};

use futures::{channel::mpsc::unbounded, lock::Mutex, SinkExt, StreamExt};
use std::{marker::PhantomData, net::SocketAddr, sync::Arc};
use ws::{listen, Message};

pub struct Server<K: Kind>(PhantomData<K>);

impl<K: Kind> IServer<K> for Server<K> {
    fn listen(
        &mut self,
        address: SocketAddr,
        handler: Box<dyn FnMut() -> Future<K> + Sync + Send>,
    ) -> Future<Result<(), ListenError>> {
        Box::pin(async move {
            let handler = Arc::new(Mutex::new(handler));
            listen(address, move |peer| {
                let handler = handler.clone();
                let (sender, mut receiver) = unbounded();
                core::<dyn Executor>().unwrap().spawn(async move {
                    let (mut sink, mut stream) = (handler.lock().await.as_mut())()
                        .await
                        .on_to::<IdChannel>()
                        .await
                        .encode::<Cbor>()
                        .split();
                    core::<dyn Executor>().unwrap().spawn(async move {
                        while let Some(item) = stream.next().await {
                            peer.send(item).unwrap();
                        }
                    });
                    while let Some(item) = receiver.next().await {
                        sink.send(item).await.unwrap();
                    }
                });
                move |message| {
                    if let Message::Binary(data) = message {
                        sender.clone().start_send(data).unwrap();
                    }
                    Ok(())
                }
            }).map_err(|e| ListenError {
                cause: e.into()
            })?;
            Ok(())
        })
    }
}

impl<K: Kind> Server<K> {
    pub fn new() -> Box<dyn IServer<K>> {
        Box::new(Server(PhantomData))
    }
}
