use crate::{
    crypto::primitives::{Nonce, NonceProvider, SigningKey, SymmetricKey, VerifyingKey},
    executor,
};
use failure::Error;
use futures::{lazy, sync::mpsc::channel, Future, Sink, Stream};
use std::sync::Mutex;

use wasm_bindgen::{prelude::*, JsCast};

pub(crate) struct AESKey<T: NonceProvider> {
    key: web_sys::CryptoKey,
    nonce_provider: Mutex<T>,
}

impl<T: NonceProvider> SymmetricKey<T> for AESKey<T> {
    fn encrypt(&self, data: &'_ [u8]) -> Box<dyn Future<Item = Vec<u8>, Error = Error>>
    where
        Self: Sized,
    {
        let (sender, receiver) = channel(0);
        let iv = self.nonce_provider.lock().unwrap().next_encrypt();
        let iv_data: &[u8] = iv.as_ref();
        let iv_data: js_sys::Uint8Array = iv_data.into();
        let mut data = data.to_vec();
        web_sys::window()
            .unwrap()
            .crypto()
            .unwrap()
            .subtle()
            .encrypt_with_object_and_u8_array(
                web_sys::AesGcmParams::new("AES-GCM", &iv_data)
                    .as_ref()
                    .dyn_ref::<js_sys::Object>()
                    .unwrap(),
                &self.key,
                &mut data,
            )
            .unwrap()
            .then(&Closure::once(Box::new(move |data: JsValue| {
                let buffer = js_sys::Uint8Array::new(&data);
                let mut data = vec![0u8; buffer.length() as usize];
                buffer.copy_to(&mut data);
                iv.after_encrypt(&mut data);
                executor::spawn(sender.send(data).then(|_| Ok(())));
            }) as Box<dyn FnOnce(_)>));
        Box::new(
            receiver
                .take(1)
                .into_future()
                .and_then(|data| Ok(data.0.unwrap()))
                .map_err(|_| failure::err_msg("temp err")),
        )
    }
    fn decrypt(&self, data: &'_ [u8]) -> Box<dyn Future<Item = Vec<u8>, Error = Error>>
    where
        Self: Sized,
    {
        let (sender, receiver) = channel(0);
        let mut data = data.to_owned();
        let iv = self.nonce_provider.lock().unwrap().next_decrypt(&mut data);
        let iv_data: &[u8] = iv.as_ref();
        let iv: js_sys::Uint8Array = iv_data.into();
        web_sys::window()
            .unwrap()
            .crypto()
            .unwrap()
            .subtle()
            .decrypt_with_object_and_u8_array(
                web_sys::AesGcmParams::new("AES-GCM", &iv)
                    .as_ref()
                    .dyn_ref::<js_sys::Object>()
                    .unwrap(),
                &self.key,
                &mut data,
            )
            .unwrap()
            .then(&Closure::once(Box::new(move |data: JsValue| {
                let buffer = js_sys::Uint8Array::new(&data);
                let mut data = vec![0u8; buffer.length() as usize];
                buffer.copy_to(&mut data);
                executor::spawn(sender.send(data).then(|_| Ok(())));
            }) as Box<dyn FnOnce(_)>));
        Box::new(
            receiver
                .take(1)
                .into_future()
                .and_then(|i| Ok(i.0.unwrap().into()))
                .map_err(|_| failure::err_msg("temp err")),
        )
    }
    fn as_bytes(&self) -> Box<dyn Future<Item = [u8; 16], Error = Error>> {
        let (sender, receiver) = channel(0);
        web_sys::window()
            .unwrap()
            .crypto()
            .unwrap()
            .subtle()
            .export_key("raw", &self.key)
            .unwrap()
            .then(&Closure::once(Box::new(move |data: JsValue| {
                let buffer = js_sys::Uint8Array::new(&data);
                let mut data: [u8; 16] = Default::default();
                buffer.copy_to(&mut data);
                executor::consecutive::spawn(sender.send(data).then(|_| Ok(())));
            }) as Box<dyn FnOnce(_)>));
        Box::new(
            receiver
                .take(1)
                .into_future()
                .and_then(|i| Ok(i.0.unwrap()))
                .map_err(|_| failure::err_msg("temp err")),
        )
    }
}

impl<T: NonceProvider + 'static> AESKey<T> {
    pub(crate) fn new() -> impl Future<Item = Box<dyn SymmetricKey<T> + 'static>, Error = Error> {
        let (sender, receiver) = channel(0);
        web_sys::window()
            .unwrap()
            .crypto()
            .unwrap()
            .subtle()
            .generate_key_with_object(
                web_sys::AesKeyGenParams::new("AES-GCM", 128)
                    .as_ref()
                    .dyn_ref::<js_sys::Object>()
                    .unwrap(),
                true,
                &vec![JsValue::from("encrypt"), JsValue::from("decrypt")]
                    .iter()
                    .collect::<js_sys::Array>(),
            )
            .unwrap()
            .then(&Closure::once(Box::new(move |key: JsValue| {
                let key = key.dyn_ref::<web_sys::CryptoKey>().unwrap().clone();
                executor::consecutive::spawn(sender.send(key).then(|_| Ok(())));
            }) as Box<dyn FnOnce(_)>));
        receiver
            .take(1)
            .into_future()
            .and_then(|item| {
                let key: Box<dyn SymmetricKey<T>> = Box::new(AESKey {
                    key: item.0.unwrap(),
                    nonce_provider: Mutex::new(T::new()),
                });
                Ok(key)
            })
            .map_err(|_| failure::err_msg("temp err"))
    }
    pub(crate) fn from_bytes(
        data: [u8; 16],
    ) -> impl Future<Item = Box<dyn SymmetricKey<T> + 'static>, Error = Error> {
        let (sender, receiver) = channel(0);
        web_sys::window()
            .unwrap()
            .crypto()
            .unwrap()
            .subtle()
            .import_key_with_str(
                "raw",
                js_sys::Uint8Array::from(data.as_ref())
                    .dyn_ref::<js_sys::Object>()
                    .unwrap(),
                "AES-GCM",
                true,
                &vec![JsValue::from("encrypt"), JsValue::from("decrypt")]
                    .iter()
                    .collect::<js_sys::Array>(),
            )
            .unwrap()
            .then(&Closure::once(Box::new(move |pair: JsValue| {
                let key = pair.dyn_ref::<web_sys::CryptoKey>().unwrap().clone();
                executor::consecutive::spawn(sender.send(key).then(|_| Ok(())));
            }) as Box<dyn FnOnce(_)>));
        receiver
            .take(1)
            .into_future()
            .and_then(|item| {
                let key: Box<dyn SymmetricKey<T>> = Box::new(AESKey {
                    key: item.0.unwrap(),
                    nonce_provider: Mutex::new(T::new()),
                });
                Ok(key)
            })
            .map_err(|_| failure::err_msg("temp err"))
    }
}

