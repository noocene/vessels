use super::super::{ConnectError, ConnectionError, RawClient};

use crate::{
    core,
    core::Executor,
    kind::{Future, SinkStream},
};

use futures::{
    channel::{
        mpsc::{unbounded, UnboundedReceiver},
        oneshot::channel,
    },
    lock::Mutex,
    SinkExt, StreamExt,
};
use std::sync::{self, Arc};
use url::Url;
use ws::{connect, Message};

pub(crate) struct Client;

impl RawClient for Client {
    fn connect(
        &mut self,
        address: Url,
    ) -> Future<Result<SinkStream<Vec<u8>, ConnectionError, Vec<u8>>, ConnectError>> {
        Box::pin(async move {
            let (out_sender, out_receiver): (_, UnboundedReceiver<Vec<u8>>) = unbounded();
            let out_receiver = Arc::new(Mutex::new(out_receiver));
            let (data_sender, data_receiver) = unbounded();
            let (sender, receiver) = channel();
            let sender = Arc::new(sync::Mutex::new(Some(sender)));
            core::<Executor>().unwrap().spawn(async move {
                connect(address.to_string(), move |peer| {
                    sender.lock().unwrap().take().unwrap().send(()).unwrap();
                    let data_sender = data_sender.clone();
                    let out_receiver = out_receiver.clone();
                    core::<Executor>().unwrap().spawn(async move {
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
                .map_err(|e| ConnectError::Connect(e.into()))
                .unwrap();
            });
            receiver.await.unwrap();
            Ok(SinkStream::new(
                out_sender.sink_map_err(|e| ConnectionError { cause: e.into() }),
                data_receiver,
            ))
        })
    }
}

impl Client {
    pub(crate) fn new() -> Box<dyn RawClient> {
        Box::new(Client)
    }
}
