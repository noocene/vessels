use super::{Client as IClient, ConnectError, Peer as IPeer};

use crate::{kind::Future, Kind};

use failure::{Error, Fail};
use futures::{
    task::{Context, Poll},
    Future as IFuture, TryFutureExt,
};
use std::{fmt::Write, marker::PhantomData, pin::Pin};
use url::Url;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    RtcIceCandidateInit, RtcPeerConnection, RtcSdpType, RtcSessionDescriptionInit, WebSocket,
};

pub struct Client<K: Kind>(PhantomData<K>);

#[cfg(not(target_feature = "atomics"))]
unsafe impl<F: IFuture> Send for SendAssert<F> {}

impl<F: IFuture> IFuture for SendAssert<F> {
    type Output = F::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.0.as_mut().poll(cx)
    }
}

struct SendAssert<F: IFuture>(Pin<Box<F>>);

#[derive(Fail, Debug)]
#[fail(display = "the target port is being blocked")]
pub struct SecurityError;

impl<K: Kind> IClient<K> for Client<K> {
    fn connect(&mut self, address: Url) -> Future<Result<K, ConnectError>> {
        Box::pin(SendAssert(Box::pin(async move {
            let socket = WebSocket::new(&address.into_string())
                .map_err(|_| ConnectError::Connect(SecurityError.into()))?;
            unimplemented!()
        })))
    }
}

impl<K: Kind> Client<K> {
    pub fn new() -> Box<dyn IClient<K>> {
        Box::new(Client(PhantomData))
    }
}
