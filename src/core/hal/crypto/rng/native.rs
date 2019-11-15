use super::Rng as IRng;

use crate::kind::Future;

use ring::rand::{SecureRandom, SystemRandom};
use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref RNG: SystemRandom = SystemRandom::new();
}

pub struct Rng;

impl IRng for Rng {
    fn bytes(&mut self, len: usize) -> Future<Vec<u8>> {
        Box::pin(async move {
            let mut data = vec![0u8; len];
            RNG.fill(&mut data).unwrap();
            data
        })
    }
}

impl Rng {
    pub fn new() -> Box<dyn IRng> {
        Box::new(Rng)
    }
}
