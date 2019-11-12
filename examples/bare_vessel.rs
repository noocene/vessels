use vessels::{
    channel::IdChannel,
    core,
    core::{executor::Spawn, Executor, Vessel},
    format::{ApplyDecode, ApplyEncode, Cbor},
    log, OnTo,
};

fn main() {
    let tv: Vessel<String> = Box::new(|| Box::pin(async { "test".to_string() }));
    core::<dyn Executor>().unwrap().run(async move {
        let encoded = tv.on_to::<IdChannel>().await.encode::<Cbor>();
        let decoded: Vessel<String> = encoded.decode::<IdChannel, Cbor>().await.unwrap();
        log!("{:?}", decoded().await);
    });
}
