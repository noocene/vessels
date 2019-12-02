use super::Hash as IHash;

use crate::{core::data::Checksum, kind::Future, SyncSendAssert};

use js_sys::Uint8Array;
use wasm_bindgen_futures::JsFuture;

pub struct Hash;

impl IHash for Hash {
    fn hash(&self, mut data: Vec<u8>) -> Future<Checksum> {
        Box::pin(SyncSendAssert(Box::pin(async move {
            let mut sum = [0u8; 32];
            sum.copy_from_slice(
                &Uint8Array::new(
                    &JsFuture::from(
                        web_sys::window()
                            .unwrap()
                            .crypto()
                            .unwrap()
                            .subtle()
                            .digest_with_str_and_u8_array("SHA-256", &mut data)
                            .unwrap(),
                    )
                    .await
                    .unwrap(),
                )
                .to_vec(),
            );
            Checksum(sum)
        })))
    }
}

impl Hash {
    pub fn new() -> Box<dyn IHash> {
        Box::new(Hash)
    }
}
