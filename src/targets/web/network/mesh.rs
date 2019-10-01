use crate::errors::Error;
use crate::network::{
    mesh::{Channel, Negotiation, NegotiationItem, Peer, SessionDescriptionType},
    DataChannel,
};
use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};
use futures::{lazy, task::AtomicTask, Async, AsyncSink, Future, Poll, Sink, StartSend, Stream};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use wasm_bindgen::{prelude::*, JsCast};

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

#[derive(Clone)]
struct RTCDataChannel {
    channel: web_sys::RtcDataChannel,
    data: Receiver<Vec<u8>>,
    task: Arc<AtomicTask>,
}

impl DataChannel for RTCDataChannel {}

impl Stream for RTCDataChannel {
    type Item = Vec<u8>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.data.try_recv() {
            Ok(message) => Ok(Async::Ready(Some(message))),
            Err(err) => match err {
                TryRecvError::Disconnected => {
                    panic!("channel disconnected in channel stream");
                }
                TryRecvError::Empty => {
                    self.task.register();
                    Ok(Async::NotReady)
                }
            },
        }
    }
}

impl Sink for RTCDataChannel {
    type SinkItem = Vec<u8>;
    type SinkError = Error;

    fn start_send(
        &mut self,
        mut item: Self::SinkItem,
    ) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.channel
            .send_with_u8_array(&mut item)
            .expect("Failed to send on channel");
        Ok(AsyncSink::Ready)
    }
    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Ok(Async::Ready(()))
    }
}

impl RTCDataChannel {
    fn make_channel(channel: web_sys::RtcDataChannel) -> RTCDataChannel {
        let (sender, data) = unbounded();
        let task = Arc::new(AtomicTask::new());
        let task_cloned = task.clone();
        let on_message_closure = Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
            let buffer = js_sys::Uint8Array::new(&e.data());
            let mut data = vec![0u8; buffer.length() as usize];
            buffer.copy_to(&mut data);
            sender.send(data).unwrap();
            task_cloned.notify();
        }) as Box<dyn FnMut(_)>);
        channel.set_onmessage(Some(on_message_closure.as_ref().unchecked_ref()));
        on_message_closure.forget();
        RTCDataChannel {
            channel,
            data,
            task,
        }
    }
    fn new(channel: web_sys::RtcDataChannel, sender: Sender<Channel>, add_task: Arc<AtomicTask>) {
        let data_channel = RTCDataChannel::make_channel(channel.clone());
        let on_open_closure = Closure::wrap(Box::new(move || {
            sender
                .send(Channel::DataChannel(Box::new(data_channel.clone())))
                .unwrap();
            add_task.notify();
        }) as Box<dyn FnMut()>);
        channel.set_onopen(Some(on_open_closure.as_ref().unchecked_ref()));
        on_open_closure.forget();
    }
    fn new_local(channel: web_sys::RtcDataChannel) -> RTCDataChannelOpening {
        let open_task = Arc::new(AtomicTask::new());
        let open = Arc::new(AtomicBool::new(false));
        let task = open_task.clone();
        let open_cloned = open.clone();
        let on_open_closure = Closure::wrap(Box::new(move || {
            open_cloned.store(true, Ordering::SeqCst);
            task.notify();
        }) as Box<dyn FnMut()>);
        channel.set_onopen(Some(on_open_closure.as_ref().unchecked_ref()));
        on_open_closure.forget();
        RTCDataChannelOpening {
            channel: Some(RTCDataChannel::make_channel(channel)),
            open,
            open_task,
        }
    }
}

struct RTCPeer {
    connection: web_sys::RtcPeerConnection,
    channels: Receiver<Channel>,
    task: Arc<AtomicTask>,
}

impl Peer for RTCPeer {
    fn data_channel(&mut self) -> Box<dyn Future<Item = Box<dyn DataChannel>, Error = Error>> {
        Box::new(RTCDataChannel::new_local(
            self.connection.create_data_channel("test"),
        ))
    }
}

