use futures::Future;

use base64::encode;
use vitruvia::{executor::run, network::mesh};

use stdweb::unstable::TryInto;

#[macro_use]
extern crate stdweb;

fn main() {
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
}
