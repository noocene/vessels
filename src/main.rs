use kinddev::{
    channel::{Channel, IdChannel},
    format::{ApplyDecode, ApplyEncode, Json},
    kind::{using, AsKind},
    Kind, OnTo,
};

use serde::{Deserialize, Serialize};

use futures::{executor::ThreadPool, future::BoxFuture};

#[derive(Serialize, Deserialize, Debug, Kind, Clone)]
#[kind(using::Serde)]
enum TestEnum {
    Yes(u32),
    No(String),
}

type Adder = Box<dyn FnOnce(u32, u32) -> BoxFuture<'static, u32> + Send + Sync>;

fn main() {
    let func: Adder = Box::new(|a, b| Box::pin(async move { a + b }));
    ThreadPool::new().unwrap().run(async move {
        let encoded = func.on_to::<IdChannel>().await.encode::<Json>();
        let add: Adder = encoded.decode::<IdChannel, Json>().await.unwrap();
        println!("{}", add(20, 35).await);
    })
}
