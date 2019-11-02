use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{
    parse_quote, punctuated::Punctuated, spanned::Spanned, FnArg, GenericParam, ItemTrait, Token,
    TraitItem,
};

pub fn build(_: TokenStream, item: &mut ItemTrait) -> TokenStream {
    item.supertraits.push(parse_quote!(::std::marker::Send));
    let mut params = TokenStream::new();
    let mut kind_bounded_params = item.generics.params.clone();
    for parameter in &mut kind_bounded_params {
        use GenericParam::{Lifetime, Type};
        if let Lifetime(_) = parameter {
            return quote_spanned!(parameter.span() => compile_error!("lifetime parameters are not supported"));
        }
        if let Type(parameter) = parameter {
            let ident = &parameter.ident;
            parameter.bounds.push(parse_quote!('static));
            parameter.bounds.push(parse_quote!(Send));
            params.extend(quote!(#ident,));
        }
    }
    let ident = &item.ident;
    let hygiene = format_ident!("_IMPLEMENT_PROTOCOL_FOR_{}", ident);
    let mut fields = TokenStream::new();
    let mut from_fields = TokenStream::new();
    let mut shim_items = TokenStream::new();
    for item in &item.items {
        use TraitItem::Method;
        if let Method(method) = item {
            let sig = method.sig.clone();
            let ident = &method.sig.ident;
            let mut args = TokenStream::new();
            let inputs = &method.sig.inputs;
            for input in inputs {
                use FnArg::Typed;
                if let Typed(ty) = input {
                    let ty = &ty.ty;
                    args.extend(quote!(#ty,));
                }
            }
            let output = &method.sig.output;
            fields.extend(quote! {
                #ident: Box<dyn Fn(#args) #output + Send + Sync>,
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
                    (self.#ident)(#inputs)
                }
            });
        }
    }
    quote! {
        #[allow(non_upper_case_globals)]
        const #hygiene: () = {
            #[derive(::vessels::Kind)]
            pub struct _DERIVED_Shim<#kind_bounded_params> {
                #fields
                _marker: ::std::marker::PhantomData<(#params)>
            }
            impl<#kind_bounded_params> _DERIVED_Shim<#params> {
                fn from_object(object: ::std::boxed::Box<dyn #ident<#params>>) -> Self {
                    let object = ::std::sync::Arc::new(::std::sync::Mutex::new(object));
                    _DERIVED_Shim {
                       #from_fields
                       _marker: ::std::marker::PhantomData
                    }
                }
            }
            impl<#kind_bounded_params> #ident<#params> for _DERIVED_Shim<#params> {
                #shim_items
            }
            impl<#kind_bounded_params> ::vessels::Kind for ::std::boxed::Box<dyn #ident<#params>> {
                type ConstructItem = ::vessels::channel::ForkHandle;
                type ConstructError = ::vessels::void::Void;
                type ConstructFuture = ::vessels::futures::future::BoxFuture<'static, ::vessels::ConstructResult<Self>>;
                type DeconstructItem = ();
                type DeconstructError = ::vessels::void::Void;
                type DeconstructFuture = ::vessels::futures::future::BoxFuture<'static, ::vessels::DeconstructResult<Self>>;

                fn deconstruct<C: ::vessels::channel::Channel<<Self as ::vessels::Kind>::DeconstructItem, <Self as ::vessels::Kind>::ConstructItem>>(
                    self,
                    mut channel: C,
                ) -> <Self as ::vessels::Kind>::DeconstructFuture {
                    use ::vessels::futures::{SinkExt, TryFutureExt};
                    ::std::boxed::Box::pin(async move {
                        channel.send(channel.fork::<_DERIVED_Shim<#params>>(_DERIVED_Shim::from_object(self)).await.unwrap()).unwrap_or_else(|_| panic!()).await;
                        Ok(())
                    })
                }

                fn construct<C: ::vessels::channel::Channel<<Self as ::vessels::Kind>::ConstructItem, <Self as ::vessels::Kind>::DeconstructItem>>(
                    mut channel: C,
                ) -> <Self as ::vessels::Kind>::ConstructFuture {
                    use ::vessels::futures::StreamExt;
                    ::std::boxed::Box::pin(async move {
                        let handle = channel.next().await.unwrap();
                        Ok(Box::new(channel.get_fork::<_DERIVED_Shim<#params>>(handle).await.unwrap()) as Box<dyn #ident<#params>>)
                    })
                }
            }
        };
    }
}
