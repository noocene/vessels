use crate::errors::Error;
use crate::network::{
    mesh::{Channel, Negotiation, NegotiationItem, Peer, Role, SessionDescriptionType},
    DataChannel,
};
use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};
use futures::{
    future::err, task::AtomicTask, Async, AsyncSink, Future, Poll, Sink, StartSend, Stream,
};
use std::sync::Arc;
use stdweb::Reference;

struct RTCPeer {}

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

impl RTCPeer {
    fn new(role: Role, connection: Reference) -> RTCPeer {
        RTCPeer {}
    }
}

struct RTCNegotiation {
    outgoing: Receiver<NegotiationItem>,
    outgoing_sender: Sender<NegotiationItem>,
    outgoing_task: Arc<AtomicTask>,
    connection: Reference,
}

impl Negotiation for RTCNegotiation {}

impl Stream for RTCNegotiation {
    type Item = NegotiationItem;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.outgoing.try_recv() {
            Ok(negotiation) => {
                console!(log, format!("{:?}", negotiation));
                Ok(Async::Ready(Some(negotiation)))
            }
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
    fn new(role: Role, connection: Reference) -> RTCNegotiation {
        let (outgoing_sender, outgoing_receiver) = unbounded();
        let outgoing_task = Arc::new(AtomicTask::new());
        let outgoing_task_cloned = outgoing_task.clone();
        let outgoing_sender_cloned = outgoing_sender.clone();
        let send_offer = move |sdp: String| {
            outgoing_sender_cloned
                .send(NegotiationItem::SessionDescription(
                    SessionDescriptionType::Offer,
                    sdp,
                ))
                .expect("could not send offer");
            outgoing_task_cloned.notify();
        };
        match role {
            Role::Offering => {
                js! {
                    let connection = @{&connection};
                    connection.createOffer().catch((error) => {
                        console.log(error);
                    }).then((offer) => {
                        connection.setLocalDescription(offer).catch((error) => {
                            console.log(error);
                        }).then(() => {
                            @{send_offer}(offer.sdp);
                        });
                    });
                };
            }
            Role::Answering => {}
        }
        RTCNegotiation {
            outgoing: outgoing_receiver,
            outgoing_sender,
            outgoing_task,
            connection,
        }
    }
    fn handle_incoming(&mut self, incoming: NegotiationItem) {
        match incoming {
            NegotiationItem::SessionDescription(ty, sdp) => {
                self.handle_session_description(ty, sdp);
            }
            NegotiationItem::ConnectivityEstablishmentCandidate {
                username_fragment,
                candidate,
            } => {
                self.handle_connectivity_establishment_candidate(username_fragment, candidate);
            }
        };
    }
    fn handle_connectivity_establishment_candidate(&mut self, ufrag: String, candidate: String) {}
    fn handle_session_description(&mut self, ty: SessionDescriptionType, sdp: String) {
        let outgoing_task = self.outgoing_task.clone();
        let outgoing_sender = self.outgoing_sender.clone();
        let connection = self.connection.clone();
        let finish_handle = move || match ty {
            SessionDescriptionType::Offer => {
                let outgoing_sender = outgoing_sender.clone();
                let outgoing_task = outgoing_task.clone();
                let connection = connection.clone();
                let send_answer = move |sdp: String| {
                    outgoing_sender
                        .send(NegotiationItem::SessionDescription(
                            SessionDescriptionType::Answer,
                            sdp,
                        ))
                        .expect("could not send offer");
                    outgoing_task.notify();
                };
                js! {
                    let connection = @{connection};
                    connection.createAnswer().catch((error) => console.log(error)).then((answer) => {
                        connection.setLocalDescription(answer).catch((error) => {
                            console.log(error);
                        }).then(() => @{send_answer}(answer.sdp));
                    });
                }
            }
            SessionDescriptionType::Answer => {}
            SessionDescriptionType::Rollback => {}
        };
        js! {
            @{&self.connection}.setRemoteDescription(new RTCSessionDescription({sdp: @{sdp}, type: @{match ty {
                SessionDescriptionType::Answer => "answer",
                SessionDescriptionType::Offer => "offer",
                SessionDescriptionType::Rollback => "rollback"
            }}})).catch((error) => {
                console.log(error);
            }).then(() => {
                @{finish_handle}();
            });
        };
    }
}

pub(crate) fn new(role: Role) -> (Box<dyn Peer>, Box<dyn Negotiation>) {
    let connection: Reference = js! {
        let connection = new RTCPeerConnection();
        connection.onnegotiationneeded = () => {
            console.log("neg");
        };
        return connection;
    }
    .into_reference()
    .unwrap();
    (
        Box::new(RTCPeer::new(role, connection.clone())),
        Box::new(RTCNegotiation::new(role, connection)),
    )
}
