use vessels::{
    channel::IdChannel,
    core::{
        orchestrator::containers::{web::WebContainers, Containers},
        run, Constructor,
    },
    format::{ApplyDecode, Cbor},
    log,
};
use wasm_bindgen::prelude::*;

const WASM_DATA: &'static [u8] =
    include_bytes!("../../../target/wasm32-unknown-unknown/debug/examples/test_vessel.wasm");

#[wasm_bindgen(start)]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    run(async move {
        let mut containers = WebContainers;
        let module = containers.compile(WASM_DATA).await;
        let instance = containers.instantiate(&module).await;
        let data: Constructor<String> = instance.decode::<IdChannel, Cbor>().await.unwrap();
        log!("{}", data().await);
    });
}