pub(crate) struct ECDSAKeyPair;

pub(crate) struct ECDSAPrivateKey {
    key: web_sys::CryptoKey,
}

impl VerifyingKey for ECDSAPublicKey {
    fn verify(
        &self,
        data: &'_ [u8],
        signature: &'_ [u8],
    ) -> Box<dyn Future<Item = bool, Error = Error>> {
        let (sender, receiver) = channel(0);
        let mut data = data.to_vec();
        let mut signature = signature.to_vec();
        web_sys::window()
            .unwrap()
            .crypto()
            .unwrap()
            .subtle()
            .verify_with_object_and_u8_array_and_u8_array(
                web_sys::EcdsaParams::new("ECDSA", &"SHA-256".into())
                    .as_ref()
                    .dyn_ref::<js_sys::Object>()
                    .unwrap(),
                &self.key,
                &mut signature,
                &mut data,
            )
            .unwrap()
            .then(&Closure::once(Box::new(move |result: JsValue| {
                let result = result.dyn_ref::<js_sys::Boolean>().unwrap().clone();
                executor::consecutive::spawn(sender.send(result.into()).then(|_| Ok(())));
            }) as Box<dyn FnOnce(_)>));
        Box::new(
            receiver
                .take(1)
                .into_future()
                .and_then(|i| Ok(i.0.unwrap()))
                .map_err(|_| failure::err_msg("temp err")),
        )
    }
    fn as_bytes(&self) -> Box<dyn Future<Item = Vec<u8>, Error = Error>> {
        let (sender, receiver) = channel(0);
        web_sys::window()
            .unwrap()
            .crypto()
            .unwrap()
            .subtle()
            .export_key("raw", &self.key)
            .unwrap()
            .then(&Closure::once(Box::new(move |data: JsValue| {
                let buffer = js_sys::Uint8Array::new(&data);
                let mut data = vec![0u8; buffer.length() as usize];
                buffer.copy_to(&mut data);
                executor::consecutive::spawn(sender.send(data).then(|_| Ok(())));
            }) as Box<dyn FnOnce(_)>));
        Box::new(
            receiver
                .take(1)
                .into_future()
                .and_then(|i| Ok(i.0.unwrap()))
                .map_err(|_| failure::err_msg("temp err")),
        )
    }
}

