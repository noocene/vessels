use vessels::{kind::Future, object, OnTo, channel::IdChannel, ApplyEncode, ApplyDecode, format::Json, Kind};

use futures::executor::ThreadPool;

#[object]
trait Test<T: Kind> {
    fn test(&self, hello: T) -> Future<u32>;
}

struct Shim;

impl<T: Kind> Test<T> for Shim {
    fn test(&self, _: T) -> Future<u32> {
        Box::pin(async move {
            20
        })
    }
}

fn main() {
    let test: Box<dyn Test<String>> = Box::new(Shim);
    ThreadPool::new().unwrap().run(async move {
        let encoded = test.on_to::<IdChannel>().await.encode::<Json>();
        let decoded: Box<dyn Test<String>> = encoded.decode::<IdChannel, Json>().await.unwrap();
        println!("{:?}", decoded.test("hello".to_string()).await);
    })
}