impl Stream for RTCPeer {
    type Item = Channel;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.channels.try_recv() {
            Ok(channel) => Ok(Async::Ready(Some(channel))),
            Err(err) => match err {
                TryRecvError::Disconnected => {
                    panic!("channel disconnected in channel stream");
                }
                TryRecvError::Empty => {
                    self.task.register();
                    Ok(Async::NotReady)
                }
            },
        }
    }
}

impl RTCPeer {
    fn new(connection: web_sys::RtcPeerConnection) -> RTCPeer {
        let (sender, receiver) = unbounded();
        let task = Arc::new(AtomicTask::new());
        let add_task = task.clone();
        connection.set_ondatachannel(Some(
            Closure::wrap(Box::new(move |channel: web_sys::RtcDataChannel| {
                RTCDataChannel::new(channel, sender.clone(), add_task.clone());
            }) as Box<dyn FnMut(_)>)
            .as_ref()
            .unchecked_ref(),
        ));
        RTCPeer {
            connection,
            channels: receiver,
            task,
        }
    }
}

struct RTCNegotiation {
    outgoing: Receiver<NegotiationItem>,
    outgoing_sender: Sender<NegotiationItem>,
    outgoing_task: Arc<AtomicTask>,
    connection: web_sys::RtcPeerConnection,
}

impl Negotiation for RTCNegotiation {}

impl Stream for RTCNegotiation {
    type Item = NegotiationItem;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.outgoing.try_recv() {
            Ok(negotiation) => Ok(Async::Ready(Some(negotiation))),
            Err(err) => match err {
                TryRecvError::Disconnected => {
                    panic!("channel disconnected in negotiation stream");
                }
                TryRecvError::Empty => {
                    self.outgoing_task.register();
                    Ok(Async::NotReady)
                }
            },
        }
    }
}

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

