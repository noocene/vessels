use core::convert::{Infallible, TryFrom};
use futures::executor::block_on;
use std::string::FromUtf8Error;
use vessels::{
    acquire, register,
    resource::{ErasedResourceManager, ResourceManagerExt},
    with_core, Convert, Core, MemoryStore, Ring, Sha256, SimpleResourceManager,
};

#[derive(Debug, Clone)]
pub struct Tester(String);

impl TryFrom<Vec<u8>> for Tester {
    type Error = FromUtf8Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        String::from_utf8(value).map(Tester)
    }
}

impl From<Tester> for Vec<u8> {
    fn from(tester: Tester) -> Vec<u8> {
        tester.0.as_bytes().into()
    }
}

fn main() {
    let core = Core::new();

    with_core! { &core => { block_on(entry()) }};
}

async fn entry() {
    let mut manager = SimpleResourceManager::new();

    let mut store = MemoryStore::<Sha256>::new();

    manager.add_provider(store.clone()).await;

    register(move || {
        let manager = manager.clone();

        Box::pin(async move { Ok::<_, Infallible>(manager.erase_resource_manager()) })
    })
    .await
    .unwrap();

    let data = Tester("hello there".into());

    let resource = store
        .intern::<Ring, _, Convert>(data.clone())
        .await
        .unwrap();

    let manager = acquire::<ErasedResourceManager>().await.unwrap().unwrap();

    let item = manager.fetch(resource).await.unwrap().unwrap();

    println!("{:?}", item);
}
