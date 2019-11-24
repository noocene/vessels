use super::super::{Client as IClient, ConnectError};

use crate::{
    channel::IdChannel,
    core,
    core::{executor::Spawn, Executor},
    format::{ApplyDecode, Cbor},
    kind::Future,
    kind::SinkStream,
    Kind,
};

use futures::{
    channel::{
        mpsc::{unbounded, UnboundedReceiver},
        oneshot::channel,
    },
    lock::Mutex,
    StreamExt,
};
use std::{marker::PhantomData, sync::{Arc, self}};
use url::Url;
use ws::{connect, Message};

pub struct Client<K: Kind>(PhantomData<K>);

impl<K: Kind> IClient<K> for Client<K> {
    fn connect(&mut self, address: Url) -> Future<Result<K, ConnectError>> {
        Box::pin(async move {
            let (out_sender, out_receiver): (_, UnboundedReceiver<Vec<u8>>) = unbounded();
            let out_receiver = Arc::new(Mutex::new(out_receiver));
            let (data_sender, data_receiver) = unbounded();
            let (sender, receiver) = channel();
            let sender = Arc::new(sync::Mutex::new(Some(sender)));
            core::<dyn Executor>().unwrap().spawn(async move {
                connect(address.to_string(), move |peer| {
                    sender.lock().unwrap().take().unwrap().send(()).unwrap();
                    let data_sender = data_sender.clone();
                    let out_receiver = out_receiver.clone();
                    core::<dyn Executor>().unwrap().spawn(async move {
                        while let Some(item) = out_receiver.lock().await.next().await {
                            peer.send(item).unwrap();
                        }
                    });
                    move |message| {
                        if let Message::Binary(data) = message {
                            data_sender.clone().start_send(data).unwrap();
                        }
                        Ok(())
                    }
                })
                .map_err(|e| ConnectError::Connect(e.into())).unwrap();
            });
            receiver.await.unwrap();
            SinkStream::new(out_sender, data_receiver)
                .decode::<IdChannel, Cbor>()
                .await
                .map_err(|e: K::ConstructError| ConnectError::Construct(e.into()))
        })
    }
}

impl<K: Kind> Client<K> {
    pub fn new() -> Box<dyn IClient<K>> {
        Box::new(Client(PhantomData))
    }
}
