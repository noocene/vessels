use super::Hasher as IHasher;

use crate::{core::data::Checksum, kind::Infallible};

use ring::digest::{digest, SHA256};

pub struct Hasher;

impl IHasher for Hasher {
    fn hash(&self, data: Vec<u8>) -> Infallible<Checksum> {
        Box::pin(async move {
            let hash = digest(&SHA256, &data);
            let mut sum = [0u8; 32];
            sum.copy_from_slice(hash.as_ref());
            Ok(Checksum(sum))
        })
    }
}

impl Hasher {
    pub fn new() -> Box<dyn IHasher> {
        Box::new(Hasher)
    }
}
