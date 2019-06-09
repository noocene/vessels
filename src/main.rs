use futures::{Sink, Stream};
use vitruvia::{executor::run, protocol::protocol};

/*use stdweb::unstable::TryInto;

#[macro_use]
extern crate stdweb;*/

/*fn main() {
    let connection = mesh::offer().map_err(|e| ()).and_then(|(offer, answer)| {
        console!(log, encode(&offer));
        let a: String = js! {
            return prompt("answer", "");
        }
        .try_into()
        .unwrap();
        answer(
            std::str::from_utf8(base64::decode(&a).unwrap().as_slice())
                .unwrap()
                .to_owned(),
        )
        .map_err(|e| ())
        .and_then(|_| Ok(()))
    });

    run(connection);
}*/

#[protocol]
pub trait Hello {
    fn data(&mut self, m: String, f: f64);
}

struct TestHello;

impl Hello for TestHello {
    fn data(&mut self, m: String, f: f64) {
        println!("{}", m);
    }
}

fn main() {
    let mut hello_remote = Hello::remote();
    hello_remote.data("test".to_owned(), 10.0);
    run(hello_remote.for_each(|call| {
        let serialized = serde_json::to_string(&call).unwrap();
        let deserialized = serde_json::from_str(&serialized).unwrap();
        let hello = TestHello;
        hello.send(deserialized);
        Ok(())
    }));
}
