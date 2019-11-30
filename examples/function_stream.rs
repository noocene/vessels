use vessels::{
    channel::IdChannel,
    core::run,
    format::{ApplyDecode, ApplyEncode, Cbor},
    kind::Stream,
    log, OnTo,
};

use futures::{stream::iter, StreamExt};

type Call = Box<dyn Fn() -> Stream<u8> + Send + Sync>;

fn main() {
    let call: Call = Box::new(|| Box::pin(iter(1..10)));

    run(async move {
        let encoded = call.on_to::<IdChannel>().await.encode::<Cbor>();
        let decoded: Call = encoded.decode::<IdChannel, Cbor>().await.unwrap();
        let mut stream = (decoded)();
        while let Some(item) = stream.next().await {
            log!("{}", item);
        }
    });
}
