use crate::reflect::{
    CallError, CastError, Erased, MethodIndex, MethodTypes, NameError, OutOfRangeError, Reflected,
    Trait,
};

use std::{
    any::{Any, TypeId},
    sync::{Arc, Mutex},
};

pub mod collections;

pub trait Share {
    fn share(&self) -> Self;
}

impl<T: Clone> Share for T {
    fn share(&self) -> Self {
        self.clone()
    }
}

pub struct Shared<T: Trait<T> + Reflected + ?Sized>(Arc<Mutex<Box<T>>>);

impl<T: Trait<T> + Reflected + ?Sized> Shared<T> {
    pub fn new(item: Box<T>) -> Self {
        Shared(Arc::new(Mutex::new(item)))
    }
}

impl<T: Trait<T> + Reflected + ?Sized> Share for Shared<T> {
    fn share(&self) -> Self {
        Shared(self.0.share())
    }
}

impl<T: Trait<T> + Reflected + ?Sized> Trait<T> for Shared<T> {
    fn call(
        &self,
        index: MethodIndex,
        args: Vec<Box<dyn Any + Send + Sync>>,
    ) -> Result<Box<dyn Any + Send + Sync>, CallError> {
        self.0.lock().unwrap().call(index, args)
    }
    fn call_mut(
        &mut self,
        index: MethodIndex,
        args: Vec<Box<dyn Any + Send + Sync>>,
    ) -> Result<Box<dyn Any + Send + Sync>, CallError> {
        self.0.lock().unwrap().call_mut(index, args)
    }
    fn call_move(
        self: Box<Self>,
        index: MethodIndex,
        args: Vec<Box<dyn Any + Send + Sync>>,
    ) -> Result<Box<dyn Any + Send + Sync>, CallError> {
        Arc::try_unwrap(self.0)
            .unwrap_or_else(|_| panic!())
            .into_inner()
            .unwrap()
            .call_move(index, args)
    }
    fn by_name(&self, name: &'_ str) -> Result<MethodIndex, NameError> {
        self.0.lock().unwrap().by_name(name)
    }
    fn count(&self) -> MethodIndex {
        self.0.lock().unwrap().count()
    }
    fn name_of(&self, index: MethodIndex) -> Result<String, OutOfRangeError> {
        self.0.lock().unwrap().name_of(index)
    }
    fn this(&self) -> TypeId {
        self.0.lock().unwrap().this()
    }
    fn name(&self) -> String {
        self.0.lock().unwrap().name()
    }
    fn types(&self, index: MethodIndex) -> Result<MethodTypes, OutOfRangeError> {
        self.0.lock().unwrap().types(index)
    }
    fn supertraits(&self) -> Vec<TypeId> {
        self.0.lock().unwrap().supertraits()
    }
    fn upcast_erased(self: Box<Self>, ty: TypeId) -> Result<Box<dyn Erased>, CastError> {
        Arc::try_unwrap(self.0)
            .unwrap_or_else(|_| panic!())
            .into_inner()
            .unwrap()
            .upcast_erased(ty)
    }
    fn erase(self: Box<Self>) -> Box<dyn Erased> {
        Arc::try_unwrap(self.0)
            .unwrap_or_else(|_| panic!())
            .into_inner()
            .unwrap()
            .erase()
    }
}
