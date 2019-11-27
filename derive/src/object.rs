use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{
    parse_quote, punctuated::Punctuated, spanned::Spanned, FnArg, GenericParam, ItemTrait, PatType,
    Receiver, ReturnType, Token, TraitItem, TypeParamBound,
};

type MethodIndex = u8;

enum Recv {
    Reference(Receiver),
    Move(PatType),
}

impl Recv {
    fn is_mutable(&self) -> Option<bool> {
        use Recv::{Move, Reference};
        match self {
            Reference(receiver) => Some(receiver.mutability.is_some()),
            Move(_) => None,
        }
    }
}

pub fn build(_: TokenStream, item: &mut ItemTrait) -> TokenStream {
    let mut params = TokenStream::new();
    let ident = &item.ident;
    let vis = &item.vis;
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
            parameter.bounds.push(parse_quote!(Sync));
            params.extend(quote!(#ident,));
        }
    }
    let mut methods = vec![];
    let mut fields = TokenStream::new();
    let mut from_fields = TokenStream::new();
    let mut shim_items = TokenStream::new();
    let mut reflected_items = TokenStream::new();
    for item in &item.items {
        use TraitItem::{Method, Type};
        if let Type(_) = item {
            return quote_spanned!(item.span() => const #hygiene: () = { compile_error!("associated types are not supported") };);
        }
        if let Method(method) = item {
            let mut arg_types = vec![];
            if methods.len() == 255 {
                return quote_spanned!(item.span() => const #hygiene: () = { compile_error!("traits with more than {} methods are not supported", ::vessels::reflect::MethodIndex::MAX) };);
            }
            let sig = method.sig.clone();
            let mident = &method.sig.ident;
            let mut receiver = None;
            let mut args = TokenStream::new();
            let inputs = &method.sig.inputs;
            let boxed_receiver: PatType = if let FnArg::Typed(ty) = parse_quote!(self: Box<Self>) {
                ty
            } else {
                panic!("could not parse hard-coded move receiver")
            };
            for input in inputs {
                use FnArg::{Receiver, Typed};
                if let Typed(ty) = input {
                    if ty == &boxed_receiver {
                        receiver = Some(Recv::Move(ty.clone()));
                        continue;
                    }
                    let ty = &ty.ty;
                    arg_types.push(ty.into_token_stream());
                    args.extend(quote!(#ty,));
                } else if let Receiver(r) = input {
                    receiver = Some(Recv::Reference(r.clone()));
                }
            }
            if receiver.is_none() {
                return quote_spanned!(method.span() => const #hygiene: () = { compile_error!("object-safe trait methods must have a borrowed or `Box<Self>` receiver") };);
            }
            let receiver = receiver.unwrap();
            let output = &method.sig.output;
            let ty;
            let lock;
            if receiver.is_mutable().is_some() {
                lock = quote!(object.lock().unwrap());
                ty = quote!(Fn(#args));
            } else {
                lock = quote!(::std::sync::Arc::try_unwrap(object)
                    .map_err(|_| panic!("arc is not held exclusively"))
                    .unwrap()
                    .into_inner()
                    .unwrap());
                ty = quote!(FnOnce(#args));
            }
            fields.extend(quote! {
                #mident: ::std::boxed::Box<dyn #ty #output + Send + Sync>,
            });
            let inputs: Punctuated<_, Token![,]> = inputs
                .iter()
                .filter_map(|arg| {
                    use FnArg::Typed;
                    if let Typed(ty) = arg {
                        if ty == &boxed_receiver {
                            return None;
                        }
                        Some(ty.pat.clone())
                    } else {
                        None
                    }
                })
                .collect();
            from_fields.extend(quote! {
                #mident: { let object = object.clone(); ::std::boxed::Box::new(move |#inputs| #lock.#mident(#inputs)) },
            });
            shim_items.extend(quote! {
                #sig {
                    (self.#mident)(#inputs)
                }
            });
            let idx = methods.len();
            let call_method = if let Some(mutability) = receiver.is_mutable() {
                if mutability {
                    quote!(call_mut)
                } else {
                    quote!(call)
                }
            } else {
                quote!(call_move)
            };
            let arg_idents: Vec<_> = inputs.iter().map(|arg| arg.clone()).collect();
            reflected_items.extend(quote! {
                #sig {
                    *::std::boxed::Box::<dyn ::std::any::Any>::downcast(::vessels::reflect::Trait::<dyn #ident<#params>>::#call_method(self, #idx as ::vessels::reflect::MethodIndex, vec![#( ::std::boxed::Box::new(#arg_idents) as ::std::boxed::Box<dyn ::std::any::Any + Send + Sync> ),*]).unwrap()).unwrap()
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
    let mut call_move_arms = TokenStream::new();
    let mut name_arms = TokenStream::new();
    let mut index_name_arms = TokenStream::new();
    for (idx, method) in methods.iter().enumerate() {
        let idx = idx as MethodIndex;
        let output = &method.1;
        let args = &method.0;
        let mident = &method.2;
        let name = &method.2.to_string();
        let mutability = method.3.is_mutable();
        let receiver;
        if let Some(mutability) = mutability {
            if mutability {
                receiver = quote!(::vessels::reflect::Receiver::Mutable);
            } else {
                receiver = quote!(::vessels::reflect::Receiver::Immutable);
            }
        } else {
            receiver = quote!(::vessels::reflect::Receiver::Owned);
        }
        types_arms.extend(quote! {
            #idx => {
                Ok(::vessels::reflect::MethodTypes {
                    arguments: vec![#(::std::any::TypeId::of::<#args>()),*],
                    output: ::std::any::TypeId::of::<#output>(),
                    receiver: #receiver
                })
            },
        });
        name_arms.extend(quote! {
            #name => {
                Ok(#idx)
            },
        });
        let mut arg_stream = TokenStream::new();
        for (idx, arg) in args.iter().enumerate() {
            let o_idx = idx as MethodIndex;
            arg_stream.extend(quote! {
                *::std::boxed::Box::<dyn ::std::any::Any>::downcast::<#arg>(args.pop().unwrap()).map_err(|_| ::vessels::reflect::CallError::Type(#o_idx))?,
            })
        }
        let args_len = args.len();
        let arm = quote! {
            #idx => {
                if args.len() == #args_len {
                    Ok(::std::boxed::Box::new(self.#mident(#arg_stream)) as ::std::boxed::Box<dyn ::std::any::Any + Send + Sync>)
                } else {
                    Err(::vessels::reflect::CallError::ArgumentCount(::vessels::reflect::ArgumentCountError {
                        got: args.len(),
                        expected: #args_len
                    }))
                }
            }
        };
        let fail_arm = quote! {
            #idx => {
                Err(::vessels::reflect::CallError::IncorrectReceiver(#receiver))
            },
        };
        if let Some(mutability) = mutability {
            if mutability {
                call_mut_arms.extend(arm);
                call_arms.extend(fail_arm.clone());
                call_move_arms.extend(fail_arm);
            } else {
                call_arms.extend(arm);
                call_mut_arms.extend(fail_arm.clone());
                call_move_arms.extend(fail_arm);
            }
        } else {
            call_move_arms.extend(arm);
            call_arms.extend(fail_arm.clone());
            call_mut_arms.extend(fail_arm);
        }
        index_name_arms.extend(quote! {
            #idx => {
                Ok(#name.to_owned())
            },
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
                #id: ::std::sync::Arc<::std::sync::Mutex<::std::boxed::Box<<dyn #path as ::vessels::reflect::Reflected>::Shim>>>,
            });
            supertrait_impls.extend(quote! {
                impl<#kind_bounded_params> ::vessels::reflect::Trait<dyn #path> for _DERIVED_Shim<#params> {
                    fn call(&self, index: ::vessels::reflect::MethodIndex, mut args: Vec<::std::boxed::Box<dyn ::std::any::Any + Send + Sync>>) -> ::std::result::Result<std::boxed::Box<dyn ::std::any::Any + Send + Sync>, ::vessels::reflect::CallError> {
                        ::vessels::reflect::Trait::<dyn #path>::call(self.#id.lock().unwrap().as_ref() as &dyn #path, index, args)
                    }
                    fn call_mut(&mut self, index: ::vessels::reflect::MethodIndex, mut args: Vec<::std::boxed::Box<dyn ::std::any::Any + Send + Sync>>) -> ::std::result::Result<std::boxed::Box<dyn ::std::any::Any + Send + Sync>, ::vessels::reflect::CallError> {
                        ::vessels::reflect::Trait::<dyn #path>::call_mut(self.#id.lock().unwrap().as_mut() as &mut dyn #path, index, args)
                    }
                    fn call_move(self: Box<Self>, index: ::vessels::reflect::MethodIndex, mut args: Vec<::std::boxed::Box<dyn ::std::any::Any + Send + Sync>>) -> ::std::result::Result<std::boxed::Box<dyn ::std::any::Any + Send + Sync>, ::vessels::reflect::CallError> {
                        ::vessels::reflect::Trait::<dyn #path>::call_move(::std::sync::Arc::try_unwrap(self.#id).map_err(|_| panic!("arc is not held exclusively")).unwrap().into_inner().unwrap() as Box<dyn #path>, index, args)
                    }
                    fn by_name(&self, name: &'_ str) -> ::std::result::Result<::vessels::reflect::MethodIndex, ::vessels::reflect::NameError> {
                        ::vessels::reflect::Trait::<dyn #path>::by_name(self.#id.lock().unwrap().as_ref() as &dyn #path, name)
                    }
                    fn count(&self) -> ::vessels::reflect::MethodIndex {
                        ::vessels::reflect::Trait::<dyn #path>::count(self.#id.lock().unwrap().as_ref() as &dyn #path)
                    }
                    fn name_of(&self, index: ::vessels::reflect::MethodIndex) -> ::std::result::Result<::std::string::String, ::vessels::reflect::OutOfRangeError> {
                        ::vessels::reflect::Trait::<dyn #path>::name_of(self.#id.lock().unwrap().as_ref() as &dyn #path, index)
                    }
                    fn types(&self, index: ::vessels::reflect::MethodIndex) -> ::std::result::Result<::vessels::reflect::MethodTypes, ::vessels::reflect::OutOfRangeError> {
                        ::vessels::reflect::Trait::<dyn #path>::types(self.#id.lock().unwrap().as_ref() as &dyn #path, index)
                    }
                    fn this(&self) -> ::std::any::TypeId {
                        ::vessels::reflect::Trait::<dyn #path>::this(self.#id.lock().unwrap().as_ref() as &dyn #path)
                    }
                    fn name(&self) -> ::std::string::String {
                        ::vessels::reflect::Trait::<dyn #path>::name(self.#id.lock().unwrap().as_ref() as &dyn #path)
                    }
                    fn supertraits(&self) -> ::std::vec::Vec<::std::any::TypeId> {
                        ::vessels::reflect::Trait::<dyn #path>::supertraits(self.#id.lock().unwrap().as_ref() as &dyn #path)
                    }
                    fn upcast_erased(self: ::std::boxed::Box<Self>, ty: ::std::any::TypeId) -> ::std::result::Result<::std::boxed::Box<dyn ::vessels::reflect::Erased>, ::vessels::reflect::CastError> {
                        ::vessels::reflect::Trait::<dyn #path>::upcast_erased(::std::sync::Arc::try_unwrap(self.#id).map_err(|_| panic!("arc is not held exclusively")).unwrap().into_inner().unwrap() as ::std::boxed::Box<dyn #path>, ty)
                    }
                    fn erase(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn ::vessels::reflect::Erased> {
                        ::vessels::reflect::Trait::<dyn #path>::erase(::std::sync::Arc::try_unwrap(self.#id).map_err(|_| panic!("arc is not held exclusively")).unwrap().into_inner().unwrap() as ::std::boxed::Box<dyn #path>)
                    }
                }
            });
            from_fields.extend(quote! {
                #id: ::std::sync::Arc::new(::std::sync::Mutex::new(::std::boxed::Box::new(<dyn #path as ::vessels::reflect::Reflected>::Shim::from_instance(object)))),
            });
            derive_param_bounds.extend(quote! {
                + #path
            });
            supertrait_ids.extend(quote! {
                ::std::any::TypeId::of::<dyn #path>(),
            });
            upcast_arms.extend(quote! {
                if ty == ::std::any::TypeId::of::<dyn #path>() {
                    return Ok(::std::boxed::Box::new(<dyn #path as ::vessels::reflect::Reflected>::ErasedShim::from(Box::new(<dyn #path as ::vessels::reflect::Reflected>::Shim::from_instance(::std::sync::Arc::new(::std::sync::Mutex::new(self)))) as Box<dyn #path>)) as ::std::boxed::Box<dyn ::vessels::reflect::Erased>);
                }
            })
        }
    }
    item.supertraits.push(parse_quote!(::std::marker::Send));
    item.supertraits.push(parse_quote!(::std::marker::Sync));
    let name = ident.to_string();
    quote! {
        #[allow(non_upper_case_globals)]
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        const #hygiene: () = {
            #[derive(::vessels::Kind)]
            #vis struct _DERIVED_Shim<#kind_bounded_params> {
                #fields
                _marker: ::std::marker::PhantomData<(#params)>
            }
            impl<#kind_bounded_params> _DERIVED_Shim<#params> {
                #vis fn from_instance<DERIVEPARAM: ?Sized + #ident<#params> + 'static>(object: ::std::sync::Arc<::std::sync::Mutex<::std::boxed::Box<DERIVEPARAM>>>) -> Self {
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
            impl<#kind_bounded_params> ::vessels::reflect::Reflected for dyn #ident<#params> {
                #[doc(hidden)]
                type Shim = _DERIVED_Shim<#params>;
                #[doc(hidden)]
                type ErasedShim = _DERIVED_ErasedShim<#params>;
                #[doc(hidden)]
                const DO_NOT_IMPLEMENT_THIS_MARKER_TRAIT_MANUALLY: () = ();
            }
            impl<#kind_bounded_params> From<Box<dyn #ident<#params>>> for _DERIVED_ErasedShim<#params> {
                fn from(input: Box<dyn #ident<#params>>) -> _DERIVED_ErasedShim<#params> {
                    _DERIVED_ErasedShim(input)
                }
            }
            #[doc(hidden)]
            impl<DERIVEPARAM: 'static + Sync + Send + ::vessels::reflect::Trait<dyn #ident<#params>> #derive_param_bounds, #kind_bounded_params> #ident<#params> for DERIVEPARAM {
                #reflected_items
            }
            #vis struct _DERIVED_ErasedShim<#kind_bounded_params>(Box<dyn #ident<#params>>);
            impl<#kind_bounded_params> ::vessels::reflect::Erased for _DERIVED_ErasedShim<#params> {
                fn cast(self: ::std::boxed::Box<Self>, ty: ::std::any::TypeId) -> ::std::result::Result<::std::boxed::Box<dyn ::std::any::Any + Send + Sync>, ::vessels::reflect::CastError> {
                    if ty == ::std::any::TypeId::of::<dyn #ident<#params>>() {
                        Ok(::std::boxed::Box::new(self.0 as ::std::boxed::Box<dyn #ident<#params>>) as ::std::boxed::Box<dyn ::std::any::Any + Send + Sync>)
                    } else {
                        Err(::vessels::reflect::CastError {
                            target: ty,
                        })
                    }
                }
            }
            impl<#kind_bounded_params> ::vessels::reflect::Trait<::vessels::reflect::SomeTrait> for _DERIVED_ErasedShim<#params> {
                fn call(&self, index: ::vessels::reflect::MethodIndex, mut args: Vec<::std::boxed::Box<dyn ::std::any::Any + Send + Sync>>) -> ::std::result::Result<std::boxed::Box<dyn ::std::any::Any + Send + Sync>, ::vessels::reflect::CallError> {
                    ::vessels::reflect::Trait::call(self.0.as_ref(), index, args)
                }
                fn call_mut(&mut self, index: ::vessels::reflect::MethodIndex, mut args: Vec<::std::boxed::Box<dyn ::std::any::Any + Send + Sync>>) -> ::std::result::Result<std::boxed::Box<dyn ::std::any::Any + Send + Sync>, ::vessels::reflect::CallError> {
                    ::vessels::reflect::Trait::call_mut(self.0.as_mut(), index, args)
                }
                fn call_move(self: Box<Self>, index: ::vessels::reflect::MethodIndex, mut args: Vec<::std::boxed::Box<dyn ::std::any::Any + Send + Sync>>) -> ::std::result::Result<std::boxed::Box<dyn ::std::any::Any + Send + Sync>, ::vessels::reflect::CallError> {
                    ::vessels::reflect::Trait::call_move(self.0, index, args)
                }
                fn by_name(&self, name: &'_ str) -> ::std::result::Result<::vessels::reflect::MethodIndex, ::vessels::reflect::NameError> {
                    ::vessels::reflect::Trait::by_name(self.0.as_ref(), name)
                }
                fn count(&self) -> ::vessels::reflect::MethodIndex {
                    ::vessels::reflect::Trait::count(self.0.as_ref())
                }
                fn name_of(&self, index: ::vessels::reflect::MethodIndex) -> ::std::result::Result<::std::string::String, ::vessels::reflect::OutOfRangeError> {
                    ::vessels::reflect::Trait::name_of(self.0.as_ref(), index)
                }
                fn this(&self) -> ::std::any::TypeId {
                    ::vessels::reflect::Trait::this(self.0.as_ref())
                }
                fn name(&self) -> ::std::string::String {
                    ::vessels::reflect::Trait::name(self.0.as_ref())
                }
                fn types(&self, index: ::vessels::reflect::MethodIndex) -> ::std::result::Result<::vessels::reflect::MethodTypes, ::vessels::reflect::OutOfRangeError> {
                    ::vessels::reflect::Trait::types(self.0.as_ref(), index)
                }
                fn supertraits(&self) -> ::std::vec::Vec<::std::any::TypeId> {
                    ::vessels::reflect::Trait::supertraits(self.0.as_ref())
                }
                fn upcast_erased(self: ::std::boxed::Box<Self>, ty: ::std::any::TypeId) -> ::std::result::Result<::std::boxed::Box<dyn ::vessels::reflect::Erased>, ::vessels::reflect::CastError> {
                    ::vessels::reflect::Trait::upcast_erased(self, ty)
                }
                fn erase(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn ::vessels::reflect::Erased> {
                    ::vessels::reflect::Trait::erase(self)
                }
            }
            #[doc(hidden)]
            impl<#kind_bounded_params> ::vessels::reflect::Trait<dyn #ident<#params>> for dyn #ident<#params> {
                fn call(&self, index: ::vessels::reflect::MethodIndex, mut args: Vec<::std::boxed::Box<dyn ::std::any::Any + Send + Sync>>) -> ::std::result::Result<std::boxed::Box<dyn ::std::any::Any + Send + Sync>, ::vessels::reflect::CallError> {
                    args.reverse();
                    match index {
                        #call_arms
                        _ => Err(::vessels::reflect::CallError::OutOfRange(::vessels::reflect::OutOfRangeError {
                            index,
                        })),
                    }
                }
                fn call_mut(&mut self, index: ::vessels::reflect::MethodIndex, mut args: Vec<::std::boxed::Box<dyn ::std::any::Any + Send + Sync>>) -> ::std::result::Result<std::boxed::Box<dyn ::std::any::Any + Send + Sync>, ::vessels::reflect::CallError> {
                    args.reverse();
                    match index {
                        #call_mut_arms
                        _ => Err(::vessels::reflect::CallError::OutOfRange(::vessels::reflect::OutOfRangeError {
                            index,
                        })),
                    }
                }
                fn call_move(self: Box<Self>, index: ::vessels::reflect::MethodIndex, mut args: Vec<::std::boxed::Box<dyn ::std::any::Any + Sync + Send>>) -> ::std::result::Result<std::boxed::Box<dyn ::std::any::Any + Send + Sync>, ::vessels::reflect::CallError> {
                    args.reverse();
                    match index {
                        #call_move_arms
                        _ => Err(::vessels::reflect::CallError::OutOfRange(::vessels::reflect::OutOfRangeError {
                            index,
                        })),
                    }
                }
                fn by_name(&self, name: &'_ str) -> ::std::result::Result<::vessels::reflect::MethodIndex, ::vessels::reflect::NameError> {
                    match name {
                        #name_arms
                        _ => {
                            Err(::vessels::reflect::NameError {
                                name: name.to_owned(),
                            })
                        }
                    }
                }
                fn count(&self) -> ::vessels::reflect::MethodIndex {
                    #methods_count as ::vessels::reflect::MethodIndex
                }
                fn name_of(&self, index: ::vessels::reflect::MethodIndex) -> ::std::result::Result<::std::string::String, ::vessels::reflect::OutOfRangeError> {
                    match index {
                        #index_name_arms
                        _ => {
                            Err(::vessels::reflect::OutOfRangeError {
                                index,
                            })
                        }
                    }
                }
                fn types(&self, index: ::vessels::reflect::MethodIndex) -> ::std::result::Result<::vessels::reflect::MethodTypes, ::vessels::reflect::OutOfRangeError> {
                    match index {
                        #types_arms
                        _ => {
                            Err(::vessels::reflect::OutOfRangeError {
                                index,
                            })
                        }
                    }
                }
                fn this(&self) -> ::std::any::TypeId {
                    ::std::any::TypeId::of::<dyn #ident<#params>>()
                }
                fn name(&self) -> ::std::string::String {
                    #name.to_owned()
                }
                fn supertraits(&self) -> ::std::vec::Vec<::std::any::TypeId> {
                    vec![#supertrait_ids]
                }
                fn upcast_erased(self: ::std::boxed::Box<Self>, ty: ::std::any::TypeId) -> ::std::result::Result<::std::boxed::Box<dyn ::vessels::reflect::Erased>, ::vessels::reflect::CastError> {
                    #upcast_arms
                    Err(::vessels::reflect::CastError {
                        target: ty,
                    })
                }
                fn erase(self: ::std::boxed::Box<Self>) -> Box<dyn ::vessels::reflect::Erased> {
                    Box::new(_DERIVED_ErasedShim::from(self)) as ::std::boxed::Box<dyn ::vessels::reflect::Erased>
                }
            }
            impl<#kind_bounded_params> ::vessels::Kind for ::std::boxed::Box<dyn #ident<#params>> {
                type ConstructItem = ::vessels::channel::ForkHandle;
                type ConstructError = ::vessels::void::Void;
                type ConstructFuture = ::vessels::kind::Future<::vessels::kind::ConstructResult<Self>>;
                type DeconstructItem = ();
                type DeconstructError = ::vessels::void::Void;
                type DeconstructFuture = ::vessels::kind::Future<::vessels::kind::DeconstructResult<Self>>;

                fn deconstruct<C: ::vessels::channel::Channel<<Self as ::vessels::Kind>::DeconstructItem, <Self as ::vessels::Kind>::ConstructItem>>(
                    self,
                    mut channel: C,
                ) -> <Self as ::vessels::Kind>::DeconstructFuture {
                    use ::vessels::futures::{SinkExt, TryFutureExt};
                    ::std::boxed::Box::pin(async move {
                        channel.send(channel.fork::<_DERIVED_Shim<#params>>(_DERIVED_Shim::from_instance(::std::sync::Arc::new(::std::sync::Mutex::new(self)))).await.unwrap()).unwrap_or_else(|_| panic!("arc is not held exclusively")).await;
                        Ok(())
                    })
                }

                fn construct<C: ::vessels::channel::Channel<<Self as ::vessels::Kind>::ConstructItem, <Self as ::vessels::Kind>::DeconstructItem>>(
                    mut channel: C,
                ) -> <Self as ::vessels::Kind>::ConstructFuture {
                    use ::vessels::futures::StreamExt;
                    ::std::boxed::Box::pin(async move {
                        let handle = channel.next().await.unwrap();
                        Ok(::std::boxed::Box::new(channel.get_fork::<_DERIVED_Shim<#params>>(handle).await.unwrap()) as ::std::boxed::Box<dyn #ident<#params>>)
                    })
                }
            }
        };
    }
}
