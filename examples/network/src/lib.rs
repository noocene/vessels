use vessels::{
    core,
    core::{executor::Spawn, hal::network::Client, Executor},
    log,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    core::<dyn Executor>().unwrap().run(async move {
        let mut network = Client::<String>::new().unwrap();
        let data = network
            .connect("ws://127.0.0.1:61200".parse().unwrap())
            .await
            .unwrap();
        log!("{}", data);
    });
}
