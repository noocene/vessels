use futures::{lazy, Sink, Stream, IntoFuture};
use futures::Future as Fut;
use vitruvia::{
    executor,
    protocol::{protocol, Future, self, Context},
};

#[protocol]
pub trait TestProtocol {
    fn test(&self) -> Future<String, ()>;
    fn sec_test(&self);
}

struct Test;

impl TestProtocol for Test {
    fn test(&self) -> Future<String, ()> {
        println!("test");
        protocol::Future::new(Ok("foo".to_owned()).into_future())
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
