use proc_macro2::TokenStream;
use quote::quote;

pub fn build(block: TokenStream) -> TokenStream {
    quote! {
        #[cfg(target_arch = "wasm32")]
        extern "C" {
            fn _EXPORT_output(ptr: *const u8, len: usize);
            fn _EXPORT_panic(ptr: *const u8, len: usize);
        }
        #[cfg(target_arch = "wasm32")]
        fn _EXPORT_safe_output<T: AsRef<[u8]>>(data: T) {
            let data = data.as_ref();
            unsafe { _EXPORT_output(data.as_ptr(), data.len()) };
        }
        #[cfg(target_arch = "wasm32")]
        #[no_mangle]
        pub extern "C" fn _EXPORT_make_buffer(len: usize) -> *mut u8 {
            use ::std::{mem::{size_of, forget, align_of}, ptr::write, alloc::{alloc, Layout}};
            let len_size = size_of::<usize>();
            unsafe {
                let ptr = alloc(Layout::from_size_align(len + len_size, align_of::<u8>()).unwrap());
                write(ptr as *mut usize, len);
                ptr.add(len_size)
            }
        }
        ::vessels::lazy_static::lazy_static! {
            static ref DATA: ::vessels::futures::lock::Mutex<(::vessels::futures::channel::mpsc::UnboundedSender<Vec<u8>>, Option<::vessels::futures::channel::mpsc::UnboundedReceiver<Vec<u8>>>)> = { let (sender, receiver) = ::vessels::futures::channel::mpsc::unbounded(); ::vessels::futures::lock::Mutex::new((sender, Some(receiver))) };
        }
        #[cfg(target_arch = "wasm32")]
        #[no_mangle]
        pub extern "C" fn _EXPORT_input(data: *mut u8) {
            use ::std::{mem::size_of, slice};
            unsafe {
                let len = Box::from_raw(data.sub(size_of::<usize>()) as *mut usize);
                let data = slice::from_raw_parts_mut(data, *len).to_vec();
                use ::vessels::futures::SinkExt;
                ::vessels::core::spawn(async move {
                    DATA.lock().await.0.clone().send(data).await.unwrap();
                });
            }
        }
        #[cfg(target_arch = "wasm32")]
        #[no_mangle]
        pub extern "C" fn _EXPORT_initialize() {
            std::panic::set_hook(Box::new(|info| {
                use ::std::ops::Deref;
                let cause = info.payload().downcast_ref::<String>().map(String::deref);
                let cause = cause.unwrap_or_else(||
                    info.payload().downcast_ref::<&str>().map(|s| *s)
                        .unwrap_or("<cause unknown>")
                );
                let data = format!("panic {}: {}", info, cause);
                unsafe { _EXPORT_panic(data.as_bytes().as_ptr(), data.len()) };
            }));
            let _export_initializer: fn(::vessels::core::Handle) -> ::vessels::kind::Future<_> = |handle| {
                ::vessels::core::register_handle(handle);
                Box::pin(async {
                    #block
                })
            };
            let _test_shims: () = {
                trait export_helper {
                    type Kind: ::vessels::Kind;
                }
                struct export<T: ::vessels::Kind>(fn(::vessels::core::Handle) -> ::vessels::kind::Future<T>);
                impl<T: ::vessels::Kind> export_helper for export<T> {
                    type Kind = T;
                }
                export(_export_initializer);
            };
            use ::vessels::{channel::IdChannel, OnTo, futures::{StreamExt, SinkExt, TryFutureExt}, format::{ApplyEncode, Cbor}, core};
            let vessel = Box::new(_export_initializer) as ::vessels::core::Constructor<_>;
            ::vessels::core::spawn(async move {
                let (sink, mut stream) = vessel.on_to::<IdChannel>().await.encode::<Cbor>().split();
                let receiver = DATA.lock().await.1.take().unwrap();
                ::vessels::core::spawn(receiver.map(Ok).forward(sink).unwrap_or_else(|_| panic!()));
                while let Some(item) = stream.next().await {
                    _EXPORT_safe_output(item);
                }
            });
        }

        fn main() {}

        const EXPORT_ITEMS_: () = {
            #[cfg(not(target_arch = "wasm32"))]
            compile_error!("vessel must be compiled to wasm");
            #[cfg(feature = "core")]
            compile_error!("vessel cannot be compiled against core");
        };
    }
}