impl SigningKey for ECDSAPrivateKey {
    fn sign(&self, data: &'_ [u8]) -> Box<dyn Future<Item = Vec<u8>, Error = Error>> {
        let (sender, receiver) = channel(0);
        let mut data = data.to_vec();
        web_sys::window()
            .unwrap()
            .crypto()
            .unwrap()
            .subtle()
            .sign_with_object_and_u8_array(
                web_sys::EcdsaParams::new("ECDSA", &"SHA-256".into())
                    .as_ref()
                    .dyn_ref::<js_sys::Object>()
                    .unwrap(),
                &self.key,
                &mut data,
            )
            .unwrap()
            .then(&Closure::once(Box::new(move |result: JsValue| {
                let buffer = js_sys::Uint8Array::new(&result);
                let mut data = vec![0u8; buffer.length() as usize];
                buffer.copy_to(&mut data);
                executor::consecutive::spawn(sender.send(data).then(|_| Ok(())));
            }) as Box<dyn FnOnce(_)>));
        Box::new(
            receiver
                .take(1)
                .into_future()
                .and_then(|i| Ok(i.0.unwrap()))
                .map_err(|_| failure::err_msg("temp err")),
        )
    }
    fn as_bytes(&self) -> Box<dyn Future<Item = Vec<u8>, Error = Error>> {
        let (sender, receiver) = channel(0);
        web_sys::window()
            .unwrap()
            .crypto()
            .unwrap()
            .subtle()
            .export_key("pkcs8", &self.key)
            .unwrap()
            .then(&Closure::once(Box::new(move |data: JsValue| {
                let buffer = js_sys::Uint8Array::new(&data);
                let mut data = vec![0u8; buffer.length() as usize];
                buffer.copy_to(&mut data);
                executor::consecutive::spawn(sender.send(data).then(|_| Ok(())));
            }) as Box<dyn FnOnce(_)>));
        Box::new(
            receiver
                .take(1)
                .into_future()
                .and_then(|i| Ok(i.0.unwrap()))
                .map_err(|_| failure::err_msg("temp err")),
        )
    }
}

pub(crate) struct ECDSAPublicKey {
    key: web_sys::CryptoKey,
}

impl ECDSAPublicKey {
    pub(crate) fn from_bytes(
        data: &'_ [u8],
    ) -> impl Future<Item = Box<dyn VerifyingKey + 'static>, Error = Error> {
        let (sender, receiver) = channel(0);
        web_sys::window()
            .unwrap()
            .crypto()
            .unwrap()
            .subtle()
            .import_key_with_object(
                "raw",
                js_sys::Uint8Array::from(data)
                    .dyn_ref::<js_sys::Object>()
                    .unwrap(),
                web_sys::EcKeyImportParams::new("ECDSA")
                    .named_curve("P-256")
                    .as_ref()
                    .dyn_ref::<js_sys::Object>()
                    .unwrap(),
                true,
                &vec![JsValue::from("verify")]
                    .iter()
                    .collect::<js_sys::Array>(),
            )
            .unwrap()
            .then(&Closure::once(Box::new(move |pair: JsValue| {
                let key = pair.dyn_ref::<web_sys::CryptoKey>().unwrap().clone();
                executor::consecutive::spawn(sender.send(key).then(|_| Ok(())));
            }) as Box<dyn FnOnce(_)>));
        receiver
            .take(1)
            .into_future()
            .and_then(|item| {
                let key: Box<dyn VerifyingKey> = Box::new(ECDSAPublicKey {
                    key: item.0.unwrap(),
                });
                Ok(key)
            })
            .map_err(|_| failure::err_msg("temp err"))
    }
}

