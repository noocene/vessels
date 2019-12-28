use vessels::{
    channel::IdChannel,
    core::run,
    format::{ApplyDecode, ApplyEncode, Cbor},
    kind::Infallible,
    log, OnTo,
};

type Call = Box<dyn Fn(Vec<u8>) -> Infallible<String> + Send + Sync>;

fn main() {
    let call: Call = Box::new(|data| Box::pin(async move { Ok(format!("{:?}", data)) }));

    run(async move {
        let encoded = call.on_to::<IdChannel>().await.encode::<Cbor>();
        let decoded: Call = encoded.decode::<IdChannel, Cbor>().await.unwrap();
        log!("{}", (decoded)(vec![5, 6, 7]).await.unwrap());
    });
}
