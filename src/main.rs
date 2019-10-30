use kinddev::{
    channel::{Channel, IdChannel},
    format::{ApplyDecode, ApplyEncode, Json},
    kind::{using, AsKind},
    Kind, OnTo,
};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use futures::executor::ThreadPool;

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
    ThreadPool::new().unwrap().run(async move {
        let encoded = meme.on_to::<IdChannel>().await.encode::<Json>();
        let decoded: HashMap<(i32, String), TestEnum> =
            encoded.decode::<IdChannel, Json>().await.unwrap();
        println!("{:?}", decoded)
    })
}
