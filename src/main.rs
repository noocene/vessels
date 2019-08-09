use vessels::{crypto::primitives::{SymmetricKey, nonce_providers::Random}, executor};
use futures::Future;

fn main() {
    executor::run(SymmetricKey::new().and_then(|key: Box<dyn SymmetricKey<Random>>| {
        key.encrypt("hello".as_bytes()).and_then(move |encrypted| {
            println!("{:?}", &encrypted);
            key.decrypt(encrypted.as_slice()).and_then(|decrypted| {
                println!("{}", unsafe { String::from_utf8_unchecked(decrypted) });
                Ok(())
            })
        })
    }).then(|_| Ok(())));
}
