use vessels::{
    channel::IdChannel,
    core,
    core::{
        Vessel,
        executor::Spawn,
        orchestrator::containers::{native::NativeContainers, Containers},
        Executor,
    },
    format::{ApplyDecode, Cbor},
    log,
};

use std::fs::read;

pub fn main() {
    let binary = read("../../target/wasm32-unknown-unknown/debug/examples/test_vessel.wasm").unwrap();
    let mut executor = core::<dyn Executor>().unwrap();
    executor.run(async move {
        let mut containers = NativeContainers;
        let module = containers.compile(binary).await;
        let instance = containers.instantiate(&module).await;
        let data: Vessel<String> = instance.decode::<IdChannel, Cbor>().await.unwrap();
        log!("{}", data().await);
    });
}
