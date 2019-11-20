use vessels::{
    channel::IdChannel,
    core,
    core::{
        executor::Spawn,
        hal::{
            crypto::Rng,
            network::{Client, StaticCandidate},
        },
        orchestrator::containers::{web::WebContainers, Containers},
        Executor, Vessel,
    },
    format::{ApplyDecode, Cbor},
    log,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    core::<dyn Executor>().unwrap().run(async move {
        let mut network = Client::new().unwrap();
        let ufrag = [236, 97, 14];
        let pwd = [
            221, 45, 14, 120, 112, 243, 215, 166, 168, 89, 18, 184, 182, 112, 11, 34, 199, 155, 31,
            137, 228, 246, 137, 69,
        ];
        let fingerprint = [
            120, 10, 220, 61, 141, 117, 214, 168, 211, 147, 233, 45, 60, 120, 108, 126, 232, 219,
            165, 127, 127, 253, 62, 79, 9, 5, 147, 126, 109, 96, 21, 103,
        ];
        let connection = network
            .connect(StaticCandidate {
                addr: "127.0.0.1:61200".parse().unwrap(),
                pwd,
                fingerprint,
                ufrag,
            })
            .await
            .unwrap();
    });
}
