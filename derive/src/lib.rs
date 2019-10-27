extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro_error::*;
use syn::{Item, Meta, Type};
mod kind;
use synstructure::{decl_derive, Structure};

decl_derive!([Kind, attributes(kind)] => kind_derive);

fn kind_derive(_: Structure) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn kind(attr: TokenStream, item: TokenStream) -> proc_macro::TokenStream {
    kind::kind(attr.into(), item.into()).into()
}
