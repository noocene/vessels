use super::{ConnectError, Network as INetwork, Peer as IPeer, StaticCandidate};

use crate::kind::Future;

use futures::{
    task::{Context, Poll},
    Future as IFuture,
};
use std::{fmt::Write, pin::Pin};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{RtcIceCandidateInit, RtcPeerConnection, RtcSdpType, RtcSessionDescriptionInit};

pub struct Network;

pub struct Peer;

impl IPeer for Peer {}

#[cfg(not(target_feature = "atomics"))]
unsafe impl<F: IFuture> Send for SendAssert<F> {}

impl<F: IFuture> IFuture for SendAssert<F> {
    type Output = F::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.0.as_mut().poll(cx)
    }
}

struct SendAssert<F: IFuture>(Pin<Box<F>>);

impl INetwork for Network {
    fn connect(
        &mut self,
        candidate: StaticCandidate,
    ) -> Future<Result<Box<dyn IPeer>, ConnectError>> {
        Box::pin(SendAssert(Box::pin(async move {
            let mut fingerprint = String::new();
            for byte in candidate.fingerprint.iter() {
                write!(fingerprint, "{:02X}:", byte).unwrap();
            }
            fingerprint.pop();
            let sdp = format!(
                r#"v=0
o=- 0 2 IN IP4 0.0.0.0
s=-
t=0 0
a=group:BUNDLE 0
a=msid-semantic: WMS
m=application 9 DTLS/SCTP 5000
c=IN IP4 0.0.0.0
a=setup:passive
a=mid:0
a=ice-ufrag:{}
a=ice-pwd:{}
a=fingerprint:sha-256 {}
"#,
                base64::encode(&candidate.ufrag),
                base64::encode(&candidate.pwd),
                fingerprint
            );
            let connection =
                RtcPeerConnection::new().expect("could not instantiate peer connection");
            connection.create_data_channel("signalling");
            let offer = JsFuture::from(connection.create_offer()).await.unwrap();
            JsFuture::from(connection.set_local_description(offer.dyn_ref().unwrap()))
                .await
                .unwrap();
            JsFuture::from(connection.set_remote_description(
                &RtcSessionDescriptionInit::new(RtcSdpType::Answer).sdp(&sdp),
            ))
            .await
            .unwrap();
            JsFuture::from(
                connection.add_ice_candidate_with_opt_rtc_ice_candidate_init(Some(
                    RtcIceCandidateInit::new(&format!(
                        "a=candidate:0 1 UDP 1 {} {} typ host",
                        candidate.addr.ip(),
                        candidate.addr.port()
                    ))
                    .sdp_mid(Some("0"))
                    .sdp_m_line_index(Some(0)),
                )),
            )
            .await
            .unwrap();
            Ok(Box::new(Peer) as Box<dyn IPeer>)
        })))
    }
}

impl Network {
    pub fn new() -> Box<dyn INetwork> {
        Box::new(Network)
    }
}
