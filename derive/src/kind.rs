use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{parse2, parse_quote, parse_str, spanned::Spanned, Fields, Path, Type, WherePredicate};
use synstructure::{AddBounds, BindStyle, Structure};

pub fn derive(mut s: Structure) -> TokenStream {
    let kind_attr = parse_str::<Path>("kind").unwrap();
    let ast = s.ast();
    let ref ident = ast.ident;
    let hygiene = format_ident!("_IMPLEMENT_KIND_FOR_{}", ident);
    let mut using_kinds = ast.attrs.iter().filter(|attr| attr.path == kind_attr);
    let stream = if let Some(ty) = using_kinds.next() {
        if let Ok(ty) = parse2::<Type>(ty.tokens.clone()) {
            if let Some(ty) = using_kinds.next() {
                quote_spanned!(ty.span() => compile_error!("duplicate kind directive"))
            } else {
                s.add_bounds(AddBounds::None);
                for parameter in ast.generics.type_params() {
                    let ident = &parameter.ident;
                    s.add_where_predicate(parse_quote!(#ident: Send + Sync + Unpin + 'static));
                }
                s.gen_impl(quote!{
                    gen impl ::vessels::Kind for @Self where Self: ::vessels::kind::AsKind<#ty> {
                        type ConstructItem = ::vessels::channel::ForkHandle;
                        type ConstructError = ();
                        type ConstructFuture = ::futures::future::BoxFuture<'static, ::vessels::ConstructResult<Self>>;
                        type DeconstructItem = ();
                        type DeconstructError = ();
                        type DeconstructFuture = ::futures::future::BoxFuture<'static, ::vessels::DeconstructResult<Self>>;

                        fn deconstruct<C: ::vessels::channel::Channel<<Self as ::vessels::Kind>::DeconstructItem, <Self as ::vessels::Kind>::ConstructItem>>(
                            self,
                            mut channel: C,
                        ) -> <Self as ::vessels::Kind>::DeconstructFuture {
                            use ::futures::{SinkExt, TryFutureExt};
                            Box::pin(async move {
                                channel.send(channel.fork(<Self as ::vessels::kind::AsKind<#ty>>::into_kind(self)).await.unwrap()).unwrap_or_else(|_| panic!()).await;
                                Ok(())
                            })
                        }
                        fn construct<C: ::vessels::channel::Channel<<Self as ::vessels::Kind>::ConstructItem, <Self as ::vessels::Kind>::DeconstructItem>>(
                            mut channel: C,
                        ) -> <Self as ::vessels::Kind>::ConstructFuture {
                            use ::futures::StreamExt;
                            Box::pin(async move {
                                let handle = channel.next().await.unwrap();
                                Ok(<Self as ::vessels::kind::AsKind<#ty>>::from_kind(channel.get_fork::<<Self as ::vessels::kind::AsKind<#ty>>::Kind>(handle).await.unwrap()))
                            })
                        }
                    }
                })
            }
        } else {
            quote_spanned!(ty.span() => compile_error!("not a valid type"))
        }
    } else {
        let mut item_fields = TokenStream::new();
        let mut cons_arms = TokenStream::new();
        s.bind_with(|_| BindStyle::Move);
        s.add_bounds(AddBounds::Generics);
        let mut predicates: Vec<WherePredicate> = vec![];
        let arms = s.each_variant(|variant| {
            let ident = variant.ast().ident;
            use Fields::{Named, Unit, Unnamed};
            let mut bindings = TokenStream::new();
            let fields = match variant.ast().fields {
                Named(fields) => {
                    let mut items = TokenStream::new();
                    let mut cons_extension = TokenStream::new();
                    for (field, binding) in (&fields.named).into_iter().zip(variant.bindings()) {
                        let ident = &field.ident;
                        let mut using_kinds = binding.ast().attrs.iter().filter(|attr| attr.path == kind_attr);
                        let pat = binding.pat();
                        let stream = if let Some(ty) = using_kinds.next() {
                            if let Ok(ty) = parse2::<Type>(ty.tokens.clone()) {
                                if let Some(ty) = using_kinds.next() {
                                    quote_spanned!(ty.span() => compile_error!("duplicate kind directive"))
                                } else {
                                    let binding_ty = &binding.ast().ty;
                                    predicates.push(syn::parse_quote!(#binding_ty: ::vessels::kind::AsKind<#ty>));
                                    quote! {
                                        channel.fork(<#binding_ty as ::vessels::kind::AsKind<#ty>>::into_kind(#pat))
                                    }
                                }
                            } else {
                                quote_spanned!(ty.span() => compile_error!("not a valid type"))
                            }
                        } else {
                            quote!(channel.fork(#pat))
                        };
                        items.extend(quote!(#ident: ::vessels::channel::ForkHandle,));
                        cons_extension.extend(quote!(#ident,));
                        bindings.extend(quote!(#ident: #stream.await.unwrap(),));
                    }
                    item_fields.extend(quote!(#ident { #items },));
                    let construct = variant.construct(|field, _| {
                        let pat = field.ident.as_ref().unwrap();
                        let mut using_kinds = field.attrs.iter().filter(|attr| attr.path == kind_attr);
                        if let Some(ty) = using_kinds.next() {
                            if let Ok(ty) = parse2::<Type>(ty.tokens.clone()) {
                                if let Some(ty) = using_kinds.next() {
                                    quote_spanned!(ty.span() => compile_error!("duplicate kind directive"))
                                } else {
                                    let binding_ty = &field.ty;
                                    predicates.push(syn::parse_quote!(#binding_ty: ::vessels::kind::AsKind<#ty>));
                                    quote! {
                                        <#binding_ty as ::vessels::kind::AsKind<#ty>>::from_kind(channel.get_fork::<<#binding_ty as ::vessels::kind::AsKind<#ty>>::Kind>(#pat).await.unwrap())
                                    }
                                }
                            } else {
                                quote_spanned!(ty.span() => compile_error!("not a valid type"))
                            }
                        } else {
                            quote!(channel.get_fork(#pat).await.unwrap())
                        }
                    });
                    cons_arms
                        .extend(quote!(_DERIVE_Items::#ident{ #cons_extension } => #construct,));
                    return quote! {
                        channel.send({
                            _DERIVE_Items::#ident { #bindings }
                        }).unwrap_or_else(|_| panic!()).await
                    };
                }
                Unnamed(fields) => {
                    let mut items = TokenStream::new();
                    let mut cons_extension = TokenStream::new();
                    let mut cons_c_extension = TokenStream::new();
                    for (_, binding) in (&fields.unnamed).into_iter().zip(variant.bindings()) {
                        let mut using_kinds = binding.ast().attrs.iter().filter(|attr| attr.path == kind_attr);
                        let pat = binding.pat();
                        let stream = if let Some(ty) = using_kinds.next() {
                            if let Ok(ty) = parse2::<Type>(ty.tokens.clone()) {
                                if let Some(ty) = using_kinds.next() {
                                    quote_spanned!(ty.span() => compile_error!("duplicate kind directive"))
                                } else {
                                    let binding_ty = &binding.ast().ty;
                                    predicates.push(syn::parse_quote!(#binding_ty: ::vessels::kind::AsKind<#ty>));
                                    quote! {
                                        <#binding_ty as ::vessels::kind::AsKind<#ty>>::from_kind(channel.get_fork::<<#binding_ty as ::vessels::kind::AsKind<#ty>>::Kind>(#pat).await.unwrap())
                                    }
                                }
                            } else {
                                quote_spanned!(ty.span() => compile_error!("not a valid type"))
                            }
                        } else {
                            quote!(channel.get_fork(#pat).await.unwrap())
                        };
                        cons_extension.extend(quote!(#pat,));
                        cons_c_extension.extend(quote!(#stream,));
                        items.extend(quote!(::vessels::channel::ForkHandle,));
                    }
                    let id = &s.ast().ident;
                    cons_arms.extend(quote!(_DERIVE_Items::#ident(#cons_extension) => #id::#ident(#cons_c_extension),));
                    quote!((#items))
                }
                Unit => {
                    item_fields.extend(quote!(#ident,));
                    let id = &s.ast().ident;
                    cons_arms.extend(quote!(_DERIVE_Items::#ident => #id::#ident,));
                    return quote! {
                        channel.send({
                            _DERIVE_Items::#ident
                        }).unwrap_or_else(|_| panic!()).await
                    };
                }
            };
            item_fields.extend(quote!(#ident#fields,));
            for binding in variant.bindings() {
                let mut using_kinds = binding.ast().attrs.iter().filter(|attr| attr.path == kind_attr);
                let pat = binding.pat();
                let stream = if let Some(ty) = using_kinds.next() {
                    if let Ok(ty) = parse2::<Type>(ty.tokens.clone()) {
                        if let Some(ty) = using_kinds.next() {
                            quote_spanned!(ty.span() => compile_error!("duplicate kind directive"))
                        } else {
                            let binding_ty = &binding.ast().ty;
                            quote! {
                                <#binding_ty as ::vessels::kind::AsKind<#ty>>::into_kind(#pat)
                            }
                        }
                    } else {
                        quote_spanned!(ty.span() => compile_error!("not a valid type"))
                    }
                } else {
                    quote!(#pat)
                };
                bindings.extend(quote!(channel.fork(#stream).await.unwrap(),))
            }
            quote! {
                channel.send({
                    _DERIVE_Items::#ident(#bindings)
                }).unwrap_or_else(|_| panic!()).await
            }
        });
        let mut stream = quote! {
            #[derive(::vessels::serde::Serialize, ::vessels::serde::Deserialize)]
            pub enum _DERIVE_Items {
                #item_fields
            }
        };
        for predicate in predicates {
            s.add_where_predicate(predicate);
        }
        stream.extend(s.gen_impl(quote!{
            gen impl ::vessels::Kind for @Self {
                type ConstructItem = _DERIVE_Items;
                type ConstructError = ();
                type ConstructFuture = ::futures::future::BoxFuture<'static, ::vessels::ConstructResult<Self>>;
                type DeconstructItem = ();
                type DeconstructError = ();
                type DeconstructFuture = ::futures::future::BoxFuture<'static, ::vessels::DeconstructResult<Self>>;

                fn deconstruct<C: ::vessels::channel::Channel<<Self as ::vessels::Kind>::DeconstructItem, <Self as ::vessels::Kind>::ConstructItem>>(
                    self,
                    mut channel: C,
                ) -> <Self as ::vessels::Kind>::DeconstructFuture {
                    use ::futures::{SinkExt, TryFutureExt};
                    ::std::boxed::Box::pin(async move {
                        match self {
                            #arms
                        }
                        Ok(())
                    })
                }

                fn construct<C: ::vessels::channel::Channel<<Self as ::vessels::Kind>::ConstructItem, <Self as ::vessels::Kind>::DeconstructItem>>(
                    mut channel: C,
                ) -> <Self as ::vessels::Kind>::ConstructFuture {
                    use ::futures::StreamExt;
                    ::std::boxed::Box::pin(async move {
                        Ok(match channel.next().await.unwrap() {
                            #cons_arms
                        })
                    })
                }
            }
        }));
        stream
    };
    (quote! {
        #[allow(non_upper_case_globals)]
        const #hygiene: () = {
            #stream
        };
    })
    .into()
}
