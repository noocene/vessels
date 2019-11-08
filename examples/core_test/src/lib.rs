use vessels::{
    channel::IdChannel,
    log,
    core,
    core::{
        executor::Spawn,
        orchestrator::containers::{web::WebContainers, Containers},
        Executor,
    },
    format::{ApplyDecode, Cbor},
};
use wasm_bindgen::prelude::*;

const WASM_DATA: &'static [u8] =
    include_bytes!("../../../target/wasm32-unknown-unknown/debug/examples/test_vessel.wasm");

#[wasm_bindgen(start)]
pub fn main() {
    let mut executor = core::<dyn Executor>().unwrap();
    executor.spawn(async move {
        let mut containers = WebContainers;
        let module = containers.compile(WASM_DATA).await;
        let instance = containers.instantiate(&module).await;
        let data: String = instance.decode::<IdChannel, Cbor>().await.unwrap();
        log!("{}", data);
    });
}
