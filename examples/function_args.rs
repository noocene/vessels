use vessels::{
    channel::IdChannel,
    core,
    core::{executor::Spawn, Executor},
    format::{ApplyDecode, ApplyEncode, Json},
    kind::Future,
    log, OnTo,
};

type Call = Box<dyn Fn(Vec<u8>) -> Future<String> + Send + Sync>;

use futures::StreamExt;

fn main() {
    let call: Call = Box::new(|data| Box::pin(async move { format!("{:?}", data) }));

    core::<dyn Executor>().unwrap().run(async move {
        let encoded = call
            .on_to::<IdChannel>()
            .await
            .encode::<Json>()
            .inspect(|item| println!("{}", item));
        let decoded: Call = encoded.decode::<IdChannel, Json>().await.unwrap();
        log!("{}", (decoded)(vec![5, 6, 7]).await);
    });
}
