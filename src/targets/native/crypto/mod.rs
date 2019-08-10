use failure::Error;
use futures::{lazy, Future};
use ring::rand::{SecureRandom, SystemRandom};
use std::sync::RwLock;

pub(crate) mod primitives;

lazy_static! {
    pub(crate) static ref RNG: RwLock<SystemRandom> = RwLock::new(SystemRandom::new());
}

pub(crate) fn random(bytes: u32) -> impl Future<Item = Vec<u8>, Error = Error> {
    lazy(move || {
        let mut data = vec![0u8; bytes as usize];
        RNG.read().unwrap().fill(&mut data).unwrap();
        Ok(data)
    })
}
