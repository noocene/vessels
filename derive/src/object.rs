use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse2, punctuated::Punctuated, FnArg, ItemTrait, Token, TraitItem};

pub fn build(_: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse2::<ItemTrait>(item.clone()).unwrap_or_else(|_| panic!("expected trait"));
    let ident = item.ident;
    let hygiene = format_ident!("_IMPLEMENT_PROTOCOL_FOR_{}", ident);
    let mut fields = TokenStream::new();
    let mut from_fields = TokenStream::new();
    let mut shim_items = TokenStream::new();
    for item in item.items {
        use TraitItem::Method;
        if let Method(method) = item {
            let sig = method.sig.clone();
            let ident = method.sig.ident;
            let mut args = TokenStream::new();
            let inputs = method.sig.inputs;
            for input in &inputs {
                use FnArg::Typed;
                if let Typed(ty) = input {
                    let ty = &ty.ty;
                    args.extend(quote!(#ty,))
                }
            }
            let output = method.sig.output;
            fields.extend(quote! {
                #ident: Box<dyn Fn(#args) #output + Send>,
            });
            let inputs: Punctuated<_, Token![,]> = inputs
                .iter()
                .filter_map(|arg| {
                    use FnArg::Typed;
                    if let Typed(ty) = arg {
                        Some(ty.pat.clone())
                    } else {
                        None
                    }
                })
                .collect();
            from_fields.extend(quote! {
                #ident: { let object = object.clone(); ::std::boxed::Box::new(move |#inputs| object.lock().unwrap().#ident(#inputs)) },
            });
            shim_items.extend(quote! {
                #sig {
                    self.#ident(#inputs)
                }
            });
        }
    }
    quote! {
        #[allow(non_upper_case_globals)]
        const #hygiene: () = {
            pub struct _DERIVED_Shim {
                #fields
            }
            impl _DERIVED_Shim {
                fn from_object(object: ::std::boxed::Box<dyn #ident>) -> Self {
                    let object = ::std::sync::Arc::new(::std::sync::Mutex::new(object));
                    _DERIVED_Shim {
                       #from_fields
                    }
                }
            }
            impl #ident for _DERIVED_Shim {
                #shim_items
            }
            impl ::vessels::object::Protocol for dyn #ident {
                type Shim = _DERIVED_Shim;
            }
        };
    }
}
