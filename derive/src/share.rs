use proc_macro2::TokenStream;
use quote::quote;
use synstructure::Structure;

pub fn derive(s: Structure) -> TokenStream {
    let body = s.each_variant(|variant| {
        let bindings = variant.bindings();
        variant.construct(|_, idx| {
            let binding = &bindings[idx].binding;
            quote! {
                #binding.share()
            }
        })
    });

    s.gen_impl(quote! {
        gen impl ::vessels::replicate::Share for @Self {
            fn share(&self) -> Self {
                match *self { #body }
            }
        }
    })
}
