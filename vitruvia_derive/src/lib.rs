#![recursion_limit = "256"]

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::{quote, quote_spanned, ToTokens};

use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Paren, Field, Fields, FieldsUnnamed, FnArg,
    Ident, ItemTrait, ReturnType, TraitItem, Type, Variant, Visibility,
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
            fields: if method.arg_types.is_empty() {
                Fields::Unit
            } else {
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
        for (index, ty) in method.arg_types.iter().enumerate() {
            let ident = Ident::new(&format!("_{}", index), Span::call_site());
            arg_stream.extend(quote! {
                #ident: #ty,
            });
            arg_names_stream.extend(quote! {
                #ident,
            });
        }
        stream.extend(quote! {
            fn #ident(#arg_stream) {
                self.queue.write().unwrap().push_back(Call {call: _Call::#index_ident(#arg_names_stream)});
                self.task.notify();
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
        let t_len = method.arg_types.len() + 1;
        if !method.arg_types.is_empty() {
            for (index, _) in method.arg_types.iter().enumerate() {
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
        }
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

fn generate_deserialize_impl(methods: &[Procedure]) -> proc_macro2::TokenStream {
    let mut arms = proc_macro2::TokenStream::new();
    for (index, method) in methods.iter().enumerate() {
        let ident = &method.ident;
        let mut sig = proc_macro2::TokenStream::new();
        let mut args = proc_macro2::TokenStream::new();
        if !method.arg_types.is_empty() {
            for (index, _) in method.arg_types.iter().enumerate() {
                let index = index + 1;
                args.extend(quote! {
                    seq.next_element()?.ok_or_else(|| ::serde::de::Error::invalid_length(#index, &self))?,
                });
            }
            sig.extend(quote! {
                (#args)
            });
        }
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
                _ => Err(::serde::de::Error::invalid_length(index, &self))?
            }
        })
    }
}

fn generate_binds(ident: &Ident, methods: &[Procedure]) -> TokenStream {
    let mod_ident = Ident::new(&format!("_{}_protocol", &ident), ident.span());
    let enum_variants = generate_enum(methods);
    let remote_impl = generate_remote_impl(methods);
    let serialize_impl = generate_serialize_impl(methods);
    let deserialize_impl = generate_deserialize_impl(methods);
    let blanket = generate_blanket(methods);
    let gen = quote! {
        #[allow(non_snake_case)]
        mod #mod_ident {
            use ::std::{collections::VecDeque, sync::{RwLock}};
            use ::futures::{Poll, Async, task::AtomicTask};
            use ::serde::ser::SerializeSeq;
            struct Remote {
                task: AtomicTask,
                queue: RwLock<VecDeque<Call>>
            }
            impl Remote {
                pub fn new() -> Remote {
                    Remote {
                        task: AtomicTask::new(),
                        queue: RwLock::new(VecDeque::new())
                    }
                }
            }
            impl super::#ident for Remote {
                #remote_impl
            }
            impl ::vitruvia::protocol::Remote for Remote {
                type Item = Call;
            }
            impl ::futures::Stream for Remote {
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
            #[repr(transparent)]
            pub struct Call {
                call: _Call,
            }
            #[allow(non_camel_case_types)]
            enum _Call {
                #(#enum_variants),*
            }
            impl ::serde::Serialize for Call {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
                    match &self.call {
                        #serialize_impl
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
            pub fn start_send<T>(receiver: &mut T, call: Call) -> ::futures::StartSend<Call, ()> where T: super::#ident + ?Sized {
                match call.call {
                    #blanket
                }
                Ok(::futures::AsyncSink::Ready)
            }
            pub fn remote() -> impl super::#ident + ::vitruvia::protocol::Remote {
                Remote::new()
            }
        }
    };
    gen.into()
}

fn generate_blanket(methods: &[Procedure]) -> proc_macro2::TokenStream {
    let mut arms = proc_macro2::TokenStream::new();
    for (index, method) in methods.iter().enumerate() {
        let ident = &method.ident;
        let mut sig = proc_macro2::TokenStream::new();
        let mut args = proc_macro2::TokenStream::new();
        if !method.arg_types.is_empty() {
            for (index, _) in method.arg_types.iter().enumerate() {
                let ident = Ident::new(&format!("_{}", index), Span::call_site());
                args.extend(quote! {
                    #ident,
                });
            }
            sig.extend(quote! {
                (#args)
            });
        }
        arms.extend(quote! {
            _Call::#ident#sig => {
                receiver.#ident#sig;
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
    let input = {
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
    for (index, item) in input.items.iter().enumerate() {
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
            // TODO: Disallow return type until I figure out how to handle async in the macro
            if let ReturnType::Type(_, _) = &method.sig.decl.output {
                return TokenStream::from(quote_spanned! {
                    method.sig.decl.output.span() =>
                    compile_error!("return type not allowed on `protocol` method");
                });
            }
            /*if let ReturnType::Type(_, ty) = &method.sig.decl.output {
                let ident = Ident::new(
                    &format!("_{}_{}_rt_AssertSerializeDeserialize", &input.ident, index),
                    Span::call_site(),
                );
                assert_stream.extend(TokenStream::from(quote_spanned! {
                    ty.span() =>
                    struct #ident where #ty: ::serde::Serialize + ::serde::de::DeserializeOwned;
                }));
                procedure.return_type = Some(*ty.clone());
            }*/
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
                            &format!(
                                "_{}_{}_arg_{}_AssertSerializeDeserialize",
                                &input.ident, index, arg_index
                            ),
                            Span::call_site(),
                        );
                        assert_stream.extend(TokenStream::from(quote_spanned! {
                            ty.span() =>
                            struct #ident where #ty: ::serde::Serialize + ::serde::de::DeserializeOwned;
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
    let ident = &input.ident;
    let mod_ident = Ident::new(&format!("_{}_protocol", ident), input.ident.span());
    let binds = generate_binds(ident, &procedures);
    let proto_ident = Ident::new(&format!("_{}_Protocol", ident), input.ident.span());
    let struct_ident = Ident::new(&format!("_{}_Protocol_Shim", ident), input.ident.span());
    let blanket_impl: TokenStream = quote! {
        impl #ident {
            fn remote() -> impl #ident + ::vitruvia::protocol::Remote {
                #mod_ident::remote()
            }
        }
        #[allow(non_camel_case_types)]
        struct #struct_ident<T: #ident> {
            c_ref: T
        }
        impl<T> ::futures::Sink for #struct_ident<T> where T: #ident {
            type SinkItem = #mod_ident::Call;
            type SinkError = ();
            fn start_send(&mut self, item: Self::SinkItem) -> ::futures::StartSend<Self::SinkItem, Self::SinkError> {
                #mod_ident::start_send(&mut self.c_ref, item)
            }
            fn poll_complete(&mut self) -> ::futures::Poll<(), Self::SinkError> {
                Ok(::futures::Async::Ready(()))
            }
        }
        #[allow(non_camel_case_types)]
        trait #proto_ident<T: #ident> {
            fn into_protocol(self) -> Box<dyn Sink<SinkItem = #mod_ident::Call, SinkError = ()> + Send>;
        }
        impl<T> #proto_ident<T> for T where T: #ident + 'static + Send {
            fn into_protocol(self) -> Box<dyn Sink<SinkItem = #mod_ident::Call, SinkError = ()> + Send> {
                Box::new(#struct_ident {
                    c_ref: self
                })
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
