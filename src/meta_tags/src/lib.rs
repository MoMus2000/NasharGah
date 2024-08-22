use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn api_callback(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    // Get the function name and arguments
    let name = &input_fn.sig.ident;
    let body = &input_fn.block;

    // Generate the transformed function

    let transformed_fn = quote! {
        pub fn #name<'a>(_request: Request, mut writer: ResponseWriter<'a>) -> Result<std::pin::Pin<std::boxed::Box<dyn std::future::Future<Output = String> + Send>> , Box<dyn std::error::Error>> {
            // Original function body
            let result = (|| {
                // Execute the original function body
                #body
            })()?;

            Ok(result)
        }
    };

    transformed_fn.into()
}

#[proc_macro_attribute]
pub fn init(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let body = &input_fn.block;
    let transformed_fn = quote! {
        #[tokio::main]
        pub async fn main() {
            let mut result = (|| {
                #body
            })();
            result.serve().await;
        }
    };

    transformed_fn.into()
}
