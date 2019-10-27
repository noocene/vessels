use proc_macro2::{Span, TokenStream};
use proc_macro_error::*;
use quote::{format_ident, quote};
use syn::{parse2, Item, Type};

pub(crate) fn kind(attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let mut items = proc_macro2::TokenStream::new();
    let ident = match parse2::<Item>(item.clone()).unwrap_or_else(|_| {
        abort!(
            Span::call_site(),
            "kind must be generated from a struct or enu,"
        )
    }) {
        Item::Struct(item) => item.ident,
        Item::Enum(item) => item.ident,
        _ => abort!(
            Span::call_site(),
            "kind must be generated from a struct or enu,"
        ),
    };
    items.extend(
        parse2::<Type>(attr)
            .map(|ty| -> TokenStream {
                (quote! {
                    impl Kind for #ident {
                        type ConstructItem = <<#ident as AsKind<#ty>>::Kind as Kind>::ConstructItem;
                        type ConstructFuture = <#ident as AsKind<#ty>>::ConstructFuture;
                        type DeconstructItem = <<#ident as AsKind<#ty>>::Kind as Kind>::DeconstructItem;
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
                })
                .into()
            })
            .unwrap_or_else(|_| abort!(Span::call_site(), "the target Kind must be specified")),
    );
    let ident = format_ident!("_IMPLEMENT_KIND_FOR_{}", ident);
    item.extend(quote! {
        #[allow(non_upper_case_globals)]
        const #ident: () = {
            #items
        };
    });
    item
}
