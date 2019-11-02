mod kind;
mod object;

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse, ItemTrait};
use synstructure::decl_derive;

decl_derive!([Kind, attributes(kind)] => kind::derive);

#[proc_macro_attribute]
pub fn object(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse::<ItemTrait>(item.clone()).unwrap_or_else(|_| panic!("expected trait"));
    let extension = object::build(attribute.into(), &mut item);
    let mut item = item.into_token_stream();
    item.extend(extension);
    item.into()
}
