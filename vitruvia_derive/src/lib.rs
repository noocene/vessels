#![recursion_limit = "512"]

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::{quote, quote_spanned, ToTokens};

use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Paren, Field, Fields, FieldsUnnamed, FnArg,
    Ident, ItemTrait, Path, PathArguments, PathSegment, ReturnType, TraitBound, TraitBoundModifier,
    TraitItem, TraitItemMethod, Type, TypeParamBound, TypeVerbatim, Variant, Visibility,
};

#[derive(Debug)]
struct Procedure {
    arg_types: Vec<Type>,
    mut_receiver: bool,
    ident: Option<Ident>,
    return_type: Option<Type>,
}

fn generate_enum(methods: &[Procedure]) -> Vec<Variant> {
    methods
        .iter()
        .map(|method| Variant {
            ident: method.ident.clone().unwrap(),
            attrs: vec![],
            discriminant: None,
            fields: {
                let mut fields = Punctuated::new();
                for ty in &method.arg_types {
                    fields.push(Field {
                        attrs: vec![],
                        ident: None,
                        ty: ty.clone(),
                        colon_token: None,
                        vis: Visibility::Inherited,
                    });
                }
                fields.push(Field {
                    attrs: vec![],
                    ident: None,
                    ty: Type::Verbatim(TypeVerbatim {
                        tts: quote! {
                            u64
                        },
                    }),
                    colon_token: None,
                    vis: Visibility::Inherited,
                });
                Fields::Unnamed(FieldsUnnamed {
                    paren_token: Paren(Span::call_site()),
                    unnamed: fields,
                })
            },
        })
        .collect::<Vec<_>>()
}

fn generate_return_variants(methods: &[Procedure]) -> Vec<Variant> {
    methods
        .iter()
        .map(|method| Variant {
            ident: method.ident.clone().unwrap(),
            attrs: vec![],
            discriminant: None,
            fields: {
                let mut fields = Punctuated::new();
                let ty = &method.return_type;
                fields.push(Field {
                    attrs: vec![],
                    ident: None,
                    ty: Type::Verbatim(TypeVerbatim {
                        tts: quote! {
                            <#ty as ::vitruvia::protocol::Value>::Item
                        },
                    }),
                    colon_token: None,
                    vis: Visibility::Inherited,
                });
                fields.push(Field {
                    attrs: vec![],
                    ident: None,
                    ty: Type::Verbatim(TypeVerbatim {
                        tts: quote! {
                            u64
                        },
                    }),
                    colon_token: None,
                    vis: Visibility::Inherited,
                });
                fields.push(Field {
                    attrs: vec![],
                    ident: None,
                    ty: Type::Verbatim(TypeVerbatim {
                        tts: quote! {
                            u64
                        },
                    }),
                    colon_token: None,
                    vis: Visibility::Inherited,
                });
                Fields::Unnamed(FieldsUnnamed {
                    paren_token: Paren(Span::call_site()),
                    unnamed: fields,
                })
            },
        })
        .collect::<Vec<_>>()
}

