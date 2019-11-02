use vessels::{
    channel::IdChannel,
    format::{ApplyDecode, ApplyEncode, Json},
    kind::using,
    Kind, OnTo,
};

use futures::executor::ThreadPool;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NotKind;

#[derive(Kind, Debug)]
pub enum Test {
    Test,
    Two(u32, String),
    Other(#[kind(using::Serde)] NotKind),
}

fn main() {
    let test = Test::Other(NotKind);
    ThreadPool::new().unwrap().run(async move {
        let encoded = test.on_to::<IdChannel>().await.encode::<Json>();
        let decoded: Test = encoded.decode::<IdChannel, Json>().await.unwrap();
        println!("{:?}", decoded);
    })
}
