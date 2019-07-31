use futures::Future as Fut;
use futures::{lazy, IntoFuture, Stream};
use vitruvia::{
    executor,
    protocol::{self, protocol, Future},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Tester<T> {
    Yes,
    No,
    Value(T),
}

#[protocol]
pub trait TestProtocol {
    fn test(&self) -> Future<Tester<u8>, ()>;
    fn sec_test(&self);
}

struct Test;

impl TestProtocol for Test {
    fn test(&self) -> Future<Tester<u8>, ()> {
        println!("test");
        protocol::Future::new(Ok(Tester::Value(3)).into_future())
    }
    fn sec_test(&self) {
        println!("sec_test");
    }
}

fn main() {
    let rem = TestProtocol::remote();
    let (rsink, rstream) = rem.clone().split();
    let (sink, stream) = Test.into_protocol().split();
    executor::run(lazy(move || {
        executor::spawn(rstream.forward(sink).then(|_| Ok(())));
        executor::spawn(stream.forward(rsink).then(|_| Ok(())));
        println!("{:?}", rem.test().wait());
        Ok(())
    }));
}
