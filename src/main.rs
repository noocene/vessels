use futures::Future as Fut;
use futures::{lazy, IntoFuture, Stream};
use vitruvia::{
    executor,
    protocol::{self, protocol, Future, Value},
};

#[derive(Value, Debug)]
pub enum TestEnum {
    Number(u8),
    Text(String),
}

#[protocol]
pub trait TestProtocol {
    fn test(&self) -> Future<TestEnum, ()>;
    fn sec_test(&self);
}

struct Test;

impl TestProtocol for Test {
    fn test(&self) -> Future<TestEnum, ()> {
        println!("test");
        protocol::Future::new(Ok(TestEnum::Number(8)).into_future())
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
