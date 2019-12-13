use vessels::{
    core::Core,
    object,
    replicate::{Share, Shared},
};

#[object]
trait Object {}

struct Tester;

impl Object for Tester {}

#[derive(Share)]
struct Test {
    shared_object: Shared<dyn Object>,
    clonable: String,
    share: Core,
}

fn main() {
    (Test {
        shared_object: Shared::new(Box::new(Tester)),
        clonable: "test".to_owned(),
        share: Core::new(),
    })
    .share();
}
