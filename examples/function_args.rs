use vessels::{
    channel::IdChannel,
    core,
    core::Executor,
    format::{ApplyDecode, ApplyEncode, Cbor},
    kind::Future,
    log, OnTo,
};

type Call = Box<dyn Fn(Vec<u8>) -> Future<String> + Send + Sync>;

fn main() {
    let call: Call = Box::new(|data| Box::pin(async move { format!("{:?}", data) }));

    core::<Executor>().unwrap().run(async move {
        let encoded = call.on_to::<IdChannel>().await.encode::<Cbor>();
        let decoded: Call = encoded.decode::<IdChannel, Cbor>().await.unwrap();
        log!("{}", (decoded)(vec![5, 6, 7]).await);
    });
}
