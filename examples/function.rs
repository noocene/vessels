use vessels::{
    channel::IdChannel,
    core::run,
    format::{ApplyDecode, ApplyEncode, Cbor},
    kind::Infallible,
    log, OnTo,
};

type Call = Box<dyn Fn() -> Infallible<String> + Send + Sync>;

fn main() {
    let call: Call = Box::new(|| Box::pin(async move { Ok("hello".to_owned()) }));

    run(async move {
        let encoded = call.on_to::<IdChannel>().await.encode::<Cbor>();
        let decoded: Call = encoded.decode::<IdChannel, Cbor>().await.unwrap();
        log!("{}", (decoded)().await.unwrap());
    });
}
