use crate::crypto::{
    self,
    primitives::{Nonce, NonceProvider, SymmetricKey},
};
use failure::Error;
use futures::{future::ok, lazy, Future};
use ring::aead::{Aad, LessSafeKey, Nonce as RingNonce, UnboundKey, AES_128_GCM};
use std::sync::{Arc, Mutex};

struct AESKeyState<T: NonceProvider + 'static> {
    key: LessSafeKey,
    key_bytes: [u8; 16],
    nonce_provider: T,
}

pub(crate) struct AESKey<T: NonceProvider + 'static> {
    state: Arc<Mutex<AESKeyState<T>>>,
}

impl<T: NonceProvider + 'static> AESKey<T> {
    pub(crate) fn new() -> impl Future<Item = Box<dyn SymmetricKey<T> + 'static>, Error = Error> {
        crypto::random(16).and_then(|bytes| {
            let mut a: [u8; 16] = Default::default();
            a.copy_from_slice(&bytes);
            let key: Box<dyn SymmetricKey<T>> = Box::new(AESKey {
                state: Arc::new(Mutex::new(AESKeyState {
                    key_bytes: a,
                    key: LessSafeKey::new(UnboundKey::new(&AES_128_GCM, &bytes).unwrap()),
                    nonce_provider: T::new(),
                })),
            });
            Ok(key)
        })
    }
    pub(crate) fn from_bytes(
        bytes: [u8; 16],
    ) -> impl Future<Item = Box<dyn SymmetricKey<T> + 'static>, Error = Error> {
        let bytes = bytes.to_owned();
        lazy(move || {
            let mut a: [u8; 16] = Default::default();
            a.copy_from_slice(&bytes);
            let key: Box<dyn SymmetricKey<T>> = Box::new(AESKey {
                state: Arc::new(Mutex::new(AESKeyState {
                    key_bytes: a,
                    key: LessSafeKey::new(UnboundKey::new(&AES_128_GCM, &bytes).unwrap()),
                    nonce_provider: T::new(),
                })),
            });
            Ok(key)
        })
    }
}

impl<T: NonceProvider + 'static> SymmetricKey<T> for AESKey<T> {
    fn as_bytes(&self) -> Box<dyn Future<Item = [u8; 16], Error = Error> + Send> {
        Box::new(ok(self.state.lock().unwrap().key_bytes))
    }
    fn encrypt(&self, data: &'_ [u8]) -> Box<dyn Future<Item = Vec<u8>, Error = Error> + Send> {
        let state = self.state.clone();
        let data = data.to_owned();
        Box::new(lazy(move || {
            let mut data = data;
            let mut state = state.lock().unwrap();
            let iv = state.nonce_provider.next_encrypt();
            state
                .key
                .seal_in_place_append_tag(
                    RingNonce::assume_unique_for_key(*iv.as_ref()),
                    Aad::empty(),
                    &mut data,
                )
                .unwrap();
            iv.after_encrypt(&mut data);
            Ok(data)
        }))
    }
    fn decrypt(&self, data: &'_ [u8]) -> Box<dyn Future<Item = Vec<u8>, Error = Error> + Send> {
        let state = self.state.clone();
        let data = data.to_owned();
        Box::new(lazy(move || {
            let mut data = data;
            let mut state = state.lock().unwrap();
            let iv = state.nonce_provider.next_decrypt(&mut data);
            Ok(state
                .key
                .open_in_place(
                    RingNonce::assume_unique_for_key(iv),
                    Aad::empty(),
                    &mut data,
                )
                .unwrap()
                .to_owned())
        }))
    }
}
