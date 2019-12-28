use vessels::{
    channel::IdChannel,
    core::run,
    format::{ApplyDecode, ApplyEncode, Cbor},
    kind::Infallible,
    log, object, Kind, OnTo,
};

use std::fmt::Display;

#[object]
pub trait ExampleObject<T: Kind + Display> {
    fn test(&self, message: T) -> Infallible<usize>;
}

pub struct Implementor;

impl<T: Kind + Display> ExampleObject<T> for Implementor {
    fn test(&self, message: T) -> Infallible<usize> {
        Box::pin(async move { Ok(format!("{}", message).len()) })
    }
}

fn main() {
    run(async move {
        let encoded = (Box::new(Implementor) as Box<dyn ExampleObject<String>>)
            .on_to::<IdChannel>()
            .await
            .encode::<Cbor>();
        let decoded: Box<dyn ExampleObject<String>> =
            encoded.decode::<IdChannel, Cbor>().await.unwrap();
        log!("{}", decoded.test("four".to_owned()).await.unwrap());
    });
}
