use crate::{crypto::SymmetricKey, executor};
use stdweb::{unstable::TryInto, web::{TypedArray, ArrayBuffer}};
use failure::Error;
use futures::{lazy, Future, sync::mpsc::channel, Sink, Stream};

type CryptoKey = stdweb::Value;

pub(crate) struct AESKey {
    key: CryptoKey,
}

impl SymmetricKey for AESKey {
    fn encrypt(&self, data: &'_ [u8]) -> Box<dyn Future<Item = Vec<u8>, Error = Error> + Send>
    where
        Self: Sized,
    {
        let (sender, receiver) = channel(0);
        let data: TypedArray<u8> = data.into();
        js! {
            let iv = window.crypto.getRandomValues(new Uint8Array(12));
            window.crypto.subtle.encrypt({
                name: "AES-GCM",
                iv: iv,
            }, @{&self.key}, @{data}).then((data) => {
                @{move |data: ArrayBuffer, iv: ArrayBuffer| {
                    let s = sender.clone();
                    executor::spawn(sender.clone().send(iv).then(move |_| s.clone().send(data).then(|_| Ok(()))));
                }}(data, iv.buffer);
            });
        };
        Box::new(receiver.take(2).fold(vec![], |acc, i| {
            let mut acc = acc;
            acc.extend(Vec::from(i).iter());
            Ok(acc)
        }).map_err(|_| failure::err_msg("temp err")))
    }
    fn decrypt(&self, data: &'_ [u8]) -> Box<dyn Future<Item = Vec<u8>, Error = Error> + Send>
    where
        Self: Sized,
    {
        let (sender, receiver) = channel(0);
        let data: TypedArray<u8> = data.into();
        js! {
            let iv = @{&data}.slice(0, 12);
            let data = @{&data}.slice(12);
            window.crypto.subtle.encrypt({
                name: "AES-GCM",
                iv: iv,
            }, @{&self.key}, data).then((decrypted) => {
                @{move |data: ArrayBuffer| {
                    let s = sender.clone();
                    executor::spawn(sender.clone().send(data).then(|_| Ok(())));
                }}(decrypted);
            });
        };
        Box::new(receiver.take(1).into_future().and_then(|i| {
            Ok(i.0.unwrap().into())
        }).map_err(|_| failure::err_msg("temp err")))
    }
}

impl AESKey {
    pub(crate) fn new() -> impl Future<Item = Box<dyn SymmetricKey + 'static>, Error = Error> {
        let (sender, receiver) = channel(0);
        let promise = js! {
            window.crypto.subtle.generateKey({
                name: "AES-GCM",
                length: 128
            }, true, ["encrypt", "decrypt"]).then(@{move |key: CryptoKey| {
                executor::spawn(sender.clone().send(key).then(|_| Ok(())));
            }}).catch((err) => {
                console.log(err);
            });
        };
        receiver.take(1).into_future().and_then(|item| {
            let key: Box<dyn SymmetricKey> = Box::new(AESKey {
                key: item.0.unwrap()
            });
            Ok(key)
        }).map_err(|_| failure::err_msg("temp err"))
    }
}