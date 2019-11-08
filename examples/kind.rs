use vessels::{
    channel::IdChannel,
    format::{ApplyDecode, ApplyEncode, Cbor},
    log, OnTo,
};

use vessels::futures::executor::LocalPool;

fn main() {
    LocalPool::new().run_until(async move {
        let encoded = "Hello there"
            .to_owned()
            .on_to::<IdChannel>()
            .await
            .encode::<Cbor>();
        let decoded: String = encoded.decode::<IdChannel, Cbor>().await.unwrap();
        log!("{:?}", decoded);
    });
}
