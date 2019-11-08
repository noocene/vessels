use vessels::{
    channel::IdChannel,
    core,
    core::{
        executor::Spawn,
        orchestrator::containers::{web::WebContainers, Containers},
        Executor,
    },
    format::{ApplyDecode, Cbor},
};
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

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
        log_1(&format!("{}", data).into());
    });
}
