use futures::Future;

use base64::encode;
use vitruvia::protocol::protocol;

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

fn main() {
    //let hello_remote = Hello::remote();
}
