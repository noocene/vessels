use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{
    parse_quote, punctuated::Punctuated, spanned::Spanned, FnArg, GenericParam, ItemTrait,
    ReturnType, Token, TraitItem, TypeParamBound,
};

type MethodIndex = u8;

pub fn build(_: TokenStream, item: &mut ItemTrait) -> TokenStream {
    let mut params = TokenStream::new();
    let ident = &item.ident;
    let hygiene = format_ident!("_IMPLEMENT_PROTOCOL_FOR_{}", ident);
    let mut kind_bounded_params = item.generics.params.clone();
    for parameter in &mut kind_bounded_params {
        use GenericParam::{Lifetime, Type};
        if let Lifetime(_) = parameter {
            return quote_spanned!(parameter.span() => const #hygiene: () = { compile_error!("lifetime parameters are not supported") };);
        }
        if let Type(parameter) = parameter {
            let ident = &parameter.ident;
            parameter.bounds.push(parse_quote!('static));
            parameter.bounds.push(parse_quote!(Send));
            params.extend(quote!(#ident,));
        }
    }
    let mut methods = vec![];
    let mut fields = TokenStream::new();
    let mut from_fields = TokenStream::new();
    let mut shim_items = TokenStream::new();
    let mut reflected_items = TokenStream::new();
    for item in &item.items {
        use TraitItem::Method;
        if let Method(method) = item {
            let mut arg_types = vec![];
            if methods.len() == 255 {
                return quote_spanned!(item.span() => const #hygiene: () = { compile_error!("traits with more than {} methods are not supported", ::vessels::reflection::MethodIndex::MAX) };);
            }
            let sig = method.sig.clone();
            let ident = &method.sig.ident;
            let mut receiver = None;
            let mut args = TokenStream::new();
            let inputs = &method.sig.inputs;
            for input in inputs {
                use FnArg::{Receiver, Typed};
                if let Typed(ty) = input {
                    let ty = &ty.ty;
                    arg_types.push(ty.into_token_stream());
                    args.extend(quote!(#ty,));
                } else if let Receiver(r) = input {
                    receiver = Some(r.clone());
                }
            }
            if receiver.is_none() {
                return quote_spanned!(method.span() => const #hygiene: () = { compile_error!("object-safe trait methods must have a borrowed receiver") };);
            }
            let receiver = receiver.unwrap();
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
            let idx = methods.len();
            let call_method = if receiver.mutability.is_some() {
                quote!(call_mut)
            } else {
                quote!(call)
            };
            let arg_idents: Vec<_> = inputs.iter().map(|arg| arg.clone()).collect();
            reflected_items.extend(quote! {
                #sig {
                    *::std::boxed::Box::<dyn ::std::any::Any + Send>::downcast(self.#call_method(#idx as ::vessels::reflection::MethodIndex, vec![#( Box::new(#arg_idents) as Box<dyn ::std::any::Any + Send> )*]).unwrap()).unwrap()
                }
            });
            use ReturnType::Type;
            methods.push((
                arg_types,
                match &method.sig.output {
                    Type(_, ty) => ty.clone().into_token_stream(),
                    _ => TokenStream::new(),
                },
                method.sig.ident.clone(),
                receiver,
            ));
        }
    }
    let methods_count = methods.len();
    let mut types_arms = TokenStream::new();
    let mut call_arms = TokenStream::new();
    let mut call_mut_arms = TokenStream::new();
    let mut name_arms = TokenStream::new();
    let mut index_name_arms = TokenStream::new();
    let mut receiver_arms = TokenStream::new();
    for (idx, method) in methods.iter().enumerate() {
        let idx = idx as MethodIndex;
        let output = &method.1;
        let args = &method.0;
        let ident = &method.2;
        let name = &method.2.to_string();
        types_arms.extend(quote! {
            #idx => {
                Ok(::vessels::reflection::MethodTypes {
                    arguments: vec![#(::std::any::TypeId::of::<#args>()),*],
                    output: ::std::any::TypeId::of::<#output>()
                })
            },
        });
        name_arms.extend(quote! {
            #name => {
                Ok(#idx)
            }
        });
        let mut arg_stream = TokenStream::new();
        for (idx, arg) in args.iter().enumerate() {
            let o_idx = idx as MethodIndex;
            arg_stream.extend(quote! {
                *::std::boxed::Box::<dyn ::std::any::Any + Send>::downcast::<#arg>(args.pop().unwrap()).map_err(|_| ::vessels::reflection::CallError::Type(#o_idx))?
            })
        }
        let args_len = args.len();
        let mutability = method.3.mutability.is_some();
        let arm = quote! {
            #idx => {
                if args.len() == #args_len {
                    Ok(Box::new(self.#ident(#arg_stream)) as Box<dyn ::std::any::Any + Send>)
                } else {
                    Err(::vessels::reflection::CallError::ArgumentCount(::vessels::reflection::ArgumentCountError {
                        got: args.len(),
                        expected: #args_len
                    }))
                }
            }
        };
        let fail_arm = quote! {
            #idx => {
                Err(::vessels::reflection::CallError::IncorrectReceiver(#mutability))
            }
        };
        if mutability {
            receiver_arms.extend(quote! {
                #idx => {
                    Ok(Mutable)
                }
            });
            call_mut_arms.extend(arm);
            call_arms.extend(fail_arm);
        } else {
            receiver_arms.extend(quote! {
                #idx => {
                    Ok(Immutable)
                }
            });
            call_arms.extend(arm);
            call_mut_arms.extend(fail_arm);
        }
        index_name_arms.extend(quote! {
            #idx => {
                Ok(#name.to_owned())
            }
        })
    }
    let mut supertrait_impls = TokenStream::new();
    let mut upcast_arms = TokenStream::new();
    let mut supertrait_ids = TokenStream::new();
    let mut derive_param_bounds = TokenStream::new();
    for (idx, supertrait) in item.supertraits.iter().enumerate() {
        use TypeParamBound::Trait;
        if let Trait(supertrait) = supertrait {
            let id = format_ident!("_SUPERTRAIT_{}_", idx);
            let path = supertrait.path.clone();
            fields.extend(quote! {
                #id: ::std::sync::Arc<::std::sync::Mutex<::std::boxed::Box<<dyn #path as ::vessels::reflection::Reflected>::Shim>>>,
            });
            supertrait_impls.extend(quote! {
                impl<#kind_bounded_params> ::vessels::reflection::Trait<dyn #path> for _DERIVED_Shim {
                    fn call(&self, index: ::vessels::reflection::MethodIndex, mut args: Vec<::std::boxed::Box<dyn ::std::any::Any + Send>>) -> ::std::result::Result<std::boxed::Box<dyn ::std::any::Any + Send>, ::vessels::reflection::CallError> {
                        (self.#id.lock().unwrap().as_ref() as &dyn #path).call(index, args)
                    }
                    fn call_mut(&mut self, index: ::vessels::reflection::MethodIndex, mut args: Vec<::std::boxed::Box<dyn ::std::any::Any + Send>>) -> ::std::result::Result<std::boxed::Box<dyn ::std::any::Any + Send>, ::vessels::reflection::CallError> {
                        (self.#id.lock().unwrap().as_mut() as &mut dyn #path).call_mut(index, args)
                    }
                    fn by_name(&self, name: &'_ str) -> ::std::result::Result<::vessels::reflection::MethodIndex, ::vessels::reflection::NameError> {
                        (self.#id.lock().unwrap().as_ref() as &dyn #path).by_name(name)
                    }
                    fn count(&self) -> ::vessels::reflection::MethodIndex {
                        (self.#id.lock().unwrap().as_ref() as &dyn #path).count()
                    }
                    fn receiver(&self, index: ::vessels::reflection::MethodIndex) -> Result<::vessels::reflection::Receiver, ::vessels::reflection::OutOfRangeError> {
                        (self.#id.lock().unwrap().as_ref() as &dyn #path).receiver(index)
                    }
                    fn name_of(&self, index: ::vessels::reflection::MethodIndex) -> ::std::result::Result<::std::string::String, ::vessels::reflection::OutOfRangeError> {
                        (self.#id.lock().unwrap().as_ref() as &dyn #path).name_of(index)
                    }
                    fn types(&self, index: ::vessels::reflection::MethodIndex) -> ::std::result::Result<::vessels::reflection::MethodTypes, ::vessels::reflection::OutOfRangeError> {
                        (self.#id.lock().unwrap().as_ref() as &dyn #path).types(index)
                    }
                    fn supertraits(&self) -> ::std::vec::Vec<::std::any::TypeId> {
                        (self.#id.lock().unwrap().as_ref() as &dyn #path).supertraits()
                    }
                    fn upcast(self: Box<Self>, ty: ::std::any::TypeId) -> ::std::result::Result<::std::boxed::Box<dyn ::std::any::Any + Send>, ::vessels::reflection::UpcastError> {
                        (::std::sync::Arc::try_unwrap(self.#id).map_err(|_| panic!()).unwrap().into_inner().unwrap() as Box<dyn #path>).upcast(ty)
                    }
                }
            });
            from_fields.extend(quote! {
                #id: ::std::sync::Arc::new(::std::sync::Mutex::new(Box::new(<dyn #path as ::vessels::reflection::Reflected>::Shim::from_instance(object)))),
            });
            derive_param_bounds.extend(quote! {
                + #path
            });
            supertrait_ids.extend(quote! {
                ::std::any::TypeId::of::<dyn #path>()
            });
        }
    }
    item.supertraits.push(parse_quote!(::std::marker::Send));
    quote! {
        #[allow(non_upper_case_globals)]
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        const #hygiene: () = {
            #[derive(::vessels::Kind)]
            pub struct _DERIVED_Shim<#kind_bounded_params> {
                #fields
                _marker: ::std::marker::PhantomData<(#params)>
            }
            impl<#kind_bounded_params> _DERIVED_Shim<#params> {
                pub fn from_instance<T: ?Sized + #ident<#params> + 'static>(object: ::std::sync::Arc<::std::sync::Mutex<Box<T>>>) -> Self {
                    _DERIVED_Shim {
                       #from_fields
                       _marker: ::std::marker::PhantomData
                    }
                }
            }
            #supertrait_impls
            impl<#kind_bounded_params> #ident<#params> for _DERIVED_Shim<#params> {
                #shim_items
            }
            impl<#kind_bounded_params> ::vessels::reflection::Reflected for dyn #ident<#params> {
                type Shim = _DERIVED_Shim;
                const DO_NOT_IMPLEMENT_THIS_MARKER_TRAIT_MANUALLY: () = ();
            }
            impl<#kind_bounded_params DERIVEPARAM: Send + ::vessels::reflection::Trait<dyn #ident<#params>> #derive_param_bounds> #ident<#params> for DERIVEPARAM {
                #reflected_items
            }
            impl<#kind_bounded_params> ::vessels::reflection::Trait<dyn #ident<#params>> for dyn #ident<#params> {
                fn call(&self, index: ::vessels::reflection::MethodIndex, mut args: Vec<::std::boxed::Box<dyn ::std::any::Any + Send>>) -> ::std::result::Result<std::boxed::Box<dyn ::std::any::Any + Send>, ::vessels::reflection::CallError> {
                    args.reverse();
                    match index {
                        #call_arms
                        _ => Err(::vessels::reflection::CallError::OutOfRange(::vessels::reflection::OutOfRangeError {
                            index,
                        })),
                    }
                }
                fn call_mut(&mut self, index: ::vessels::reflection::MethodIndex, mut args: Vec<::std::boxed::Box<dyn ::std::any::Any + Send>>) -> ::std::result::Result<std::boxed::Box<dyn ::std::any::Any + Send>, ::vessels::reflection::CallError> {
                    args.reverse();
                    match index {
                        #call_mut_arms
                        _ => Err(::vessels::reflection::CallError::OutOfRange(::vessels::reflection::OutOfRangeError {
                            index,
                        })),
                    }
                }
                fn by_name(&self, name: &'_ str) -> ::std::result::Result<::vessels::reflection::MethodIndex, ::vessels::reflection::NameError> {
                    match name {
                        #name_arms
                        _ => {
                            Err(::vessels::reflection::NameError {
                                name: name.to_owned(),
                            })
                        }
                    }
                }
                fn count(&self) -> ::vessels::reflection::MethodIndex {
                    #methods_count as ::vessels::reflection::MethodIndex
                }
                fn receiver(&self, index: ::vessels::reflection::MethodIndex) -> Result<::vessels::reflection::Receiver, ::vessels::reflection::OutOfRangeError> {
                    use ::vessels::reflection::Receiver::{Mutable, Immutable};
                    match index {
                        #receiver_arms
                        _ => {
                            Err(::vessels::reflection::OutOfRangeError {
                                index,
                            })
                        }
                    }
                }
                fn name_of(&self, index: ::vessels::reflection::MethodIndex) -> ::std::result::Result<::std::string::String, ::vessels::reflection::OutOfRangeError> {
                    match index {
                        #index_name_arms
                        _ => {
                            Err(::vessels::reflection::OutOfRangeError {
                                index,
                            })
                        }
                    }
                }
                fn types(&self, index: ::vessels::reflection::MethodIndex) -> ::std::result::Result<::vessels::reflection::MethodTypes, ::vessels::reflection::OutOfRangeError> {
                    match index {
                        #types_arms
                        _ => {
                            Err(::vessels::reflection::OutOfRangeError {
                                index,
                            })
                        }
                    }
                }
                fn supertraits(&self) -> ::std::vec::Vec<::std::any::TypeId> {
                    vec![#supertrait_ids]
                }
                fn upcast(self: Box<Self>, ty: ::std::any::TypeId) -> ::std::result::Result<::std::boxed::Box<dyn ::std::any::Any + Send>, ::vessels::reflection::UpcastError> {
                    match ty {
                        #upcast_arms
                        _ => {
                            Err(::vessels::reflection::UpcastError {
                                supertrait: ty,
                            })
                        }
                    }
                }
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
                        channel.send(channel.fork::<_DERIVED_Shim<#params>>(_DERIVED_Shim::from_instance(::std::sync::Arc::new(::std::sync::Mutex::new(self)))).await.unwrap()).unwrap_or_else(|_| panic!()).await;
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
