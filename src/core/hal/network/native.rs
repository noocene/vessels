use super::{ListenError, Peer as IPeer, Server as IServer};

use crate::kind::Stream;

use failure::Fail;
use futures::{
    future::err,
    stream::{once, unfold},
    task::{Context, Poll},
    FutureExt, Stream as IStream, StreamExt,
};
use glib::{ObjectExt, StaticType};
use gstreamer::{
    message::MessageView, ElementExt, ElementExtManual, GObjectExtManualGst, GstBinExt,
    GstObjectExt, Pipeline, Promise, State, Structure,
};
use gstreamer_webrtc::WebRTCSessionDescription;
use std::{net::SocketAddr, pin::Pin};

#[derive(Fail, Debug)]
pub enum GstListenError {
    #[fail(display = "missing gstreamer plugins {:?}", _0)]
    MissingPlugins(Vec<&'static str>),
}

pub struct Server;

type Peers = Stream<Result<Box<dyn IPeer>, ListenError>>;

struct PeersShim {
    stream: Peers,
    pipeline: Pipeline,
}

impl IStream for PeersShim {
    type Item = Result<Box<dyn IPeer>, ListenError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        self.stream.as_mut().poll_next(cx)
    }
}

impl Drop for PeersShim {
    fn drop(&mut self) {
        self.pipeline.set_state(State::Null).unwrap();
    }
}

impl IServer for Server {
    fn listen(&mut self, address: SocketAddr) -> Peers {
        Box::pin(
            async move {
                gstreamer::init().unwrap();
                let needed = ["nice", "webrtc", "dtls", "srtp", "rtpmanager"];
                let registry = gstreamer::Registry::get();
                let missing = needed
                    .iter()
                    .filter(|n| registry.find_plugin(n).is_none())
                    .cloned()
                    .collect::<Vec<_>>();
                if !missing.is_empty() {
                    return Box::pin(once(err(ListenError {
                        cause: GstListenError::MissingPlugins(missing).into(),
                    }))) as Peers;
                }
                let pipeline = Pipeline::new(Some("main"));
                let webrtcbin = gstreamer::ElementFactory::make("webrtcbin", None).unwrap();
                pipeline.add(&webrtcbin).unwrap();
                pipeline.set_state(State::Playing).unwrap();
                webrtcbin.set_property_from_str("bundle-policy", "max-bundle");
                let channel = webrtcbin
                    .emit(
                        "create-data-channel",
                        &[&("signalling".to_owned()), &None::<Structure>],
                    )
                    .unwrap()
                    .unwrap();
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
                let rtc = webrtcbin.clone();
                let promise = Promise::new_with_change_func(move |promise| {
                    let offer = promise
                        .get_reply()
                        .unwrap()
                        .get_value("offer")
                        .unwrap()
                        .get::<WebRTCSessionDescription>()
                        .unwrap();
                    let promise =
                        Promise::new_with_change_func(move |promise| match promise.wait() {
                            gstreamer::PromiseResult::Replied => println!("local description set"),
                            err => panic!(format!("{:?}", err)),
                        });
                    rtc.emit("set-local-description", &[&offer, &promise])
                        .unwrap();
                });
                webrtcbin
                    .emit("create-offer", &[&None::<Structure>, &promise])
                    .unwrap();
                Box::pin(PeersShim {
                    stream: Box::pin(unfold((), |mut unit| async move { None })),
                    pipeline,
                })
            }
            .into_stream()
            .flatten(),
        )
    }
}

impl Server {
    pub fn new() -> Box<dyn IServer> {
        Box::new(Server)
    }
}
