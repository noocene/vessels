use kinddev::{
    channel::{Channel, IdChannel},
    format::{ApplyDecode, ApplyEncode, Json},
    kind::{using, AsKind},
    Kind, OnTo,
};

use serde::{Deserialize, Serialize};

use futures::{executor::ThreadPool, stream::{BoxStream,iter}, StreamExt};

#[derive(Serialize, Deserialize, Debug, Kind, Clone)]
#[kind(using::Serde)]
enum TestEnum {
    Yes(u32),
    No(String),
}

fn main() {
    let meme: BoxStream<'static, String> = Box::pin(iter(vec!["test".to_owned(); 100].into_iter()));
    ThreadPool::new().unwrap().run(async move {
        let encoded = meme.on_to::<IdChannel>().await.encode::<Json>();
        let mut decoded: BoxStream<'static, String> = encoded.decode::<IdChannel, Json>().await.unwrap();
        while let Some(item) = decoded.next().await {
            println!("{}", item);
        }
    })
}
