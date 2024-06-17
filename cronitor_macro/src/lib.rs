extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

#[proc_macro_attribute]
pub fn cronitor(attr: TokenStream, item: TokenStream) -> TokenStream {
    let cron_expression = parse_macro_input!(attr as LitStr);
    let cron_expression_str = cron_expression.value();

    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_name_str = fn_name.to_string();

    let expanded = quote! {
        fn #fn_name() {
            #fn_block

            cronitor_runtime::CRON_REGISTRY.lock().unwrap().register(#fn_name_str.to_string(), #cron_expression_str.to_string(), #fn_name);
        }
    };

    TokenStream::from(expanded)
}
