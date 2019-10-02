use failure::Error;
use futures::{lazy, Future, IntoFuture};
use stdweb::{unstable::TryInto, web::TypedArray};

pub(crate) mod primitives;

pub(crate) fn random(bytes: u32) -> impl Future<Item = Vec<u8>, Error = Error> {
    lazy(move || {
        let data = vec![0u8; bytes as usize];
        web_sys::window()
            .unwrap()
            .crypto()
            .unwrap()
            .get_random_values_with_u8_array(&mut data)
            .unwrap();
        Ok(data.into()).into_future()
    })
}
