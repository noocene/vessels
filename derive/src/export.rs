use proc_macro2::TokenStream;
use quote::quote;

pub fn build(block: TokenStream) -> TokenStream {
    quote! {
        #[cfg(target_arch = "wasm32")]
        extern "C" {
            fn _EXPORT_output(ptr: *const u8, len: usize);
        }
        #[cfg(target_arch = "wasm32")]
        fn _EXPORT_safe_output<T: AsRef<[u8]>>(data: T) {
            let data = data.as_ref();
            unsafe { _EXPORT_output(data.as_ptr(), data.len()) };
        }
        #[cfg(target_arch = "wasm32")]
        #[no_mangle]
        pub extern "C" fn _EXPORT_make_buffer(len: usize) -> *mut u8 {
            use std::{mem::{size_of, forget}, ptr::write};
            let len_size = size_of::<usize>();
            let mut buf = vec![0u8; len + len_size].into_boxed_slice();
            let ptr = unsafe { buf.as_mut_ptr() };
            forget(buf);
            unsafe {
                write(ptr as *mut usize, len);
                ptr.add(len_size)
            }
        }
        #[cfg(target_arch = "wasm32")]
        #[no_mangle]
        pub extern "C" fn _EXPORT_input(data: *mut u8) {
            use ::std::{mem::size_of, slice};
            unsafe {
                let len = Box::from_raw(data.sub(size_of::<usize>()) as *mut usize);
                let data = slice::from_raw_parts_mut(data, *len);
                let data: Box<[u8]> = Box::from_raw(data);
                let data = data.into_vec();
                _EXPORT_safe_output(data);
            }
        }
        #[cfg(target_arch = "wasm32")]
        #[no_mangle]
        pub extern "C" fn initialize() {
            let _export_initializer: fn() -> _ = || {
                #block
            };
            let _test_shims: () = {
                trait export_helper {
                    type Kind: ::vessels::Kind;
                }
                struct export<T: ::vessels::Kind>(fn() -> T);
                impl<T: ::vessels::Kind> export_helper for export<T> {
                    type Kind = T;
                }
                export(_export_initializer);
            };
            async move {
                use ::vessels::{channel::IdChannel, OnTo, futures::{StreamExt, SinkExt}, format::{ApplyEncode, Cbor}};
                let (mut sink, mut stream) = _export_initializer().on_to::<IdChannel>().await.encode::<Cbor>().split();
                while let Some(item) = stream.next().await {
                    _EXPORT_safe_output(item);
                }
            };
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
