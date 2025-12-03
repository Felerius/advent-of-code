use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn register(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();
    let static_name = format_ident!("STATIC_{fn_name_str}");
    let krate = quote! { ::register };
    let macro_supp = quote! { #krate::__macro_support };

    quote! {
        #input_fn

        #[#macro_supp::distributed_slice(#macro_supp::REGISTERED_FUNCTIONS)]
        #[linkme(crate=#macro_supp::linkme)]
        static #static_name: #krate::RegisteredFunction = #krate::RegisteredFunction {
            module_path: module_path!(),
            name: #fn_name_str,
            file: file!(),
            func: |input: &str| #macro_supp::NormalizeOutput::normalize(#fn_name(input)),
        };
    }
    .into()
}
