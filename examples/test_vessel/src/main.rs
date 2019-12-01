use vessels::{core::acquire, export};

export! {
    acquire::<Box<dyn test_vessel::Test>>().await.unwrap().test("hello there".to_owned()).await;
    "test".to_string()
}
