use kinddev::{
    channel::{Channel, IdChannel},
    format::{Decode, Encode, Json},
    kind::{using, AsKind},
    Kind, OnTo,
};

use serde::{Deserialize, Serialize};

use futures::{executor::ThreadPool, future::ready, FutureExt, StreamExt, TryFutureExt};

#[derive(Serialize, Deserialize, Debug, Kind, Clone)]
#[kind(using::Serde)]
enum TestEnum {
    Yes(u32),
    No(String),
}

fn main() {
    let meme: Vec<TestEnum> = vec![TestEnum::No("ok this is epic".to_owned()); 1000];
    ThreadPool::new().unwrap().run(
        meme.on_to::<IdChannel>()
            .map(Json::encode)
            .map(|c| c.inspect(|item| println!("{}", item)))
            .map(Json::decode::<IdChannel>)
            .flatten()
            .unwrap_or_else(|e| panic!(e))
            .then(|item: Vec<TestEnum>| {
                println!("{:?}", item);
                ready(())
            }),
    )
}
