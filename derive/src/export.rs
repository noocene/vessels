use proc_macro2::TokenStream;
use quote::quote;

pub fn build(block: TokenStream) -> TokenStream {
    quote! {
        fn main() {
            let _export_initializer: fn() -> _ = || {
                #block
            };
            let _test_shims: () = {
                pub struct export<T: ::vessels::Kind>(fn() -> T);
                export(_export_initializer);
            };
        }

        const EXPORT_ITEMS_: () = {
            #[cfg(not(target_arch = "wasm32"))]
            compile_error!("vessel must be compiled to wasm");
            #[cfg(feature = "core")]
            compile_error!("vessel cannot be compiled against core");
        };
    }
}
