use vessels::{
    kind::Future,
    object,
    reflection::{Trait, Upcasted},
    Kind
};

use futures::executor::block_on;

use std::any::Any;

#[object]
pub trait Supertrait {
    fn super_test(&self, hello: String) -> Future<u32>;
}

#[object]
pub trait Test<T: Kind> {
    fn test(&self, hello: String, t: T) -> Future<u32>;
}

impl Supertrait for Shim {
    fn super_test(&self, hello: String) -> Future<u32> {
        Box::pin(async move { (hello.len() + 1) as u32 })
    }
}

struct Shim;

impl Test<u32> for Shim {
    fn test(&self, hello: String, _: u32) -> Future<u32> {
        Box::pin(async move { hello.len() as u32 })
    }
}

fn main() {
    let trait_object = Box::new(Shim) as Box<dyn Test<u32>>;
    let method_index = trait_object.by_name("test").unwrap();
    let supertraits = trait_object.supertraits();
    println!("{:?}", supertraits);
    //let upcast_object: Upcasted<dyn Supertrait> =
    //    *Box::<dyn Any + Send>::downcast(trait_object.upcast(supertraits[0]).unwrap()).unwrap();
    println!(
        "{}",
        block_on(
            *Box::<dyn Any + Send>::downcast::<Future<u32>>(
                trait_object
                    .call(method_index, vec![Box::new("four".to_owned()), Box::new(0u32)])
                    .unwrap()
            )
            .unwrap()
        )
    );
}
