extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse2, parse_str, Path, Type};
use synstructure::{decl_derive, Structure};

decl_derive!([Kind, attributes(kind)] => kind_derive);

fn kind_derive(s: Structure) -> TokenStream {
    let kind_attr = parse_str::<Path>("kind").unwrap();
    let ref ast = s.ast();
    let ty = ast
        .attrs
        .iter()
        .filter(move |attr| attr.path == kind_attr)
        .flat_map(|attr| parse2::<Type>(attr.tokens.clone()))
        .next()
        .unwrap();
    let ref ident = ast.ident;
    let hygiene = format_ident!("_IMPLEMENT_KIND_FOR_{}", ident);
    (quote! {
        #[allow(non_upper_case_globals)]
        const #hygiene: () = {
            impl Kind for #ident {
                type ConstructItem = <<#ident as AsKind<#ty>>::Kind as Kind>::ConstructItem;
                type ConstructFuture = <#ident as AsKind<#ty>>::ConstructFuture;
                type ConstructError = <<#ident as AsKind<#ty>>::Kind as Kind>::ConstructError;
                type DeconstructItem = <<#ident as AsKind<#ty>>::Kind as Kind>::DeconstructItem;
                type DeconstructError = <<#ident as AsKind<#ty>>::Kind as Kind>::DeconstructError;
                type DeconstructFuture = <<#ident as AsKind<#ty>>::Kind as Kind>::DeconstructFuture;

                fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                    self,
                    channel: C,
                ) -> Self::DeconstructFuture {
                    self.into_kind().deconstruct(channel)
                }
                fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                    channel: C,
                ) -> Self::ConstructFuture {
                    <#ident as AsKind<#ty>>::from_kind(<<#ident as AsKind<#ty>>::Kind as Kind>::construct(channel))
                }
            }
        };
    }).into()
}
