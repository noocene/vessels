use crate::network::mesh::{Negotiation, Peer};
use glib::{MainLoop, ObjectExt};
use gstreamer::{
    Element, ElementExt, ElementExtManual, ElementFactory, GObjectExtManualGst, GstBinExt,
    Pipeline, Promise, State, Structure,
};
use gstreamer_webrtc::WebRTCSessionDescription;

struct RTCNegotiation {}

pub(crate) fn new() {
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
}
