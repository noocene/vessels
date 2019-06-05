extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::{quote, quote_spanned};

use syn::spanned::Spanned;
use syn::{parse_macro_input, FnArg, ItemTrait, ReturnType, TraitItem};
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
pub fn protocol(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = item;
    if !attr.is_empty() {
        return r#"compile_error!("unexpected arguments passed to `protocol`");"#
            .parse()
            .unwrap();
    }
    let input = {
        let item = item.clone();
        parse_macro_input!(item as ItemTrait)
    };
    if !input.generics.params.is_empty() {
        return TokenStream::from(quote_spanned! {
            input.generics.params.first().unwrap().span() =>
            compile_error!("generic parameters not allowed in `protocol` trait");
        });
    }
    if !input.supertraits.is_empty() {
        return TokenStream::from(quote_spanned! {
            input.supertraits.first().unwrap().span() =>
            compile_error!("supertraits not allowed on `protocol` trait");
        });
    }
    let mut assert_stream = TokenStream::new();
    for item in input.items {
        if let TraitItem::Method(method) = item {
            if let Some(default) = method.default {
                return TokenStream::from(quote_spanned! {
                    default.span() =>
                    compile_error!("default implementations not allowed in `protocol` methods");
                });
            }
            if !method.sig.decl.generics.params.is_empty() {
                return TokenStream::from(quote_spanned! {
                    method.sig.decl.generics.params.first().unwrap().span() =>
                    compile_error!("generic parameters not allowed on `protocol` method");
                });
            }
            if let ReturnType::Type(_, ty) = &method.sig.decl.output {
                assert_stream.extend(TokenStream::from(quote_spanned! {
                    ty.span() =>
                    struct _AssertSerializeDeserialize where #ty: serde::Serialize + serde::de::DeserializeOwned;
                }));
            }
            let mut has_receiver = false;
            for argument in &method.sig.decl.inputs {
                match argument {
                    FnArg::SelfValue(_) => {
                        return TokenStream::from(quote_spanned! {
                            argument.span() =>
                            compile_error!("cannot consume self in `protocol` method");
                        });
                    }
                    FnArg::SelfRef(_) => {
                        has_receiver = true;
                    }
                    FnArg::Captured(argument) => {
                        let ty = &argument.ty;
                        assert_stream.extend(TokenStream::from(quote_spanned! {
                            ty.span() =>
                            struct _AssertSerializeDeserialize where #ty: serde::Serialize + serde::de::DeserializeOwned;
                        }));
                    }
                    _ => {
                        return TokenStream::from(quote_spanned! {
                            argument.span() =>
                            compile_error!("inferred or ignored argument not allowed in `protocol` method");
                        });
                    }
                };
            }
            if !has_receiver {
                return TokenStream::from(quote_spanned! {
                    method.sig.ident.span() =>
                    compile_error!("method in `protocol` has no receiver");
                });
            }
        } else {
            return TokenStream::from(quote_spanned! {
                item.span() =>
                compile_error!("`protocol` expected method");
            });
        }
    }
    item.extend(assert_stream);
    item
}