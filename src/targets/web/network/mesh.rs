use futures::{
    future::err, lazy, task::AtomicTask, Async, AsyncSink, Future, Poll, Sink, StartSend, Stream,
};

use serde::{Serialize, Serializer};

use stdweb::{web::error::DomException, Reference};

use std::sync::Arc;

use crossbeam_channel::{bounded, Receiver, Sender, TryRecvError};

use crate::{
    errors::Error,
    network::mesh::{Answer, Candidate, Negotiation, Offer, Peer},
};

struct RTCPeer {
    connection: Reference,
}

struct RTCAcceptAnswer {
    connection: Reference,
}

impl Future for RTCAcceptAnswer {
    type Item = Box<dyn Peer>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Err(Error::connection_failed())
    }
}

impl RTCAcceptAnswer {
    fn new(connection: Reference, answer: String) -> RTCAcceptAnswer {
        js! {
            @{&connection}.setRemoteDescription({sdp: @{answer}, type: "answer"});
            @{&connection}.createDataChannel("test");
        }
        RTCAcceptAnswer { connection }
    }
}

impl Negotiation for RTCAcceptAnswer {}

impl Sink for RTCAcceptAnswer {
    type SinkItem = Candidate;
    type SinkError = Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        Ok(AsyncSink::Ready)
    }
    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Ok(Async::Ready(()))
    }
}

impl Stream for RTCAcceptAnswer {
    type Item = Candidate;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Ok(Async::NotReady)
    }
}

impl RTCPeer {}

struct RTCOffer {
    task: Arc<AtomicTask>,
    receiver: Receiver<Option<String>>,
    connection: Reference,
}

impl RTCOffer {
    fn new() -> RTCOffer {
        let task = Arc::new(AtomicTask::new());
        let (sender, receiver) = bounded(1);
        let s_sender = sender.clone();
        let f_task = task.clone();
        let offer_fail = move |_: DomException| {
            sender.send(None).unwrap();
            f_task.notify();
        };
        let s_task = task.clone();
        let offer_succeed = move |sdp: String| {
            s_sender.send(Some(sdp)).unwrap();
            s_task.notify();
        };
        let peer_connection = js! {
            let connection = new RTCPeerConnection();
            connection.createOffer().catch(@{offer_fail.clone()}).then((desc) => {
                connection.setLocalDescription(desc, () => {
                    @{offer_succeed}(desc.sdp);
                }, () => {
                    @{offer_fail}();
                });
            });
            return connection;
        }
        .into_reference()
        .unwrap();
        RTCOffer {
            task,
            receiver,
            connection: peer_connection,
        }
    }
}

type AnswerCallback = Box<dyn FnOnce(Answer) -> Box<dyn Negotiation + 'static> + Send + 'static>;

impl Future for RTCOffer {
    type Item = (Offer, AnswerCallback);
    type Error = Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.receiver.try_recv() {
            Ok(sdp) => match sdp {
                Some(sdp) => {
                    let connection = self.connection.clone();
                    Ok(Async::Ready((
                        sdp,
                        Box::new(move |answer| Box::new(RTCAcceptAnswer::new(connection, answer))),
                    )))
                }
                None => Err(Error::connection_failed()),
            },
            Err(err) => match err {
                TryRecvError::Empty => {
                    self.task.register();
                    Ok(Async::NotReady)
                }
                TryRecvError::Disconnected => panic!("Offer generation channel disconnected!"),
            },
        }
    }
}

struct RTCAnswer {
    task: Arc<AtomicTask>,
    receiver: Receiver<Option<String>>,
    connection: Reference,
}

impl RTCAnswer {
    fn new(offer: Offer) -> RTCAnswer {
        let task = Arc::new(AtomicTask::new());
        let (sender, receiver) = bounded(1);
        let answer = {
            let task = task.clone();
            move |answer: String| {
                console!(log, base64::encode(&answer));
                sender.send(Some(answer)).unwrap();
                task.notify();
            }
        };
        let peer_connection = js! {
            let connection = new RTCPeerConnection();
            connection.ondatachannel = () => {
                console.log("channel");
            };
            connection.setRemoteDescription(new RTCSessionDescription({sdp: @{offer}, type: "offer"})).then(() => {
                connection.createAnswer().then((answer) => {
                    connection.setLocalDescription(answer).catch(() => {
                        console.log("set local description failed");
                    }).then(() => {
                        @{answer}(answer.sdp);
                    });
                }).catch(() => {
                    console.log("create answer failed");
                });
            }).catch(() => {
                console.log("sdp parse failed");
            });
            return connection;
        }
        .into_reference()
        .unwrap();
        RTCAnswer {
            task,
            receiver,
            connection: peer_connection,
        }
    }
}

impl Future for RTCAnswer {
    type Item = (Answer, Box<dyn Negotiation + 'static>);
    type Error = Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.receiver.try_recv() {
            Ok(sdp) => Ok(Async::NotReady),
            Err(err) => match err {
                TryRecvError::Empty => {
                    self.task.register();
                    Ok(Async::NotReady)
                }
                TryRecvError::Disconnected => panic!("Offer generation channel disconnected!"),
            },
        }
    }
}

pub(crate) fn offer() -> impl Future<
    Item = (
        Offer,
        Box<dyn FnOnce(Answer) -> Box<dyn Negotiation + 'static> + Send + 'static>,
    ),
    Error = Error,
> {
    RTCOffer::new()
}

pub(crate) fn answer(
    offer: Offer,
) -> impl Future<Item = (Answer, Box<dyn Negotiation + 'static>), Error = Error> {
    RTCAnswer::new(offer)
}