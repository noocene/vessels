use futures::{Stream, lazy, Future};
use vitruvia::protocol::protocol;

#[protocol]
pub trait TestProtocol {
    fn test(&self);
    fn sec_test(&self);
}

struct Test;

impl TestProtocol for Test {
    fn test(&self) {
        println!("test");
    }
    fn sec_test(&self) {
        println!("sec_test");
    }
}

fn main() {
    let rem = TestProtocol::remote();
    let (sink, stream) = Test.into_protocol().split();
    tokio::run(lazy(move || {
        tokio::spawn(rem.clone().forward(sink).then(|_| Ok(())));
        tokio::spawn(stream.for_each(|i| {
            println!("{}", serde_json::to_string(&i).unwrap());
            Ok(())
        }));
        rem.test();
        rem.sec_test();
        rem.test();
        Ok(())
    }));
}
