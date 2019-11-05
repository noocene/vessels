use vessels::{
    kind::Future,
    object,
    reflection::{Erased, Trait, Downcast},
    Kind,
};

use futures::executor::block_on;

#[object]
pub trait Supertrait {
    fn super_test(&self, hello: String) -> Future<u32>;
    fn move_out(self: Box<Self>) -> Future<String>;
}

#[object]
pub trait Test<T: Kind>: Supertrait {
    fn test(&self) -> Future<u32>;
}

impl Supertrait for Shim {
    fn super_test(&self, _: String) -> Future<u32> {
        Box::pin(async move { 2u32 })
    }
    fn move_out(self: Box<Self>) -> Future<String> {
        Box::pin(async move { "test".to_owned() })
    }
}

struct Shim;

impl Test<u32> for Shim {
    fn test(&self) -> Future<u32> {
        Box::pin(async move { 3u32 })
    }
}

fn main() {
    let trait_object = Box::new(Shim) as Box<dyn Test<u32>>;
    let supertraits = trait_object.supertraits();
    println!("{:?}", supertraits);
    let upcast_object: Box<dyn Erased> = trait_object.upcast(supertraits[0]).unwrap();
    let concrete_object: Box<dyn Supertrait> = upcast_object.downcast().unwrap();
    println!(
        "{}",
        block_on(concrete_object.super_test("hello".to_owned()))
    );
}
