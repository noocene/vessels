use super::Rng as IRng;

use crate::kind::Future;

pub struct Rng;

impl IRng for Rng {
    fn bytes(&mut self, len: usize) -> Future<Vec<u8>> {
        Box::pin(async move {
            let mut data = vec![0u8; len];
            web_sys::window()
                .unwrap()
                .crypto()
                .unwrap()
                .get_random_values_with_u8_array(&mut data)
                .unwrap();
            data
        })
    }
}

impl Rng {
    pub fn new() -> Box<dyn IRng> {
        Box::new(Rng)
    }
}
