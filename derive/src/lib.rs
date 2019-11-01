extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{parse2, parse_str, spanned::Spanned, Path, Type};
use synstructure::{decl_derive, Structure};

decl_derive!([Kind, attributes(kind)] => kind_derive);

fn kind_derive(s: Structure) -> TokenStream {
    let kind_attr = parse_str::<Path>("kind").unwrap();
    let ref ast = s.ast();
    let ref ident = ast.ident;
    let hygiene = format_ident!("_IMPLEMENT_KIND_FOR_{}", ident);
    let mut using_kinds = ast.attrs.iter().filter(move |attr| attr.path == kind_attr);
    let stream = if let Some(ty) = using_kinds.next() {
        if let Ok(ty) = parse2::<Type>(ty.tokens.clone()) {
            if let Some(ty) = using_kinds.next() {
                quote_spanned!(ty.span() => compile_error!("duplicative kind directive"))
            } else {
                quote! {
                    impl ::vessels::Kind for #ident {
                        type ConstructItem = <<#ident as ::vessels::kind::AsKind<#ty>>::Kind as ::vessels::Kind>::ConstructItem;
                        type ConstructFuture = <#ident as ::vessels::kind::AsKind<#ty>>::ConstructFuture;
                        type ConstructError = <<#ident as ::vessels::kind::AsKind<#ty>>::Kind as ::vessels::Kind>::ConstructError;
                        type DeconstructItem = <<#ident as ::vessels::kind::AsKind<#ty>>::Kind as ::vessels::Kind>::DeconstructItem;
                        type DeconstructError = <<#ident as ::vessels::kind::AsKind<#ty>>::Kind as ::vessels::Kind>::DeconstructError;
                        type DeconstructFuture = <<#ident as ::vessels::kind::AsKind<#ty>>::Kind as ::vessels::Kind>::DeconstructFuture;

                        fn deconstruct<C: ::vessels::channel::Channel<<Self as ::vessels::Kind>::DeconstructItem, <Self as ::vessels::Kind>::ConstructItem>>(
                            self,
                            channel: C,
                        ) -> <Self as ::vessels::Kind>::DeconstructFuture {
                            ::vessels::Kind::deconstruct(<#ident as ::vessels::kind::AsKind<#ty>>::into_kind(self), channel)
                        }
                        fn construct<C: ::vessels::channel::Channel<<Self as ::vessels::Kind>::ConstructItem, <Self as ::vessels::Kind>::DeconstructItem>>(
                            channel: C,
                        ) -> <Self as ::vessels::Kind>::ConstructFuture {
                            <#ident as ::vessels::kind::AsKind<#ty>>::from_kind(<<#ident as ::vessels::kind::AsKind<#ty>>::Kind as ::vessels::Kind>::construct(channel))
                        }
                    }
                }
            }
        } else {
            quote_spanned!(ty.span() => compile_error!("not a valid type"))
        }
    } else {
        proc_macro2::TokenStream::new()
    };
    (quote! {
        #[allow(non_upper_case_globals)]
        const #hygiene: () = {
            #stream
        };
    })
    .into()
}
