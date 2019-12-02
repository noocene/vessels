use super::Hash as IHash;

use crate::{core::data::Checksum, kind::Future};

use ring::digest::{digest, SHA256};

pub struct Hash;

impl IHash for Hash {
    fn hash(&self, data: Vec<u8>) -> Future<Checksum> {
        Box::pin(async move {
            let hash = digest(&SHA256, &data);
            let mut sum = [0u8; 32];
            sum.copy_from_slice(hash.as_ref());
            Checksum(sum)
        })
    }
}

impl Hash {
    pub fn new() -> Box<dyn IHash> {
        Box::new(Hash)
    }
}
