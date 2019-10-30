use kinddev::{
    channel::{Channel, IdChannel},
    format::{ApplyDecode, ApplyEncode, Json},
    kind::{using, AsKind},
    Kind, OnTo,
};

use serde::{Deserialize, Serialize};

use futures::executor::ThreadPool;

#[derive(Serialize, Deserialize, Debug, Kind, Clone)]
#[kind(using::Serde)]
enum TestEnum {
    Yes(u32),
    No(String),
}

fn main() {
    let meme = vec!["test".to_owned(); 100];
    ThreadPool::new().unwrap().run(async move {
        let encoded = meme.on_to::<IdChannel>().await.encode::<Json>();
        let decoded: Vec<String> = encoded.decode::<IdChannel, Json>().await.unwrap();
        println!("{:?}", decoded)
    })
}
