use vessels::{
    channel::IdChannel,
    core::{
        orchestrator::containers::{native::NativeContainers, Containers},
        run, Constructor, Core,
    },
    format::{ApplyDecode, Cbor},
    kind::Future,
    log,
};

use std::fs::read;

pub struct Tester;

impl test_vessel::Test for Tester {
    fn test(&self, message: String) -> Future<String> {
        Box::pin(async move {
            format!("passed through: {}", message)
        })
    }
}

pub fn main() {
    let binary =
        read("../../target/wasm32-unknown-unknown/debug/test_vessel.wasm").unwrap();
    run(async move {
        let mut containers = NativeContainers;
        let module = containers.compile(binary).await;
        let instance = containers.instantiate(&module).await;
        let data: Constructor<String> = instance.decode::<IdChannel, Cbor>().await.unwrap();
        let mut core = Core::new();
        core.register(|| Box::new(Tester) as Box<dyn test_vessel::Test>);
        log!("{}", data(core.into_handle()).await);
    });
}
