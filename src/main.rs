use vessels::{
    channel::IdChannel,
    format::{ApplyDecode, ApplyEncode, Json},
    kind::using,
    Kind, OnTo,
};

use futures::executor::ThreadPool;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NotKind<T>(T);

#[derive(Kind, Debug)]
pub enum Test<T> {
    Test,
    Two(u32, String),
    Other {
        #[kind(using::Serde)]
        test: NotKind<T> 
    },
}

fn main() {
    let test = Test::Other { test: NotKind(0u32) };
    ThreadPool::new().unwrap().run(async move {
        let encoded = test.on_to::<IdChannel>().await.encode::<Json>();
        let decoded: Test<u32> = encoded.decode::<IdChannel, Json>().await.unwrap();
        println!("{:?}", decoded);
    })
}