impl RTCNegotiation {
    fn new(connection: web_sys::RtcPeerConnection) -> RTCNegotiation {
        let (outgoing_sender, outgoing_receiver) = unbounded();
        let outgoing_task = Arc::new(AtomicTask::new());
        let outgoing_task_cloned = outgoing_task.clone();
        let outgoing_sender_cloned = outgoing_sender.clone();
        let n_connection = connection.clone();
        let ice_sender = outgoing_sender.clone();
        let ice_task = outgoing_task.clone();
        let ice_termination_sender = outgoing_sender.clone();
        let ice_termination_task = outgoing_task.clone();
        connection.set_onnegotiationneeded(Some(
            Closure::wrap(Box::new(move || {
                let outgoing_task_cloned = outgoing_task_cloned.clone();
                let outgoing_sender_cloned = outgoing_sender_cloned.clone();
                let m_connection = n_connection.clone();
                n_connection
                    .create_offer()
                    .then(&Closure::wrap(Box::new(move |sdp: JsValue| {
                        let sdp = sdp
                            .dyn_ref::<web_sys::RtcSessionDescription>()
                            .unwrap()
                            .clone();
                        let outgoing_task_cloned = outgoing_task_cloned.clone();
                        let outgoing_sender_cloned = outgoing_sender_cloned.clone();
                        m_connection
                            .set_local_description(
                                &web_sys::RtcSessionDescriptionInit::new(sdp.type_())
                                    .sdp(&sdp.sdp()),
                            )
                            .then(&Closure::wrap(Box::new(move |_| {
                                outgoing_sender_cloned
                                    .send(NegotiationItem::SessionDescription(
                                        SessionDescriptionType::Offer,
                                        sdp.sdp(),
                                    ))
                                    .expect("could not send offer");
                                outgoing_task_cloned.notify();
                            })
                                as Box<dyn FnMut(_)>));
                    }) as Box<dyn FnMut(_)>));
            }) as Box<dyn Fn()>)
            .as_ref()
            .unchecked_ref(),
        ));
        connection.set_onicecandidate(Some(
            Closure::wrap(Box::new(move |c: web_sys::RtcIceCandidate| {
                if c.candidate().is_empty() {
                    ice_termination_sender
                        .send(NegotiationItem::ConnectivityEstablishmentCandidate(None))
                        .unwrap();
                    ice_termination_task.notify();
                } else {
                    ice_sender
                        .send(NegotiationItem::ConnectivityEstablishmentCandidate(Some(
                            c.candidate(),
                        )))
                        .expect("could not send candidate");
                    ice_task.notify();
                }
            }) as Box<dyn FnMut(_)>)
            .as_ref()
            .unchecked_ref(),
        ));
        RTCNegotiation {
            outgoing: outgoing_receiver,
            outgoing_sender,
            outgoing_task,
            connection,
        }
    }
    //fn create_offer(&mut self) {}
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
    fn handle_connectivity_establishment_candidate(&mut self, candidate: Option<String>) {
        match &candidate {
            Some(candidate) => {
                self.connection
                    .add_ice_candidate_with_opt_rtc_ice_candidate_init(Some(
                        &web_sys::RtcIceCandidateInit::new(candidate),
                    ));
            }
            None =>
                /*js! {
                @{&self.connection}.addIceCandidate(null);
            }*/
                {}
        };
    }
    fn handle_session_description(&mut self, ty: SessionDescriptionType, sdp: String) {
        let outgoing_task = self.outgoing_task.clone();
        let outgoing_sender = self.outgoing_sender.clone();
        let connection = self.connection.clone();
        self.connection
            .set_remote_description(&web_sys::RtcSessionDescriptionInit::new(match ty {
                SessionDescriptionType::Answer => web_sys::RtcSdpType::Answer,
                SessionDescriptionType::Offer => web_sys::RtcSdpType::Offer,
                SessionDescriptionType::Rollback => web_sys::RtcSdpType::Rollback,
            }))
            .then(&Closure::wrap(Box::new(move |_| {
                match ty {
                    SessionDescriptionType::Offer => {
                        let outgoing_sender = outgoing_sender.clone();
                        let outgoing_task = outgoing_task.clone();
                        let connection = connection.clone();
                        connection.create_answer().then(&Closure::wrap(Box::new(
                            move |sdp: JsValue| {
                                let sdp = sdp
                                    .dyn_ref::<web_sys::RtcSessionDescription>()
                                    .unwrap()
                                    .clone();
                                let outgoing_sender = outgoing_sender.clone();
                                let outgoing_task = outgoing_task.clone();
                                connection
                                    .set_local_description(
                                        &web_sys::RtcSessionDescriptionInit::new(sdp.type_())
                                            .sdp(&sdp.sdp()),
                                    )
                                    .then(&Closure::wrap(Box::new(move |_| {
                                        outgoing_sender
                                            .send(NegotiationItem::SessionDescription(
                                                SessionDescriptionType::Answer,
                                                sdp.sdp(),
                                            ))
                                            .expect("could not send offer");
                                        outgoing_task.notify();
                                    })
                                        as Box<dyn FnMut(_)>));
                            },
                        )
                            as Box<dyn FnMut(_)>));
                    }
                    SessionDescriptionType::Answer => {}
                    SessionDescriptionType::Rollback => {}
                };
            }) as Box<dyn FnMut(_)>));
    }
}

pub(crate) fn new(
) -> impl Future<Item = (Box<dyn Peer + 'static>, Box<dyn Negotiation + 'static>), Error = Error> {
    lazy(move || {
        let connection =
            web_sys::RtcPeerConnection::new().expect("Could not instantiate peer connection");
        let peer: Box<dyn Peer> = Box::new(RTCPeer::new(connection.clone()));
        let negotiation: Box<dyn Negotiation> = Box::new(RTCNegotiation::new(connection));
        Ok((peer, negotiation))
    })
}
