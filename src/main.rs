use kinddev::{
    channel::{Channel, IdChannel},
    format::{Decode, Encode, Json},
    kind::{using, AsKind},
    Kind, OnTo,
};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use futures::{executor::ThreadPool, future::ready, FutureExt, StreamExt, TryFutureExt};

#[derive(Serialize, Deserialize, Debug, Kind, Clone)]
#[kind(using::Serde)]
enum TestEnum {
    Yes(u32),
    No(String),
}

fn main() {
    let mut meme: HashMap<(i32, String), TestEnum> = HashMap::new();
    meme.insert((69, "nice".to_owned()), TestEnum::Yes(5021));
    meme.insert(
        (-1, "not nice".to_owned()),
        TestEnum::No("unepic".to_owned()),
    );
    ThreadPool::new().unwrap().run(
        meme.on_to::<IdChannel>()
            .map(Json::encode)
            .map(|c| c.inspect(|item| println!("{}", item)))
            .map(Json::decode::<IdChannel>)
            .flatten()
            .unwrap_or_else(|e| panic!(e))
            .then(|item: HashMap<(i32, String), TestEnum>| {
                println!("{:?}", item);
                ready(())
            }),
    )
}
