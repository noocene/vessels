use crate::network::{
    mesh::{Channel, Negotiation, NegotiationItem, Peer, SessionDescriptionType},
    DataChannel,
};
use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};
use futures::{lazy, task::AtomicTask, Async, AsyncSink, Future, Poll, Sink, StartSend, Stream};
use glib::{Bytes, Object, ObjectExt};
use gstreamer::{
    message::MessageView, Element, ElementExt, ElementExtManual, ElementFactory,
    GObjectExtManualGst, GstBinExt, Pipeline, Promise, Registry, State, Structure,
};

use failure::Error;

use gstreamer_sdp::SDPMessage;
use gstreamer_webrtc::{WebRTCSDPType, WebRTCSessionDescription};

use std::{
    ops::{Deref, DerefMut},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

#[derive(Clone)]
struct RTCNegotiation {
    outgoing: Receiver<NegotiationItem>,
    outgoing_sender: Sender<NegotiationItem>,
    task: Arc<AtomicTask>,
    webrtc: Element,
}

impl RTCNegotiation {
    fn new(webrtc: Element) -> RTCNegotiation {
        let (sender, outgoing) = unbounded();
        let task = Arc::new(AtomicTask::new());
        let ice_task = task.clone();
        let ice_sender = sender.clone();
        webrtc
            .connect("on-ice-candidate", false, move |values| {
                let candidate = values[2].get::<String>().unwrap();
                ice_sender
                    .send(NegotiationItem::ConnectivityEstablishmentCandidate(Some(
                        candidate,
                    )))
                    .unwrap();
                ice_task.notify();
                None
            })
            .unwrap();
        RTCNegotiation {
            outgoing,
            task,
            webrtc,
            outgoing_sender: sender,
        }
    }
    fn negotiate(&mut self) {
        let webrtc = self.webrtc.clone();
        let sender = self.outgoing_sender.clone();
        let task = self.task.clone();
        let promise = Promise::new_with_change_func(move |promise| {
            let offer = promise
                .get_reply()
                .unwrap()
                .get_value("offer")
                .unwrap()
                .get::<WebRTCSessionDescription>()
                .unwrap();
            webrtc
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
        self.webrtc
            .emit("create-offer", &[&None::<Structure>, &Some(promise)])
            .unwrap();
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
        let outgoing_sender = self.outgoing_sender.clone();
        let task = self.task.clone();
        let promise = Promise::new_with_change_func(move |_| {
            if let SessionDescriptionType::Offer = ty {
                let webrtc_cloned = webrtc.clone();
                let promise = Promise::new_with_change_func(move |promise| {
                    let answer = promise
                        .get_reply()
                        .unwrap()
                        .get_value("answer")
                        .unwrap()
                        .get::<WebRTCSessionDescription>()
                        .unwrap();
                    webrtc_cloned
                        .emit("set-local-description", &[&answer, &None::<Promise>])
                        .unwrap();
                    outgoing_sender
                        .send(NegotiationItem::SessionDescription(
                            SessionDescriptionType::Answer,
                            answer.get_sdp().as_text().unwrap(),
                        ))
                        .unwrap();
                    task.notify();
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
    fn handle_connectivity_establishment_candidate(&mut self, candidate: Option<String>) {
        if let Some(candidate) = candidate {
            self.webrtc
                .emit("add-ice-candidate", &[&0u32, &candidate])
                .unwrap();
        }
    }
    fn handle_incoming(&mut self, incoming: NegotiationItem) {
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

#[derive(Clone)]
struct WebRTCDataChannel(Object);

impl Deref for WebRTCDataChannel {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WebRTCDataChannel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

unsafe impl Send for WebRTCDataChannel {}
unsafe impl Sync for WebRTCDataChannel {}

#[derive(Clone)]
struct RTCDataChannel {
    channel: WebRTCDataChannel,
    incoming_messages: Receiver<Vec<u8>>,
    incoming_messages_task: Arc<AtomicTask>,
}

struct RTCDataChannelOpening {
    channel: Option<RTCDataChannel>,
    open: Arc<AtomicBool>,
    open_task: Arc<AtomicTask>,
}

impl Future for RTCDataChannelOpening {
    type Item = Box<dyn DataChannel>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if !self.open.load(Ordering::SeqCst) {
            self.open_task.register();
            Ok(Async::NotReady)
        } else {
            let mut channel = None;
            std::mem::swap(&mut self.channel, &mut channel);
            Ok(Async::Ready(Box::new(channel.unwrap())))
        }
    }
}

impl RTCDataChannel {
    fn make_channel(channel: Object) -> RTCDataChannel {
        let (sender, incoming_messages) = unbounded();
        let incoming_messages_task = Arc::new(AtomicTask::new());
        let task = incoming_messages_task.clone();
        channel
            .connect("on-message-data", false, move |values| {
                let data = &(*(values[1].get::<Bytes>().unwrap()));
                sender.send(data.to_vec()).unwrap();
                task.notify();
                None
            })
            .unwrap();
        RTCDataChannel {
            channel: WebRTCDataChannel(channel),
            incoming_messages,
            incoming_messages_task,
        }
    }
    fn local_create(channel: Object) -> RTCDataChannelOpening {
        let task = Arc::new(AtomicTask::new());
        let open = Arc::new(AtomicBool::new(false));
        let open_cloned = open.clone();
        let task_cloned = task.clone();
        channel
            .connect("on-open", false, move |_| {
                open_cloned.store(true, Ordering::SeqCst);
                task_cloned.notify();
                None
            })
            .unwrap();
        let data_channel = RTCDataChannel::make_channel(channel);
        RTCDataChannelOpening {
            open,
            channel: Some(data_channel),
            open_task: task,
        }
    }
    fn create(channel: Object, sender: Sender<RTCDataChannel>, task: Arc<AtomicTask>) {
        let data_channel = RTCDataChannel::make_channel(channel.clone());
        channel
            .connect("on-open", false, move |_| {
                sender.send(data_channel.clone()).unwrap();
                task.notify();
                None
            })
            .unwrap();
    }
}

impl DataChannel for RTCDataChannel {}

impl Sink for RTCDataChannel {
    type SinkItem = Vec<u8>;
    type SinkError = Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.channel
            .emit("send-data", &[&Bytes::from(&item)])
            .unwrap();
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
        match self.incoming_messages.try_recv() {
            Ok(message) => Ok(Async::Ready(Some(message))),
            Err(err) => match err {
                TryRecvError::Disconnected => {
                    panic!("channel disconnected in incoming messages stream");
                }
                TryRecvError::Empty => {
                    self.incoming_messages_task.register();
                    Ok(Async::NotReady)
                }
            },
        }
    }
}

struct RTCPeer {
    webrtc: Element,
    negotiation: RTCNegotiation,
    receiver: Receiver<RTCDataChannel>,
    task: Arc<AtomicTask>,
}

impl RTCPeer {
    fn new(webrtc: Element, negotiation: RTCNegotiation) -> RTCPeer {
        let (sender, receiver) = unbounded();
        let task = Arc::new(AtomicTask::new());
        let task_cloned = task.clone();
        webrtc
            .connect("on-data-channel", false, move |values| {
                let channel = values[1].get::<Object>().unwrap();
                RTCDataChannel::create(channel, sender.clone(), task_cloned.clone());
                None
            })
            .unwrap();
        RTCPeer {
            webrtc,
            receiver,
            task,
            negotiation,
        }
    }
}

impl Peer for RTCPeer {
    fn data_channel(&mut self) -> Box<dyn Future<Item = Box<dyn DataChannel>, Error = Error>> {
        let webrtc = self.webrtc.clone();
        let mut negotiation = self.negotiation.clone();
        Box::new(lazy(move || {
            let channel = webrtc
                .emit(
                    "create-data-channel",
                    &[&("test".to_owned()), &None::<Structure>],
                )
                .unwrap()
                .unwrap();
            let channel = channel.get::<Object>().unwrap();
            negotiation.negotiate();
            Box::new(RTCDataChannel::local_create(channel))
        }))
    }
}

impl Stream for RTCPeer {
    type Item = Channel;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.receiver.try_recv() {
            Ok(channel) => Ok(Async::Ready(Some(Channel::DataChannel(Box::new(channel))))),
            Err(err) => match err {
                TryRecvError::Disconnected => {
                    panic!("channel disconnected in data channel stream");
                }
                TryRecvError::Empty => {
                    self.task.register();
                    Ok(Async::NotReady)
                }
            },
        }
    }
}

pub(crate) fn new(
) -> impl Future<Item = (Box<dyn Peer + 'static>, Box<dyn Negotiation + 'static>), Error = Error> {
    lazy(move || {
        gstreamer::init().unwrap();
        let needed = ["nice", "webrtc", "dtls", "srtp", "sctp"];
        let registry = Registry::get();
        let missing = needed
            .iter()
            .filter(|n| registry.find_plugin(n).is_none())
            .cloned()
            .collect::<Vec<_>>();
        if !missing.is_empty() {
            panic!("missing plugins: {:?}", missing);
        }
        let main_loop = glib::MainLoop::new(None, false);
        let pipeline = Pipeline::new("main");
        let bus = pipeline.get_bus().unwrap();
        bus.add_watch(move |_, msg| {
            match msg.view() {
                MessageView::Error(err) => eprintln!("Error: {}", err.get_debug().unwrap()),
                MessageView::Warning(warning) => {
                    eprintln!("Warning: {}", warning.get_debug().unwrap())
                }
                _ => {}
            };
            glib::Continue(true)
        });
        let webrtc = ElementFactory::make("webrtcbin", "sendrecv").unwrap();
        webrtc.set_property_from_str("bundle-policy", "max-bundle");
        pipeline.add(&webrtc).unwrap();
        let negotiation = RTCNegotiation::new(webrtc.clone());
        let peer: Box<dyn Peer> = Box::new(RTCPeer::new(webrtc, negotiation.clone()));
        let negotiation: Box<dyn Negotiation> = Box::new(negotiation);
        pipeline.set_state(State::Playing).unwrap();
        tokio::spawn(lazy(move || {
            main_loop.run();
            Ok(())
        }));
        Ok((peer, negotiation))
    })
}
