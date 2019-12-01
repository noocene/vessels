use vessels::{core::acquire, export};

export! {
    acquire::<String>().await.unwrap()
}
