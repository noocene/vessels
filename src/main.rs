use futures::{lazy, Future, Sink, Stream};
use vitruvia::{
    executor,
    protocol::{protocol, Context},
};

#[protocol]
pub trait TestProtocol {
    fn test(&self) -> String;
    fn sec_test(&self);
}

struct Test;

impl TestProtocol for Test {
    fn test(&self) -> String {
        println!("test");
        "foo".to_owned()
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
        println!("{}", rem.test());
        Ok(())
    }));
}
