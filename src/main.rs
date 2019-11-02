use vessels::{
    channel::IdChannel,
    format::{ApplyDecode, ApplyEncode, Json},
    kind::Iterator,
    Kind, OnTo,
};

use futures::executor::ThreadPool;

#[derive(Kind, Debug)]
pub enum Test {
    Test,
    Two(u32, String),
    Other(Iterator<Vec<u32>>)
}

fn main() {
    let func = Test::Other(Iterator(vec![0; 5]));
    ThreadPool::new().unwrap().run(async move {
        let encoded = func.on_to::<IdChannel>().await.encode::<Json>();
        let decoded: Test = encoded.decode::<IdChannel, Json>().await.unwrap();
        println!("{:?}", decoded);
    })
}
