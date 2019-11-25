use super::super::{ConnectionError, ListenError, RawServer};

use crate::{
    core,
    core::Executor,
    kind::{Future, SinkStream},
};

use futures::{channel::mpsc::unbounded, lock::Mutex, SinkExt, StreamExt};
use std::{net::SocketAddr, sync::Arc};
use ws::{listen, Message};

pub(crate) struct Server;

impl RawServer for Server {
    fn listen(
        &mut self,
        address: SocketAddr,
        handler: Box<
            dyn FnMut(SinkStream<Vec<u8>, ConnectionError, Vec<u8>>) -> Future<()> + Sync + Send,
        >,
    ) -> Future<Result<(), ListenError>> {
        Box::pin(async move {
            let handler = Arc::new(Mutex::new(handler));
            listen(address, move |peer| {
                let handler = handler.clone();
                let (sender, receiver) = unbounded();
                core::<Executor>().unwrap().spawn(async move {
                    let (data_sender, mut stream) = unbounded();
                    core::<Executor>().unwrap().spawn(async move {
                        while let Some(item) = stream.next().await {
                            peer.send(item).unwrap();
                        }
                    });
                    (handler.lock().await.as_mut())(SinkStream::new(
                        data_sender.sink_map_err(|e| ConnectionError { cause: e.into() }),
                        receiver,
                    ))
                    .await;
                });
                move |message| {
                    if let Message::Binary(data) = message {
                        sender.clone().start_send(data).unwrap();
                    }
                    Ok(())
                }
            })
            .map_err(|e| ListenError { cause: e.into() })?;
            Ok(())
        })
    }
}

impl Server {
    pub(crate) fn new() -> Box<dyn RawServer> {
        Box::new(Server)
    }
}