fn generate_remote_impl(methods: &[Procedure]) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();
    for method in methods.iter() {
        let index_ident = method.ident.clone().unwrap();
        let ident = &method.ident;
        let mut arg_stream = proc_macro2::TokenStream::new();
        let mut arg_names_stream = proc_macro2::TokenStream::new();
        if method.mut_receiver {
            arg_stream.extend(quote! {
                &mut self,
            });
        } else {
            arg_stream.extend(quote! {
                &self,
            });
        }
        let mut call_sig = proc_macro2::TokenStream::new();
        for (index, ty) in method.arg_types.iter().enumerate() {
            let ident = Ident::new(&format!("_{}", index), Span::call_site());
            arg_stream.extend(quote! {
                #ident: #ty,
            });
            arg_names_stream.extend(quote! {
                #ident,
            });
        }
        arg_names_stream.extend(quote! {
            _proto_id,
        });
        call_sig.extend(quote! {
            (#arg_names_stream)
        });
        let return_type = &method.return_type;
        stream.extend(quote! {
            fn #ident(#arg_stream) -> #return_type {
                let _proto_id = self.next_id();
                self.queue.write().unwrap().push_back(Call {call: _Call::#index_ident#call_sig});
                self.task.notify();
                let (ct, ct1) = ::vitruvia::protocol::Context::new();
                self.channels.write().unwrap().insert(_proto_id, st_traits::Channel::#ident(Box::new(ct1)));
                <#return_type as ::vitruvia::protocol::Value>::construct(ct)
            }
        });
    }
    stream
}

fn generate_serialize_impl(methods: &[Procedure]) -> proc_macro2::TokenStream {
    let mut arms = proc_macro2::TokenStream::new();
    for (index, method) in methods.iter().enumerate() {
        let ident = &method.ident;
        let mut sig = proc_macro2::TokenStream::new();
        let mut args = proc_macro2::TokenStream::new();
        let mut element_calls = proc_macro2::TokenStream::new();
        let t_len = method.arg_types.len() + 2;
        for index in 0..=method.arg_types.len() {
            let ident = Ident::new(&format!("_{}", index), Span::call_site());
            args.extend(quote! {
                #ident,
            });
            element_calls.extend(quote! {
                seq.serialize_element(#ident)?;
            });
        }
        sig.extend(quote! {
            (#args)
        });
        arms.extend(quote! {
            _Call::#ident#sig => {
                let mut seq = serializer.serialize_seq(Some(#t_len))?;
                seq.serialize_element(&#index)?;
                #element_calls
                seq.end()
            },
        });
    }
    arms
}

fn generate_serialize_return_impl(methods: &[Procedure]) -> proc_macro2::TokenStream {
    let mut arms = proc_macro2::TokenStream::new();
    for method in methods {
        let ident = &method.ident;
        arms.extend(quote! {
            Response::#ident(data, idx, m) => {
                let mut seq = serializer.serialize_seq(Some(3))?;
                seq.serialize_element(m)?;
                seq.serialize_element(idx)?;
                seq.serialize_element(data)?;
                seq.end()
            },
        });
    }
    arms
}

fn generate_deserialize_impl(methods: &[Procedure]) -> proc_macro2::TokenStream {
    let mut arms = proc_macro2::TokenStream::new();
    for (index, method) in methods.iter().enumerate() {
        let ident = &method.ident;
        let mut sig = proc_macro2::TokenStream::new();
        let mut args = proc_macro2::TokenStream::new();
        for index in (0..=method.arg_types.len()).into_iter().map(|i| i + 1) {
            args.extend(quote! {
                seq.next_element()?.ok_or_else(|| ::serde::de::Error::invalid_length(#index, &self))?,
            });
        }
        sig.extend(quote! {
            (#args)
        });
        arms.extend(quote! {
            #index => {
                _Call::#ident#sig
            }
        });
    }
    quote! {
        Ok(Call{
            call: match index {
                #arms,
                _ => {
                    let d: Response = seq.next_element()?.ok_or_else(|| ::serde::de::Error::invalid_length(1, &self))?;
                    _Call::__Response(d)
                }
            }
        })
    }
}

fn generate_deserialize_return_impl(methods: &[Procedure]) -> proc_macro2::TokenStream {
    let mut arms = proc_macro2::TokenStream::new();
    for (index, method) in methods.iter().enumerate() {
        let ident = &method.ident;
        let index = index as u64;
        arms.extend(quote! {
            #index => {
                Ok(Response::#ident(seq.next_element()?.ok_or_else(|| ::serde::de::Error::invalid_length(0, &self))?, seq.next_element()?.ok_or_else(|| ::serde::de::Error::invalid_length(0, &self))?, index))
            }
        });
    }
    quote! {
        match index {
            #arms
            _ => Err(::serde::de::Error::invalid_length(0, &self))?
        }
    }
}

fn generate_shim_forward(methods: &[Procedure]) -> proc_macro2::TokenStream {
    let mut calls = proc_macro2::TokenStream::new();
    for method in methods {
        let ident = &method.ident;
        let mut args = proc_macro2::TokenStream::new();
        let mut arg_names = proc_macro2::TokenStream::new();
        if !method.arg_types.is_empty() {
            for (index, ty) in method.arg_types.iter().enumerate() {
                let ident = Ident::new(&format!("_{}", index), Span::call_site());
                args.extend(quote! {
                    #ident: #ty,
                });
                arg_names.extend(quote! {
                    #ident,
                });
            }
        }
        let receiver = if method.mut_receiver {
            quote! {
                &mut self
            }
        } else {
            quote! {
                &self
            }
        };
        let return_type = &method.return_type;
        calls.extend(quote! {
            fn #ident(#receiver, #args) -> #return_type {
                let ctx = ::vitruvia::protocol::Context::<<#return_type as ::vitruvia::protocol::Value>::Item>::new();
                self.inner.#ident(#arg_names)
            }
        });
    }
    calls
}

fn generate_st_traits(methods: &[Procedure]) -> proc_macro2::TokenStream {
    let mut items = proc_macro2::TokenStream::new();
    let mut variants = proc_macro2::TokenStream::new();

    methods.iter().for_each(|m| {
        let r_type = m.return_type.as_ref().unwrap();
        let ident = m.ident.as_ref().unwrap();
        items.extend(quote! {
            pub trait #ident: ::futures::Stream<Item = <#r_type as ::vitruvia::protocol::Value>::Item, Error = ()> + ::futures::Sink<SinkItem = <#r_type as ::vitruvia::protocol::Value>::Item, SinkError = ()> + Send + Sync {}
            impl<T> #ident for T where T: ::futures::Stream<Item = <#r_type as ::vitruvia::protocol::Value>::Item, Error = ()> + ::futures::Sink<SinkItem = <#r_type as ::vitruvia::protocol::Value>::Item, SinkError = ()> + Send + Sync {}
        });
        variants.extend(quote! {
            #ident(Box<dyn #ident>),
        })
    });

    quote! {
        pub enum Channel {
            #variants
        }
        #items
    }
}

fn generate_handle_response(methods: &[Procedure]) -> proc_macro2::TokenStream {
    let mut arms = proc_macro2::TokenStream::new();
    for method in methods {
        let ident = method.ident.as_ref().unwrap();
        arms.extend(quote! {
            Response::#ident(data, index, id) => {
                let mut channels = self.channels.write().unwrap();
                if let Some(st_traits::Channel::#ident(channel)) = channels.get_mut(&id) {
                    channel.start_send(data);
                }
            }
        });
    }
    quote! {
        match item {
            #arms
        }
    }
}

fn generate_binds(ident: &Ident, methods: &[Procedure]) -> TokenStream {
    let mod_ident = Ident::new(&format!("_{}_protocol", &ident), ident.span());
    let enum_variants = generate_enum(methods);
    let return_variants = generate_return_variants(methods);
    let remote_impl = generate_remote_impl(methods);
    let serialize_impl = generate_serialize_impl(methods);
    let serialize_return_impl = generate_serialize_return_impl(methods);
    let deserialize_impl = generate_deserialize_impl(methods);
    let deserialize_return_impl = generate_deserialize_return_impl(methods);
    let blanket = generate_blanket(methods);
    let st_traits = generate_st_traits(methods);
    let handle_response = generate_handle_response(methods);
    let shim_forward = generate_shim_forward(methods);
    let call_repr: proc_macro2::TokenStream;
    let m_len = methods.len();
    if methods.len() == 1 && methods[0].arg_types.len() == 0 {
        call_repr = proc_macro2::TokenStream::new();
    } else {
        call_repr = quote! {
            #[repr(transparent)]
        };
    }
    let gen = quote! {
        #[allow(non_snake_case)]
        mod #mod_ident {
            use ::std::{collections::VecDeque, sync::{RwLock, Arc, atomic::{AtomicU64, Ordering}}};
            use ::futures::{Poll, Async, task::AtomicTask};
            use ::serde::ser::SerializeSeq;
            #[derive(Clone)]
            struct CRemote {
                task: Arc<AtomicTask>,
                queue: Arc<RwLock<VecDeque<Call>>>,
                ids: Arc<RwLock<Vec<u64>>>,
                last_id: Arc<AtomicU64>,
                channels: Arc<RwLock<::std::collections::HashMap<u64, st_traits::Channel>>>,
            }
            impl CRemote {
                pub fn new() -> CRemote {
                    CRemote {
                        task: Arc::new(AtomicTask::new()),
                        queue: Arc::new(RwLock::new(VecDeque::new())),
                        ids: Arc::new(RwLock::new(vec![])),
                        last_id: Arc::new(AtomicU64::new(0)),
                        channels: Arc::new(RwLock::new(::std::collections::HashMap::new())),
                    }
                }
                fn next_id(&self) -> u64 {
                    let mut ids = self.ids.write().unwrap();
                    if let Some(id) = ids.pop() {
                        id
                    } else {
                        self.last_id.fetch_add(1, Ordering::SeqCst)
                    }
                }
            }
            impl super::#ident for CRemote {
                #remote_impl
            }
            impl ::futures::Stream for CRemote {
                type Item = Call;
                type Error = ();

                fn poll(&mut self) -> Poll<::std::option::Option<Self::Item>, Self::Error> {
                    match self.queue.write().unwrap().pop_front() {
                        Some(item) => {
                            Ok(Async::Ready(Some(item)))
                        },
                        None => {
                            self.task.register();
                            Ok(Async::NotReady)
                        }
                    }
                }
            }
            impl ::futures::Sink for CRemote {
                type SinkItem = Response;
                type SinkError = ();

                fn start_send(&mut self, item: Self::SinkItem) -> ::futures::StartSend<Self::SinkItem, Self::SinkError> {
                    #handle_response
                    Ok(::futures::AsyncSink::Ready)
                }
                fn poll_complete(&mut self) -> ::futures::Poll<(), Self::SinkError> {
                    Ok(::futures::Async::Ready(()))
                }
            }
            #call_repr
            pub struct Call {
                call: _Call,
            }
            #[allow(non_camel_case_types)]
            enum _Call {
                #(#enum_variants),*,
                __Response(Response)
            }
            #[allow(non_camel_case_types)]
            mod st_traits {
                #st_traits
            }
            #[allow(non_camel_case_types)]
            pub enum Response {
                #(#return_variants),*
            }
            impl ::serde::Serialize for Call {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
                    match &self.call {
                        #serialize_impl
                        _Call::__Response(response) => {
                            let mut seq = serializer.serialize_seq(Some(4))?;
                            seq.serialize_element(&#m_len)?;
                            seq.serialize_element(response);
                            seq.end()
                        }
                    }
                }
            }
            impl ::serde::Serialize for Response {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
                    match self {
                        #serialize_return_impl
                    }
                }
            }
            impl <'de> ::serde::Deserialize<'de> for Call {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                    struct CallVisitor;
                    impl<'de> ::serde::de::Visitor<'de> for CallVisitor {
                        type Value = Call;

                        fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                            formatter.write_str("a serialized protocol Call")
                        }
                        fn visit_seq<V>(self, mut seq: V) -> Result<Call, V::Error> where V: ::serde::de::SeqAccess<'de>, {
                            let index: usize = seq.next_element()?.ok_or_else(|| ::serde::de::Error::invalid_length(0, &self))?;
                            #deserialize_impl
                        }
                    }
                    deserializer.deserialize_seq(CallVisitor)
                }
            }
            pub trait Remote: futures::Stream<Item = Call, Error = ()> + futures::Sink<SinkItem = Response, SinkError = ()> + Clone {}
            impl Remote for CRemote {}
            impl <'de> ::serde::Deserialize<'de> for Response {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                    struct ResponseVisitor;
                    impl<'de> ::serde::de::Visitor<'de> for ResponseVisitor {
                        type Value = Response;

                        fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                            formatter.write_str("a serialized protocol Response")
                        }
                        fn visit_seq<V>(self, mut seq: V) -> Result<Response, V::Error> where V: ::serde::de::SeqAccess<'de>, {
                            let index: u64 = seq.next_element()?.ok_or_else(|| ::serde::de::Error::invalid_length(0, &self))?;
                            #deserialize_return_impl
                        }
                    }
                    deserializer.deserialize_seq(ResponseVisitor)
                }
            }
            #[allow(non_camel_case_types)]
            pub struct ProtocolShim<T: super::#ident> {
                inner: T,
                channels: ::std::collections::HashMap<u64, st_traits::Channel>,
                inner_stream: Box<dyn ::futures::Stream<Item = Response, Error = ()> + Send>,
            }
            impl<T: super::#ident> ProtocolShim<T> {
                pub fn new(inner: T) -> Self {
                    ProtocolShim {
                        inner,
                        channels: ::std::collections::HashMap::new(),
                        inner_stream: Box::new(::futures::stream::empty())
                    }
                }
            }
            impl<T> ::futures::Sink for ProtocolShim<T> where T: super::#ident {
                type SinkItem = Call;
                type SinkError = ();
                fn start_send(&mut self, item: Self::SinkItem) -> ::futures::StartSend<Self::SinkItem, Self::SinkError> {
                    use super::#ident;
                    use ::vitruvia::protocol::Value;
                    use ::futures::{Stream, Sink, Future};
                    match item.call {
                        #blanket
                        _Call::__Response(resp) => {
                            // TODO
                        }
                    }
                    Ok(::futures::AsyncSink::Ready)
                }
                fn poll_complete(&mut self) -> ::futures::Poll<(), Self::SinkError> {
                    Ok(::futures::Async::Ready(()))
                }
            }
            impl<T> ::futures::Stream for ProtocolShim<T> where T: super::#ident {
                type Item = Response;
                type Error = ();

                fn poll(&mut self) -> ::futures::Poll<Option<Self::Item>, Self::Error> {
                    self.inner_stream.poll()
                }
            }
            pub trait Protocol: ::futures::Sink<SinkItem = Call, SinkError = ()> + ::futures::Stream<Item = Response, Error = ()> + super::#ident + Send {}
            #[allow(non_camel_case_types)]
            impl<T> Protocol for ProtocolShim<T> where T: super::#ident + Send {}
            impl<T: super::#ident> super::#ident for ProtocolShim<T> {
                #shim_forward
            }
            pub fn remote() -> impl super::#ident + Remote {
                CRemote::new()
            }
        }
    };
    gen.into()
}

