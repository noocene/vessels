use super::RNG;
use crate::crypto::{
    self,
    primitives::{Nonce, NonceProvider, SigningKey, SymmetricKey, VerifyingKey},
};
use failure::Error;
use futures::{future::ok, lazy, Future};
use ring::{
    aead::{Aad, LessSafeKey, Nonce as RingNonce, UnboundKey, AES_128_GCM},
    pkcs8::Document,
    signature::{
        EcdsaKeyPair, KeyPair, UnparsedPublicKey, ECDSA_P256_SHA256_FIXED,
        ECDSA_P256_SHA256_FIXED_SIGNING,
    },
};
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

struct ECDSAPrivateKeyState {
    key: EcdsaKeyPair,
    key_data: Vec<u8>,
}

pub(crate) struct ECDSAPrivateKey {
    state: Arc<Mutex<ECDSAPrivateKeyState>>,
}

impl ECDSAPrivateKey {
    pub(crate) fn from_bytes(
        data: &'_ [u8],
    ) -> impl Future<Item = Box<dyn SigningKey + 'static>, Error = Error> {
        let data = data.to_owned();
        lazy(move || {
            let key = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, data.as_slice())
                .unwrap();
            let private_key: Box<dyn SigningKey> = Box::new(ECDSAPrivateKey {
                state: Arc::new(Mutex::new(ECDSAPrivateKeyState {
                    key,
                    key_data: data,
                })),
            });
            Ok(private_key)
        })
    }
}

impl SigningKey for ECDSAPrivateKey {
    fn sign(&self, data: &'_ [u8]) -> Box<dyn Future<Item = Vec<u8>, Error = Error> + Send> {
        let state = self.state.clone();
        let data = data.to_owned();
        Box::new(lazy(move || {
            let state = state.lock().unwrap();
            Ok(state
                .key
                .sign(&*RNG.read().unwrap(), data.as_slice())
                .unwrap()
                .as_ref()
                .to_owned())
        }))
    }
    fn as_bytes(&self) -> Box<dyn Future<Item = Vec<u8>, Error = Error> + Send> {
        Box::new(ok(Vec::from(self.state.lock().unwrap().key_data.clone())))
    }
}

struct ECDSAPublicKeyState {
    key: UnparsedPublicKey<Vec<u8>>,
    key_data: Vec<u8>,
}

pub(crate) struct ECDSAPublicKey {
    state: Arc<Mutex<ECDSAPublicKeyState>>,
}

impl ECDSAPublicKey {
    pub(crate) fn from_bytes(
        data: &'_ [u8],
    ) -> impl Future<Item = Box<dyn VerifyingKey + 'static>, Error = Error> {
        let data = data.to_owned();
        lazy(move || {
            let public_key: Box<dyn VerifyingKey> = Box::new(ECDSAPublicKey {
                state: Arc::new(Mutex::new(ECDSAPublicKeyState {
                    key: UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, data.clone()),
                    key_data: data,
                })),
            });
            Ok(public_key)
        })
    }
}

impl VerifyingKey for ECDSAPublicKey {
    fn verify(
        &self,
        data: &'_ [u8],
        signature: &'_ [u8],
    ) -> Box<dyn Future<Item = bool, Error = Error> + Send> {
        let state = self.state.clone();
        let data = data.to_owned();
        let signature = signature.to_owned();
        Box::new(lazy(move || {
            let state = state.lock().unwrap();
            Ok(state
                .key
                .verify(data.as_slice(), signature.as_slice())
                .is_ok())
        }))
    }
    fn as_bytes(&self) -> Box<dyn Future<Item = Vec<u8>, Error = Error> + Send> {
        Box::new(ok(self.state.lock().unwrap().key_data.clone()))
    }
}

pub(crate) struct ECDSAKeyPair;

impl ECDSAKeyPair {
    pub(crate) fn new() -> impl Future<
        Item = (
            Box<dyn SigningKey + 'static>,
            Box<dyn VerifyingKey + 'static>,
        ),
        Error = Error,
    > {
        lazy(|| {
            let key_data = EcdsaKeyPair::generate_pkcs8(
                &ECDSA_P256_SHA256_FIXED_SIGNING,
                &*RNG.read().unwrap(),
            )
            .unwrap();
            let key = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, key_data.as_ref())
                .unwrap();
            let public_key_data = key.public_key().as_ref().to_owned();
            let public_key: Box<dyn VerifyingKey> = Box::new(ECDSAPublicKey {
                state: Arc::new(Mutex::new(ECDSAPublicKeyState {
                    key: UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, public_key_data.clone()),
                    key_data: public_key_data,
                })),
            });
            let private_key: Box<dyn SigningKey> = Box::new(ECDSAPrivateKey {
                state: Arc::new(Mutex::new(ECDSAPrivateKeyState {
                    key,
                    key_data: Vec::from(key_data.as_ref()),
                })),
            });
            Ok((private_key, public_key))
        })
    }
}
