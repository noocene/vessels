use vessels::{core::acquire, export};

export! {
    acquire::<Box<dyn test_vessel::Test>>().await.unwrap().test("hello".to_owned()).await
}
