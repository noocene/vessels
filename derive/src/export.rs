use proc_macro2::TokenStream;
use quote::quote;

pub fn build(_: TokenStream) -> TokenStream {
    quote! {
        fn main() {}

        const EXPORT_ITEMS_: () = {
            #[cfg(not(target_arch = "wasm32"))]
            compile_error!("vessel must be compiled to wasm");
        };
    }
}
