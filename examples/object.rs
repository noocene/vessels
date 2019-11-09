use vessels::{
    channel::IdChannel,
    core,
    core::{executor::Spawn, Executor},
    format::{ApplyDecode, ApplyEncode, Cbor},
    kind::Future,
    log, object, Kind, OnTo,
};

use std::fmt::Display;

#[object]
pub trait ExampleObject<T: Kind + Display> {
    fn test(&self, message: T) -> Future<usize>;
}

pub struct Implementor;

impl<T: Kind + Display> ExampleObject<T> for Implementor {
    fn test(&self, message: T) -> Future<usize> {
        Box::pin(async move { format!("{}", message).len() })
    }
}

fn main() {
    core::<dyn Executor>().unwrap().run(async move {
        let encoded = (Box::new(Implementor) as Box<dyn ExampleObject<String>>)
            .on_to::<IdChannel>()
            .await
            .encode::<Cbor>();
        let decoded: Box<dyn ExampleObject<String>> =
            encoded.decode::<IdChannel, Cbor>().await.unwrap();
        log!("{}", decoded.test("four".to_owned()).await);
    });
}
