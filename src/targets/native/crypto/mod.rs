use failure::Error;
use futures::{lazy, Future};
use ring::rand::{SecureRandom, SystemRandom};

pub(crate) mod primitives;

lazy_static! {
    static ref RNG: SystemRandom = SystemRandom::new();
}

pub(crate) fn random(bytes: u32) -> impl Future<Item = Vec<u8>, Error = Error> {
    lazy(move || {
        let mut data = vec![0u8; bytes as usize];
        RNG.fill(&mut data).unwrap();
        Ok(data)
    })
}
