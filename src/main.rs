use vessels::{
    channel::IdChannel,
    format::{ApplyDecode, ApplyEncode, Json},
    kind::Iterator,
    Kind, OnTo,
};

use futures::executor::ThreadPool;

#[derive(Kind, Debug)]
pub struct Test {
    test: Iterator<Vec<String>>,
}

fn main() {
    let func = Test {
        test: Iterator(vec!["test".to_owned(); 10]),
    };
    ThreadPool::new().unwrap().run(async move {
        let encoded = func.on_to::<IdChannel>().await.encode::<Json>();
        let decoded: Test = encoded.decode::<IdChannel, Json>().await.unwrap();
        println!("{:?}", decoded);
    })
}
