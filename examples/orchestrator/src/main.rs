use vessels::{
    core::{
        data::Resource,
        hal::crypto::Hasher,
        orchestrator::{Module, Orchestrator},
        register, run, Core,
    },
    kind::Infallible,
    log,
};

use std::fs::read;

pub struct Tester;

impl test_vessel::Test for Tester {
    fn test(&self, message: String) -> Infallible<String> {
        Box::pin(async move { Ok(format!("passed through: {}", message)) })
    }
}

pub fn main() {
    let binary = read("../../target/wasm32-unknown-unknown/debug/test_vessel.wasm").unwrap();
    run(async move {
        let orchestrator = Orchestrator::new().unwrap();
        register(|| Hasher::new().unwrap());
        let mut core = Core::new();
        core.register(|| Box::new(Tester) as Box<dyn test_vessel::Test>);
        let data: String = orchestrator
            .instantiate(
                Resource::new(Module::new(binary)).await.unwrap(),
                core.into_handle(),
            )
            .await
            .unwrap();
        log!("{}", data);
    });
}
