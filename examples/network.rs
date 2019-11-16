use vessels::{
    core,
    core::{executor::Spawn, hal::{network::{Network, StaticCandidate}, crypto::Rng}, Executor},
    log,
};

fn main() {
    core::<dyn Executor>().unwrap().run(async move {
        let mut network = Network::new().unwrap();
        let mut rng = Rng::new().unwrap();
        let mut ufrag = [0u8; 3];
        ufrag.copy_from_slice(rng.bytes(3).await.as_slice());
        let mut pwd = [0u8; 16];
        pwd.copy_from_slice(rng.bytes(16).await.as_slice());
        let mut fingerprint = [0u8; 32];
        fingerprint.copy_from_slice(rng.bytes(16).await.as_slice());
        let connection = network.connect(StaticCandidate {
            addr: "127.0.0.1:8080".parse().unwrap(),
            pwd,
            fingerprint,
            ufrag
        }).await.unwrap();
    });
}
