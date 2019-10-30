use kinddev::{
    channel::{Channel, IdChannel},
    format::{ApplyDecode, ApplyEncode, Json},
    kind::{using, AsKind,Iterator},
    Kind, OnTo,
};

use serde::{Deserialize, Serialize};

use futures::{executor::ThreadPool};

#[derive(Serialize, Deserialize, Debug, Kind, Clone)]
#[kind(using::Serde)]
enum TestEnum {
    Yes(u32),
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
