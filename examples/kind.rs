use vessels::{
    channel::IdChannel,
    core::run,
    format::{ApplyDecode, ApplyEncode, Cbor},
    log, OnTo,
};

fn main() {
    run(async move {
        let encoded = "Hello there"
            .to_owned()
            .on_to::<IdChannel>()
            .await
            .encode::<Cbor>();
        let decoded: String = encoded.decode::<IdChannel, Cbor>().await.unwrap();
        log!("{:?}", decoded);
    });
}
