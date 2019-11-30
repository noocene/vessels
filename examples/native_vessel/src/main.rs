use vessels::{
    channel::IdChannel,
    core::{
        orchestrator::containers::{native::NativeContainers, Containers},
        run, Constructor, Core
    },
    format::{ApplyDecode, Cbor},
    log,
};

use std::fs::read;

pub fn main() {
    let binary =
        read("../../target/wasm32-unknown-unknown/debug/examples/test_vessel.wasm").unwrap();
    run(async move {
        let mut containers = NativeContainers;
        let module = containers.compile(binary).await;
        let instance = containers.instantiate(&module).await;
        let data: Constructor<String> = instance.decode::<IdChannel, Cbor>().await.unwrap();
        log!("{}", data(Core::new().as_handle()).await);
    });
}
