use vessels::{crypto::SymmetricKey, executor};
use futures::Future;

#[macro_use]
extern crate stdweb;

fn main() {
    executor::spawn(SymmetricKey::new().and_then(|key| {
        key.encrypt("hello".as_bytes()).and_then(move |encrypted| {
            console!(log, &encrypted);
            key.decrypt(encrypted.as_slice()).and_then(|decrypted| {
                console!(log, unsafe { String::from_utf8_unchecked(decrypted) });
                Ok(())
            })
        })
    }).then(|_| Ok(())));
}
