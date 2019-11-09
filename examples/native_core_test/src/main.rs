use vessels::{
    channel::IdChannel,
    core,
    core::{
        executor::Spawn,
        orchestrator::containers::{native::NativeContainers, Containers},
        Executor,
    },
    format::{ApplyDecode, Cbor},
    log,
};

const WASM_DATA: &'static [u8] =
    include_bytes!("../../../target/wasm32-unknown-unknown/release/examples/test_vessel.wasm");

pub fn main() {
    let mut executor = core::<dyn Executor>().unwrap();
    executor.spawn(async move {
        let mut containers = NativeContainers;
        let module = containers.compile(WASM_DATA).await;
        let instance = containers.instantiate(&module).await;
        let data: String = instance.decode::<IdChannel, Cbor>().await.unwrap();
        log!("{}", data);
    });
    core::event_loop();
}
