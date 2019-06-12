use crate::{
    errors::Error,
    network::{
        mesh::{
            Channel, ConnectivityEstablishmentCandidate, Negotiation, NegotiationItem, Peer,
            SessionDescriptionType,
        },
        DataChannel,
    },
};
use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};
use futures::{
    future::err, lazy, task::AtomicTask, Async, AsyncSink, Future, Poll, Sink, StartSend, Stream,
};
use glib::ObjectExt;
use gstreamer::{
    Element, ElementExt, ElementExtManual, ElementFactory, GObjectExtManualGst, GstBinExt,
    Pipeline, Promise, State, Structure,
};

use gstreamer_sdp::SDPMessage;
use gstreamer_webrtc::{WebRTCSDPType, WebRTCSessionDescription};

use std::sync::Arc;

struct RTCNegotiation {
    outgoing: Receiver<NegotiationItem>,
    task: Arc<AtomicTask>,
    webrtc: Element,
}

impl RTCNegotiation {
    fn new(webrtc: Element) -> RTCNegotiation {
        let (sender, outgoing) = unbounded();
        let task = Arc::new(AtomicTask::new());
        let c_task = task.clone();
        webrtc
            .connect("on-negotiation-needed", false, move |values| {
                let task = c_task.clone();
                let webrtc = values[0].get::<Element>().unwrap();
                let sender = sender.clone();
                let set_desc_rtc = webrtc.clone();
                let promise = Promise::new_with_change_func(move |promise| {
                    let offer = promise
                        .get_reply()
                        .unwrap()
                        .get_value("offer")
                        .unwrap()
                        .get::<WebRTCSessionDescription>()
                        .unwrap();
                    set_desc_rtc
                        .emit("set-local-description", &[&offer, &None::<Promise>])
                        .unwrap();
                    sender
                        .send(NegotiationItem::SessionDescription(
                            SessionDescriptionType::Offer,
                            offer.get_sdp().as_text().unwrap(),
                        ))
                        .unwrap();
                    task.notify();
                });
                webrtc
                    .emit("create-offer", &[&None::<Structure>, &Some(promise)])
                    .unwrap();
                None
            })
            .unwrap();
        webrtc
            .connect("on-ice-candidate", false, move |values| {
                println!("ice candidate");
                None
            })
            .unwrap();
        RTCNegotiation {
            outgoing,
            task,
            webrtc,
        }
    }
    fn handle_session_description(&mut self, ty: SessionDescriptionType, sdp: String) {
        let gst_sdp = SDPMessage::parse_buffer(sdp.as_bytes()).unwrap();
        let rtc_sdp = WebRTCSessionDescription::new(
            match ty {
                SessionDescriptionType::Answer => WebRTCSDPType::Answer,
                SessionDescriptionType::Offer => WebRTCSDPType::Offer,
                SessionDescriptionType::Rollback => panic!("rollback not handled"),
            },
            gst_sdp,
        );
        let webrtc = self.webrtc.clone();
        let promise = Promise::new_with_change_func(move |_| {
            if let SessionDescriptionType::Offer = ty {
                let promise = Promise::new_with_change_func(move |promise| {
                    let offer = promise.get_reply();
                    let offer = offer.unwrap();
                    let offer = offer.get_value("answer").unwrap();
                    let offer = offer.get::<WebRTCSessionDescription>().unwrap();
                    println!("{}", offer.get_sdp().as_text().unwrap());
                });
                webrtc
                    .emit("create-answer", &[&None::<Structure>, &Some(promise)])
                    .unwrap();
            };
        });
        self.webrtc
            .emit("set-remote-description", &[&rtc_sdp, &promise])
            .unwrap();
    }
    fn handle_connectivity_establishment_candidate(
        &mut self,
        candidate: Option<ConnectivityEstablishmentCandidate>,
    ) {
    }
    fn handle_incoming(&mut self, incoming: NegotiationItem) {
        println!("incoming: {:?}", incoming);
        match incoming {
            NegotiationItem::SessionDescription(ty, sdp) => {
                self.handle_session_description(ty, sdp);
            }
            NegotiationItem::ConnectivityEstablishmentCandidate(candidate) => {
                self.handle_connectivity_establishment_candidate(candidate)
            }
        };
    }
}

impl Negotiation for RTCNegotiation {}

impl Sink for RTCNegotiation {
    type SinkItem = NegotiationItem;
    type SinkError = Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.handle_incoming(item);
        Ok(AsyncSink::Ready)
    }
    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Ok(Async::Ready(()))
    }
}

impl Stream for RTCNegotiation {
    type Item = NegotiationItem;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.outgoing.try_recv() {
            Ok(item) => Ok(Async::Ready(Some(item))),
            Err(err) => match err {
                TryRecvError::Disconnected => {
                    panic!("channel disconnected in negotiation stream");
                }
                TryRecvError::Empty => {
                    self.task.register();
                    Ok(Async::NotReady)
                }
            },
        }
    }
}

struct RTCDataChannel {}

impl DataChannel for RTCDataChannel {}

impl Sink for RTCDataChannel {
    type SinkItem = Vec<u8>;
    type SinkError = Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        Ok(AsyncSink::Ready)
    }
    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Ok(Async::Ready(()))
    }
}

impl Stream for RTCDataChannel {
    type Item = Vec<u8>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Ok(Async::Ready(None))
    }
}

struct RTCPeer {}

impl RTCPeer {
    fn new(webrtc: Element) -> RTCPeer {
        RTCPeer {}
    }
}

impl Peer for RTCPeer {
    fn data_channel(&mut self) -> Box<dyn Future<Item = Box<dyn DataChannel>, Error = Error>> {
        Box::new(err(Error::connection_failed()))
    }
}

impl Stream for RTCPeer {
    type Item = Channel;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Ok(Async::Ready(None))
    }
}

pub(crate) fn new(
) -> impl Future<Item = (Box<dyn Peer + 'static>, Box<dyn Negotiation + 'static>), Error = Error> {
    lazy(move || {
        gstreamer::init().unwrap();
        let main_loop = glib::MainLoop::new(None, false);
        let pipeline = Pipeline::new("main");
        let bus = pipeline.get_bus().unwrap();
        let webrtc = ElementFactory::make("webrtcbin", "sendrecv").unwrap();
        webrtc.set_property_from_str("bundle-policy", "max-bundle");
        pipeline.add(&webrtc).unwrap();
        let peer: Box<dyn Peer> = Box::new(RTCPeer::new(webrtc.clone()));
        let negotiation: Box<dyn Negotiation> = Box::new(RTCNegotiation::new(webrtc));
        pipeline.set_state(State::Playing).unwrap();
        tokio::spawn(lazy(move || {
            main_loop.run();
            Ok(())
        }));
        Ok((peer, negotiation))
    })
}
