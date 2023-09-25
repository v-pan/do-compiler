use std::{fs::File, io::Write};

use proc_macro::TokenStream;
use proc_macro2::{TokenTree::Ident, TokenTree::Literal};
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Data, Meta, Expr, Lit};

#[proc_macro_derive(TokenTypeDef, attributes(word, pair, operator))]
pub fn token_derive(_tokens: TokenStream) -> TokenStream {
    let tokens = _tokens.clone();
    let input = parse_macro_input!(tokens as DeriveInput);
    let mut log = File::create("token_macro.log").unwrap();

    log.write(format!("{:#?}\n", input).as_bytes()).unwrap();

    let mut idents: Vec<proc_macro2::TokenStream> = vec![];
    let mut words: Vec<proc_macro2::TokenStream> = vec![];
    let mut pairs: Vec<proc_macro2::TokenStream> = vec![];

    let mut operator_idents: Vec<proc_macro2::TokenStream> = vec![];
    let mut operator_precedences: Vec<proc_macro2::TokenStream> = vec![];

    let ident = input.ident;

    match input.data {
        Data::Enum(token_enum) => {
            for variant in token_enum.variants {
                let var_ident = variant.ident;
                idents.push(quote! { #var_ident });

                for attr in variant.attrs {
                    let path = attr.meta.path();
                    if let Some(ident) = path.get_ident() {
                        match ident.to_string().as_str() {
                            "word" => {
                                if let Meta::NameValue(name_value) = attr.meta {
                                    if let Expr::Lit(value) = name_value.value {
                                        if let Lit::Str(string) = value.lit {
                                            words.push(quote! { #string });
                                        }
                                    }
                                }
                            },
                            "pair" => {
                                if let Meta::List(list) = attr.meta {
                                    for token in list.tokens {
                                        if let Ident(identifier) = token {
                                            pairs.push(quote! { #identifier });
                                        }
                                    }
                                }
                            },
                            "operator" => {
                                if let Meta::List(list) = attr.meta {
                                    let mut last_identifier = String::new();
                                    for token in list.tokens {
                                        match token {
                                            Ident(identifier) => {
                                                last_identifier = identifier.to_string();
                                                operator_idents.push(quote! { #var_ident });
                                            }
                                            Literal(lit) => {
                                                match last_identifier.as_str() {
                                                    "precedence" => {
                                                        operator_precedences.push(quote! { #lit });
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
        _ => {}
    }

    // log.write(format!("{:#?}\n", idents).as_bytes()).unwrap();
    // log.write(format!("{:#?}\n", words).as_bytes()).unwrap();
    // log.write(format!("{:#?}\n", pairs).as_bytes()).unwrap();

    let match_arms = quote! { #(#words => Self::#idents),* };
    // log.write(format!("{:#?}\n", match_arms.to_string()).as_bytes()).unwrap();

    let from_str = quote! {
        impl core::convert::From<&str> for #ident {
            fn from(value: &str) -> Self {
                match value {
                    #match_arms,
                    _ => Self::Unknown,
                }
            }
        }
    };

    let match_arms = quote! { #(Self::#pairs => Some(Self::#idents)),* };
    let get_pair = quote! {
        fn get_pair(&self) -> Option<Self> {
            match self {
                #match_arms,
                _ => None
            }
        }
    };

    // log.write(format!("{:#?}\n", operator_idents).as_bytes()).unwrap();
    // log.write(format!("{:#?}\n", operator_precedences).as_bytes()).unwrap();

    let match_arms = quote! { #(Self::#operator_idents => Some(#operator_precedences)),* };
    let get_precedence = quote! {
        // Returns None if not an operator type
        fn get_precedence(&self) -> Option<u8> {
            match self {
                #match_arms,
                _ => None
            }
        }
    };

    let output = quote! {
        #from_str

        impl #ident {
            pub #get_pair
            pub #get_precedence
        }
    };

    output.into()
}