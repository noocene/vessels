use vessels::{
    kind::Future,
    object,
    reflection::{Trait, Erased},
    Kind,
};

use futures::executor::block_on;

use std::any::Any;

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
    let method_index = upcast_object.by_name("super_test").unwrap();
    println!(
        "{}",
        block_on(
            *Box::<dyn Any + Send>::downcast::<Future<u32>>(
                upcast_object.call(method_index, vec![Box::new("test".to_owned())]).unwrap()
            )
            .unwrap()
        )
    );
}
