// src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn callback(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    // Get the function name and arguments
    let name = &input_fn.sig.ident;
    let body = &input_fn.block;

    // Generate the transformed function

    let transformed_fn = quote! {
        pub fn #name<'a>(_request: Request, mut writer: ResponseWriter<'a>) -> std::pin::Pin<std::boxed::Box<dyn std::future::Future<Output = String> + Send>> {
            // Transformed function
            println!("Function {} is called", stringify!(#name));
            // Original function body
            let result = (|| {
                // Execute the original function body
                #body
            })();

            result
        }
    };

    transformed_fn.into()
}