use ring::aead::{AES_128_GCM, LessSafeKey, UnboundKey, Aad, Nonce as RingNonce};
use crate::crypto::{primitives::{NonceProvider, SymmetricKey, Nonce}, self};
use futures::{future::ok, Future, lazy};
use failure::Error;
use std::sync::Mutex;

pub(crate) struct AESKey<T: NonceProvider + 'static> {
    key: LessSafeKey,
    key_bytes: [u8; 16],
    nonce_provider: Mutex<T>,
}

impl<T: NonceProvider + 'static> AESKey<T> {
    pub(crate) fn new() -> impl Future<Item = Box<dyn SymmetricKey<T> + 'static>, Error = Error> {
        crypto::random(16).and_then(|bytes| {
            let mut a: [u8; 16] = Default::default();
            a.copy_from_slice(&bytes);
            let key: Box<dyn SymmetricKey<T>> = Box::new(AESKey {
                key_bytes: a,
                key: LessSafeKey::new(UnboundKey::new(&AES_128_GCM, &bytes).unwrap()),
                nonce_provider: Mutex::new(T::new())
            });
            Ok(key)
        })
    }
    pub(crate) fn from_bytes(bytes: [u8; 16]) -> impl Future<Item = Box<dyn SymmetricKey<T> + 'static>, Error = Error> {
        let bytes = bytes.to_owned();
        lazy(move || {
            let mut a: [u8; 16] = Default::default();
            a.copy_from_slice(&bytes);
            let key: Box<dyn SymmetricKey<T>> = Box::new(AESKey {
                key_bytes: a,
                key: LessSafeKey::new(UnboundKey::new(&AES_128_GCM, &bytes).unwrap()),
                nonce_provider: Mutex::new(T::new())
            });
            Ok(key)
        })
    }
}

impl<T: NonceProvider + 'static> SymmetricKey<T> for AESKey<T> {
    fn as_bytes(&self) -> Box<dyn Future<Item = [u8; 16], Error = Error> + Send> {
        Box::new(ok(self.key_bytes))
    }
    fn encrypt(&self, data: &'_ [u8]) -> Box<dyn Future<Item = Vec<u8>, Error = Error> + Send> {
        let mut data = data.to_owned().clone();
        let iv = self.nonce_provider.lock().unwrap().next_encrypt();
        self.key.seal_in_place_append_tag(RingNonce::assume_unique_for_key(*iv.as_ref()), Aad::empty(), &mut data).unwrap();
        iv.after_encrypt(&mut data);
        Box::new(ok(data))
    }
    fn decrypt(&self, data: &'_ [u8]) -> Box<dyn Future<Item = Vec<u8>, Error = Error> + Send> {
        let mut data = data.to_owned();
        let iv = self.nonce_provider.lock().unwrap().next_decrypt(&mut data);
        Box::new(ok(self.key.open_in_place(RingNonce::assume_unique_for_key(iv), Aad::empty(), &mut data).unwrap().to_owned()))
    }
}