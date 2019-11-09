use vessels::{
    core,
    core::{Executor, executor::Spawn},
    channel::IdChannel,
    format::{ApplyDecode, ApplyEncode, Cbor},
    log, OnTo,
    object,
    kind::Future
};

#[object]
pub trait ExampleObject {
    fn test(&self, message: String) -> Future<usize>;
}

pub struct Implementor;

impl ExampleObject for Implementor {
    fn test(&self, message: String) -> Future<usize> {
        Box::pin(async move {
            message.len()
        })
    }
}

fn main() {
    core::<dyn Executor>().unwrap().run(async move {
        let encoded = (Box::new(Implementor) as Box<dyn ExampleObject>)
            .on_to::<IdChannel>()
            .await
            .encode::<Cbor>();
        let decoded: Box<dyn ExampleObject> = encoded.decode::<IdChannel, Cbor>().await.unwrap();
        log!("{}", decoded.test("four".to_owned()).await);
    });
}
