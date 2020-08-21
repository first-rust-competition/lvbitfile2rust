extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn lvbitfile2rust(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::LitStr);
    match lvbitfile2rust::codegen(&input.value()) {
        Ok(tokens) => tokens,
        Err(e) => {
            let err_string = e.to_string();
            quote!{
                compile_error!(
                    #err_string
                );
            }
        },
    }.into()
}
