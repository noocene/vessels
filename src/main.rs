use futures::Future;
use vessels::{
    crypto::primitives::SigningKeyPair,
    executor,
};

fn main() {
    executor::run(
        SigningKeyPair::new()
            .and_then(|(private_key, public_key)| {
                private_key
                    .sign("hello".as_bytes())
                    .and_then(move |signature| {
                        public_key
                            .verify("hello".as_bytes(), &signature)
                            .and_then(|result| {
                                println!("{:?}", result);
                                Ok(())
                            })
                    })
            })
            .then(|_| Ok(())),
    );
}
