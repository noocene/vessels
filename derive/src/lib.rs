mod kind;
mod object;

extern crate proc_macro;

use proc_macro::TokenStream;
use synstructure::decl_derive;

decl_derive!([Kind, attributes(kind)] => kind::derive);

#[proc_macro_attribute]
pub fn object(attribute: TokenStream, mut item: TokenStream) -> TokenStream {
    item.extend(TokenStream::from(object::build(
        attribute.into(),
        item.clone().into(),
    )));
    item
}
