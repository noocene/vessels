use super::{ConnectError, ConnectionError, RawClient};

use crate::{
    core,
    core::{executor::Spawn, Executor},
    kind::Future,
    kind::SinkStream,
};

use failure::Fail;
use futures::{
    channel::{
        mpsc::{unbounded, UnboundedReceiver},
        oneshot::channel,
    },
    task::{Context, Poll},
    Future as IFuture, SinkExt, StreamExt,
};
use js_sys::Uint8Array;
use std::pin::Pin;
use url::Url;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{BinaryType, MessageEvent, WebSocket};

pub(crate) struct Client;

#[cfg(not(target_feature = "atomics"))]
unsafe impl<F: IFuture> Send for SyncSendAssert<F> {}
#[cfg(not(target_feature = "atomics"))]
unsafe impl<F: IFuture> Sync for SyncSendAssert<F> {}

impl<F: IFuture> IFuture for SyncSendAssert<F> {
    type Output = F::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.0.as_mut().poll(cx)
    }
}

struct SyncSendAssert<F: IFuture>(Pin<Box<F>>);

#[derive(Fail, Debug)]
#[fail(display = "the target port is being blocked")]
pub struct SecurityError;

impl RawClient for Client {
    fn connect(
        &mut self,
        address: Url,
    ) -> Future<Result<SinkStream<Vec<u8>, ConnectionError, Vec<u8>>, ConnectError>> {
        Box::pin(SyncSendAssert(Box::pin(async move {
            let socket = WebSocket::new(&address.into_string())
                .map_err(|_| ConnectError::Connect(SecurityError.into()))?;
            socket.set_binary_type(BinaryType::Arraybuffer);
            let (sender, receiver) = channel();
            let on_open = Closure::once(move || {
                sender.send(()).unwrap();
            });
            socket.set_onopen(Some(on_open.as_ref().unchecked_ref()));
            let (mut data_sender, data_receiver) = unbounded();
            let on_data = Closure::wrap(Box::new(move |e: MessageEvent| {
                data_sender
                    .start_send(Uint8Array::new(&e.data()).to_vec())
                    .unwrap();
            }) as Box<dyn FnMut(_)>);
            socket.set_onmessage(Some(on_data.as_ref().unchecked_ref()));
            on_data.forget();
            let (out_sender, mut out_receiver): (_, UnboundedReceiver<Vec<u8>>) = unbounded();
            core::<dyn Executor>()
                .unwrap()
                .spawn(SyncSendAssert(Box::pin(async move {
                    while let Some(mut data) = out_receiver.next().await {
                        socket.send_with_u8_array(data.as_mut_slice()).unwrap();
                    }
                })));
            receiver.await.unwrap();
            Ok(SinkStream::new(
                out_sender.sink_map_err(|e| ConnectionError { cause: e.into() }),
                data_receiver,
            ))
        })))
    }
}

impl Client {
    pub(crate) fn new() -> Box<dyn RawClient> {
        Box::new(Client)
    }
}