fn generate_blanket(methods: &[Procedure]) -> proc_macro2::TokenStream {
    let mut arms = proc_macro2::TokenStream::new();
    for (index, method) in methods.iter().enumerate() {
        let index = index as u64;
        let ident = &method.ident;
        let mut sig = proc_macro2::TokenStream::new();
        let mut args = proc_macro2::TokenStream::new();
        for index in 0..method.arg_types.len() {
            let ident = Ident::new(&format!("_{}", index), Span::call_site());
            args.extend(quote! {
                #ident,
            });
        }
        let mut s_args = args.clone();
        let id = Ident::new(&format!("_{}", method.arg_types.len()), Span::call_site());
        s_args.extend(quote! {
            #id,
        });
        sig.extend(quote! {
            (#s_args)
        });
        arms.extend(quote! {
            _Call::#ident#sig => {
                let (context, loc_context) = ::vitruvia::protocol::Context::new();
                self.#ident(#args).deconstruct(context);
                let (sink, stream) = loc_context.split();
                let mut i_stream: Box<dyn ::futures::Stream<Error = (), Item = Response> + Send + 'static> = Box::new(futures::stream::empty());
                std::mem::swap(&mut self.inner_stream, &mut i_stream);
                self.inner_stream = Box::new(stream.map(move |i| Response::#ident(i, #index, #id)).select(i_stream));
            }
        });
    }
    arms
}

#[proc_macro_attribute]
pub fn protocol(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return r#"compile_error!("unexpected arguments passed to `protocol`");"#
            .parse()
            .unwrap();
    }
    let mut input = {
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
    let mut procedures = vec![];
    for (index, item) in input.items.iter_mut().enumerate() {
        let mut procedure = Procedure {
            arg_types: vec![],
            return_type: None,
            ident: None,
            mut_receiver: false,
        };
        if let TraitItem::Method(method) = item {
            if &format!("{}", method.sig.ident) == "remote" {
                return TokenStream::from(quote_spanned! {
                    method.sig.ident.span() =>
                    compile_error!("`protocol` methods must not be named remote");
                });
            }
            if &format!("{}", method.sig.ident) == "into_protocol" {
                return TokenStream::from(quote_spanned! {
                    method.sig.ident.span() =>
                    compile_error!("`protocol` methods must not be named into_protocol");
                });
            }
            if let Some(default) = &method.default {
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
            if let Some(where_clause) = &method.sig.decl.generics.where_clause {
                return TokenStream::from(quote_spanned! {
                    where_clause.span() =>
                    compile_error!("where clause not allowed on `protocol` method");
                });
            }
            if let ReturnType::Type(_, ty) = &mut method.sig.decl.output {
                let ident = Ident::new(
                    &format!("_{}_{}_rt_AssertValue", &input.ident, index),
                    Span::call_site(),
                );
                assert_stream.extend(TokenStream::from(quote_spanned! {
                    ty.span() =>
                    struct #ident where #ty: ::vitruvia::protocol::Value;
                }));
                procedure.return_type = Some(*ty.clone());
            } else {
                let m: proc_macro::TokenStream = quote! {
                    ()
                }
                .into();
                let ty = parse_macro_input!(m as Type);
                procedure.return_type = Some(ty);
            }
            let mut has_receiver = false;
            for (arg_index, argument) in method.sig.decl.inputs.iter().enumerate() {
                match argument {
                    FnArg::SelfValue(_) => {
                        return TokenStream::from(quote_spanned! {
                            argument.span() =>
                            compile_error!("cannot consume self in `protocol` method");
                        });
                    }
                    FnArg::SelfRef(self_ref) => {
                        if self_ref.mutability.is_some() {
                            procedure.mut_receiver = true;
                        }
                        has_receiver = true;
                    }
                    FnArg::Captured(argument) => {
                        let ty = &argument.ty;
                        let ident = Ident::new(
                            &format!("_{}_{}_arg_{}_AssertValue", &input.ident, index, arg_index),
                            Span::call_site(),
                        );
                        assert_stream.extend(TokenStream::from(quote_spanned! {
                            ty.span() =>
                            struct #ident where #ty: ::vitruvia::protocol::Value;
                        }));
                        procedure.arg_types.push(argument.ty.clone());
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
            procedure.ident = Some(method.sig.ident.clone());
        } else {
            return TokenStream::from(quote_spanned! {
                item.span() =>
                compile_error!("`protocol` expected method");
            });
        }
        procedures.push(procedure);
    }
    if procedures.len() == 0 {
        return TokenStream::from(quote_spanned! {
            input.span() =>
            compile_error!("`protocol` with no methods is invalid");
        });
    }
    let ident = &input.ident;
    let mod_ident = Ident::new(&format!("_{}_protocol", ident), input.ident.span());
    let mut m: TokenStream = quote! {
        #[doc(hidden)]
        fn into_protocol(self) -> Box<dyn #mod_ident::Protocol> where Self: Sized + 'static {
            Box::new(#mod_ident::ProtocolShim::new(self))
        }
    }
    .into();
    input
        .items
        .push(TraitItem::Method(parse_macro_input!(m as TraitItemMethod)));
    m = quote! {
        #[doc(hidden)]
        fn IS_PROTO() where Self: Sized {}
    }
    .into();
    input
        .items
        .push(TraitItem::Method(parse_macro_input!(m as TraitItemMethod)));
    let mut ty_path = Punctuated::new();
    ty_path.push_value(PathSegment {
        arguments: PathArguments::None,
        ident: Ident::new("Send", input.ident.span()),
    });
    input
        .supertraits
        .push_value(TypeParamBound::Trait(TraitBound {
            paren_token: None,
            modifier: TraitBoundModifier::None,
            lifetimes: None,
            path: Path {
                leading_colon: None,
                segments: ty_path,
            },
        }));
    let binds = generate_binds(ident, &procedures);
    let blanket_impl: TokenStream = quote! {
        impl dyn #ident {
            fn remote() -> impl #ident + #mod_ident::Remote {
                #mod_ident::remote()
            }
        }
    }
    .into();
    let mut item: TokenStream = input.into_token_stream().into();
    item.extend(blanket_impl);
    item.extend(assert_stream);
    item.extend(binds);
    item
}
