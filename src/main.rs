use vessels::{kind::Future, object, reflection::Trait};

use futures::executor::block_on;

use std::any::Any;

#[object]
pub trait Supertrait {
    fn super_test(&self, hello: String) -> Future<u32>;
}

#[object]
pub trait Test: Supertrait {
    fn test(&self, hello: String) -> Future<u32>;
}

impl Supertrait for Shim {
    fn super_test(&self, hello: String) -> Future<u32> {
        Box::pin(async move {
            (hello.len() + 1) as u32
        })
    }
}

struct Shim;

impl Test for Shim {
    fn test(&self, hello: String) -> Future<u32> {
        Box::pin(async move {
            hello.len() as u32
        })
    }
}

fn main() {
    let trait_object = Box::new(Shim) as Box<dyn Test>;
    let method_index = trait_object.by_name("test").unwrap();
    println!("{:?}", trait_object.supertraits());
    println!("{}", block_on(*Box::<dyn Any + Send>::downcast::<Future<u32>>(trait_object.call(method_index, vec![Box::new("four".to_owned())]).unwrap()).unwrap()));
}