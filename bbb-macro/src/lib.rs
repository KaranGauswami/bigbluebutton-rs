extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ApiName)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let tokens = quote!(
        impl GetApiName for #name{
            fn get_api_name(&self) -> &str {
                &self.api_name
            }
        }
    );
    tokens.into()
}
