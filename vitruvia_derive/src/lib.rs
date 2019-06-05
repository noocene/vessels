extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

fn impl_protocol(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ::vitruvia::protocol::Protocol for #name {
            fn hello() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn protocol(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse(item).unwrap();
    impl_protocol(&ast)
}