impl ECDSAPrivateKey {
    pub(crate) fn from_bytes(
        data: &'_ [u8],
    ) -> impl Future<Item = Box<dyn SigningKey + 'static>, Error = Error> {
        let (sender, receiver) = channel(0);
        web_sys::window()
            .unwrap()
            .crypto()
            .unwrap()
            .subtle()
            .import_key_with_object(
                "pkcs8",
                js_sys::Uint8Array::from(data)
                    .dyn_ref::<js_sys::Object>()
                    .unwrap(),
                web_sys::EcKeyImportParams::new("ECDSA")
                    .named_curve("P-256")
                    .as_ref()
                    .dyn_ref::<js_sys::Object>()
                    .unwrap(),
                true,
                &vec![JsValue::from("sign")]
                    .iter()
                    .collect::<js_sys::Array>(),
            )
            .unwrap()
            .then(&Closure::once(Box::new(move |pair: JsValue| {
                let key = pair.dyn_ref::<web_sys::CryptoKey>().unwrap().clone();
                executor::consecutive::spawn(sender.send(key).then(|_| Ok(())));
            }) as Box<dyn FnOnce(_)>));
        receiver
            .take(1)
            .into_future()
            .and_then(|item| {
                let key: Box<dyn SigningKey> = Box::new(ECDSAPrivateKey {
                    key: item.0.unwrap(),
                });
                Ok(key)
            })
            .map_err(|_| failure::err_msg("temp err"))
    }
}

impl ECDSAKeyPair {
    pub(crate) fn new() -> impl Future<
        Item = (
            Box<dyn SigningKey + 'static>,
            Box<dyn VerifyingKey + 'static>,
        ),
        Error = Error,
    > {
        lazy(|| {
            let (sender, receiver) = channel(0);
            web_sys::window()
                .unwrap()
                .crypto()
                .unwrap()
                .subtle()
                .generate_key_with_object(
                    web_sys::EcKeyGenParams::new("ECDSA", "P-256")
                        .as_ref()
                        .dyn_ref::<js_sys::Object>()
                        .unwrap(),
                    true,
                    &vec![JsValue::from("sign"), JsValue::from("verify")]
                        .iter()
                        .collect::<js_sys::Array>(),
                )
                .unwrap()
                .then(&Closure::once(Box::new(move |pair: JsValue| {
                    let private_key = js_sys::Reflect::get(&pair, &"privateKey".into())
                        .unwrap()
                        .dyn_ref::<web_sys::CryptoKey>()
                        .unwrap()
                        .clone();
                    let public_key = js_sys::Reflect::get(&pair, &"publicKey".into())
                        .unwrap()
                        .dyn_ref::<web_sys::CryptoKey>()
                        .unwrap()
                        .clone();
                    executor::consecutive::spawn(
                        sender.send((private_key, public_key)).then(|_| Ok(())),
                    );
                }) as Box<dyn FnOnce(_)>));
            receiver
                .take(1)
                .into_future()
                .and_then(|take| {
                    let (private_key, public_key) = take.0.unwrap();
                    let private_key: Box<dyn SigningKey> =
                        Box::new(ECDSAPrivateKey { key: private_key });
                    let public_key: Box<dyn VerifyingKey> =
                        Box::new(ECDSAPublicKey { key: public_key });
                    Ok((private_key, public_key))
                })
                .map_err(|_| failure::err_msg("temp err"))
        })
    }
}
