use vessels::{
    channel::IdChannel,
    format::{ApplyDecode, ApplyEncode, Json},
    kind::{using, Iterator}, Kind, OnTo,
};

use serde::{Deserialize, Serialize};

use futures::executor::ThreadPool;

#[derive(Serialize, Deserialize, Debug, Kind, Clone)]
#[kind(using::Serde)]
enum TestEnum<T> {
    Yes(T),
    No(String),
}

fn main() {
    let func = Iterator(vec!["test".to_owned(); 10]);
    ThreadPool::new().unwrap().run(async move {
        let encoded = func.on_to::<IdChannel>().await.encode::<Json>();
        let decoded: Iterator<Vec<String>> = encoded.decode::<IdChannel, Json>().await.unwrap();
        println!("{:?}", decoded);
    })
}
