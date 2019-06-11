use crate::{errors::Error, network::{DataChannel, mesh::{Negotiation, Peer, NegotiationItem, Channel}}};
use futures::{Sink, Stream, Poll, StartSend, AsyncSink, Async, Future, future::err};
use glib::{ObjectExt};
use gstreamer::{
    Element, ElementExt, ElementExtManual, ElementFactory, GObjectExtManualGst, GstBinExt,
    Pipeline, Promise, State, Structure,
};
use gstreamer_webrtc::WebRTCSessionDescription;

struct RTCNegotiation {}

impl RTCNegotiation {
    fn new() -> RTCNegotiation {
        RTCNegotiation {}
    }
}

impl Negotiation for RTCNegotiation {}

impl Sink for RTCNegotiation {
    type SinkItem = NegotiationItem;
    type SinkError = Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
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
        Ok(Async::Ready(None))
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
    fn new() -> RTCPeer {
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

pub(crate) fn new() -> (Box<dyn Peer>, Box<dyn Negotiation>) {
    gstreamer::init().unwrap();
    let main_loop = glib::MainLoop::new(None, false);
    let pipeline = Pipeline::new("main");
    let bus = pipeline.get_bus().unwrap();
    let webrtcbin = ElementFactory::make("webrtcbin", "sendrecv").unwrap();
    webrtcbin.set_property_from_str("bundle-policy", "max-bundle");
    pipeline.add(&webrtcbin).unwrap();

    webrtcbin
        .connect("on-negotiation-needed", false, move |values| {
            println!("negotiation needed");
            let webrtc = values[0].get::<Element>().unwrap();
            let promise = Promise::new_with_change_func(move |promise| {
                let offer = promise
                    .get_reply()
                    .unwrap()
                    .get_value("offer")
                    .unwrap()
                    .get::<WebRTCSessionDescription>()
                    .unwrap()
                    .get_sdp()
                    .as_text()
                    .unwrap();
                println!("{:?}", offer);
            });
            webrtc
                .emit("create-offer", &[&None::<Structure>, &Some(promise)])
                .unwrap();
            None
        })
        .unwrap();

    webrtcbin
        .connect("on-ice-candidate", false, move |values| {
            println!("negotiation needed");
            None
        })
        .unwrap();

    pipeline.set_state(State::Playing).unwrap();

    main_loop.run();

    (Box::new(RTCPeer::new()), Box::new(RTCNegotiation::new()))
}
