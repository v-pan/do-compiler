use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Expr};

#[proc_macro_derive(TokenTypeDef, attributes(info))]
pub fn token_derive(_tokens: TokenStream) -> TokenStream {
    let tokens = _tokens.clone();
    let input = parse_macro_input!(tokens as DeriveInput);

    for attr in input.attrs {
        let arg: Expr = attr.parse_args().unwrap();

        panic!("{arg:?}");
    }

    quote! { 
        
    }.into()
}