use vessels::{Kind, channel::IdChannel, format::{Json, ApplyEncode, ApplyDecode}, OnTo, kind::Iterator};

use futures::{executor::ThreadPool};

#[derive(Kind, Debug)]
pub enum Test {
    Item(Iterator<Vec<u32>>, u64),
    StructStyle { test: u32 },
    Other(String),
    Empty,
}

fn main() {
    let func = Test::Item(Iterator(vec![4, 5]), 4);
    ThreadPool::new().unwrap().run(async move {
        let encoded = func.on_to::<IdChannel>().await.encode::<Json>();
        let decoded: Test = encoded.decode::<IdChannel, Json>().await.unwrap();
        println!("{:?}", decoded);
    })
}
