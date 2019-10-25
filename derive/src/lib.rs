extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use syn::{parse_macro_input, ImplItem, ItemImpl};

#[proc_macro_attribute]
pub fn value(
    _attr: proc_macro::TokenStream,
    i: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item = parse_macro_input!(i as ItemImpl);
    let mut stream = TokenStream::new();
    if item
        .items
        .iter()
        .find(|item| {
            if let ImplItem::Const(item) = item {
                if item.ident.to_string() == "DO_NOT_IMPLEMENT_THIS_TRAIT_MANUALLY" {
                    stream.extend(quote_spanned! {
                        item.ident.span() =>
                        compile_error!("No constant with this name can be present");
                    });
                    return true;
                }
            }
            false
        })
        .is_none()
    {
        let stream = quote! {
            const DO_NOT_IMPLEMENT_THIS_TRAIT_MANUALLY: () = ();
        }
        .into();
        item.items.push(parse_macro_input!(stream as ImplItem));
    };
    item.items.iter().for_each(|i| {
        if let ImplItem::Type(ty) = i {
            let ty_name = ty.ident.to_string();
            let is_ci = ty_name == "ConstructItem";
            if is_ci || ty_name == "DeconstructItem" {
                let mut hasher = DefaultHasher::new();
                (ty.into_token_stream().to_string(), format!("{:?}", ty.ident.span()), is_ci, item.clone().into_token_stream().to_string()).hash(&mut hasher);
                let unique = format_ident!("_{}", hasher.finish());
                let ty = &ty.ty;
                stream.extend(quote! {
                    inventory::submit! {
                        let #unique: () = ();
                        ErasedDeserialize::new({
                            ::std::any::TypeId::of::<#ty>()
                        }, |de| <#ty as ::serde::Deserialize>::deserialize(de).map(|v| Box::new(v) as Box<dyn SerdeAny>))
                    }
                });
            }
        }
    });
    item.to_tokens(&mut stream);
    stream.into()
}
