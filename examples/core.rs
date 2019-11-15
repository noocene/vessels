use vessels::{core::{hal::crypto::Rng, Executor, executor::Spawn}, core, log};

fn main() {
    core::<dyn Executor>().unwrap().run(async move {
        let mut rng = Rng::new().unwrap();
        log!("{:?}", rng.bytes(16).await);
    });
}
