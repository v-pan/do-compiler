// use std::{fs::File, io::Write};

use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Expr, Lit, Meta};

#[proc_macro_error]
#[proc_macro_derive(Token, attributes(word, operator, initial, terminal))]
pub fn token_derive(_tokens: TokenStream) -> TokenStream {
    let tokens = _tokens.clone();
    let input = parse_macro_input!(tokens as DeriveInput);

    let lifetimes: Vec<_> = input
        .generics
        .lifetimes()
        .map(|lifetime_param| lifetime_param.into_token_stream())
        .collect();

    let lifetime = lifetimes.first();
    if lifetime.is_none() {
        abort!(
            input.generics.span(),
            "Expected at least one lifetime for string slices"
        );
    }

    let mut variant_idents = vec![];
    let mut words = vec![];
    let mut operators = vec![];
    let mut initials = vec![];
    let mut terminals = vec![];

    let output = if let Data::Enum(enum_data) = input.data {
        let enum_ident = input.ident;

        for variant in enum_data.variants {
            variant_idents.push(variant.ident.clone());

            for attr in variant.attrs {
                match attr.meta {
                    Meta::NameValue(meta) => {
                        if let Some(ident) = meta.path.get_ident() {
                            if *ident != "word" {
                                abort!(ident.span(), "Invalid attribute");
                            }
                        } else {
                            abort!(meta.path.span(), "Expected ident");
                        }

                        if let Expr::Lit(literal) = meta.value {
                            if let Lit::Str(str_literal) = literal.lit {
                                words.push(str_literal.value());
                            } else {
                                abort!(literal.span(), "Expected str literal");
                            }
                        } else {
                            abort!(meta.value.span(), "Expected str literal");
                        }
                    }
                    Meta::Path(path) => {
                        if path.is_ident("operator") {
                            operators.push(variant.ident.clone());
                        } else if path.is_ident("initial") {
                            initials.push(variant.ident.clone());
                        } else if path.is_ident("terminal") {
                            terminals.push(variant.ident.clone());
                        }
                    }
                    _ => {
                        abort!(attr.span(), "Expected name = \"value\" pair or path");
                    }
                }
            }
        }

        let lifetime = lifetime.unwrap();
        // Implement ::from(loc: usize, word: &str)
        let from = quote! {
            impl<#(#lifetimes),*> #enum_ident<#(#lifetimes),*> {
                pub fn from(loc: usize, slice: & #lifetime str) -> Self {
                    match slice {
                        #(#words => Self::#variant_idents(Inner::new(loc, slice))),*,
                        _ => Self::Unknown(Inner::new(loc, slice)),
                    }
                }
            }
        };

        // Implement getters for variants
        let impls = quote! {
            impl<#(#lifetimes),*> #enum_ident<#(#lifetimes),*> {
                pub fn inner_mut(&mut self) -> &mut Inner<#(#lifetimes),*> {
                    match self {
                        #(Self::#variant_idents(inner) => inner),*
                    }
                }

                pub fn inner(&self) -> &Inner<#(#lifetimes),*> {
                    match self {
                        #(Self::#variant_idents(inner) => inner),*
                    }
                }

                // pub fn spaced(&self) -> bool {
                //     match self {
                //         #(Self::#variant_idents(inner) => inner.spaced),*
                //     }
                // }

                pub fn loc(&self) -> usize {
                    match self {
                        #(Self::#variant_idents(inner) => inner.loc),*
                    }
                }

                pub fn as_str(&self) -> &str {
                    match self {
                        #(Self::#variant_idents(inner) => inner.slice),*
                    }
                }

                pub fn is_operator(&self) -> bool {
                    match self {
                        #(Self::#operators(_) => true,)*
                        _ => false,
                    }
                }
                pub fn is_initial(&self) -> bool {
                    match self {
                        #(Self::#initials(_) => true,)*
                        _ => false,
                    }
                }
                pub fn is_terminal(&self) -> bool {
                    match self {
                        #(Self::#terminals(_) => true,)*
                        _ => false,
                    }
                }
            }
        };

        quote! {
            #impls

            #from
        }
    } else {
        quote! {}
    };

    output.into()
}

// #[proc_macro_derive(TokenTypeDef, attributes(word, char, pair, operator))]
// pub fn token_type_derive(_tokens: TokenStream) -> TokenStream {
//     let tokens = _tokens.clone();
//     let input = parse_macro_input!(tokens as DeriveInput);
//     // let mut log = File::create("token_macro.log").unwrap();

//     // log.write(format!("{:#?}\n", input).as_bytes()).unwrap();

//     let mut idents: Vec<proc_macro2::TokenStream> = vec![];

//     let mut word_idents: Vec<proc_macro2::TokenStream> = vec![];
//     let mut words: Vec<proc_macro2::TokenStream> = vec![];
//     let mut char_idents: Vec<proc_macro2::TokenStream> = vec![];
//     let mut chars: Vec<proc_macro2::TokenStream> = vec![];

//     let mut pairs: Vec<proc_macro2::TokenStream> = vec![];

//     let mut operator_idents: Vec<proc_macro2::TokenStream> = vec![];
//     let mut operator_precedences: Vec<proc_macro2::TokenStream> = vec![];

//     let ident = input.ident;

//     if let Data::Enum(token_enum) = input.data {
//         for variant in token_enum.variants {
//             let var_ident = variant.ident;
//             idents.push(quote! { #var_ident });

//             for attr in variant.attrs {
//                 let path = attr.meta.path();
//                 if let Some(ident) = path.get_ident() {
//                     match ident.to_string().as_str() {
//                         "word" => {
//                             if let Meta::NameValue(name_value) = attr.meta {
//                                 if let Expr::Lit(value) = name_value.value {
//                                     if let Lit::Str(string) = value.lit {
//                                         word_idents.push(quote! { #var_ident });
//                                         words.push(quote! { #string });
//                                     }
//                                 }
//                             }
//                         }
//                         // "char" => {
//                         //     if let Meta::NameValue(name_value) = attr.meta {
//                         //         if let Expr::Lit(value) = name_value.value {
//                         //             if let Lit::Char(lit_char) = value.lit {
//                         //                 word_idents.push(quote! { #var_ident });
//                         //                 words.push(quote! { #lit_char });
//                         //             } else if let Lit::Str(string) = value.lit {
//                         //                 word_idents.push(quote! { #var_ident });
//                         //                 let c = string.value().chars().next().unwrap();
//                         //                 words.push(quote! { #c });
//                         //             }
//                         //         }
//                         //     }
//                         // }
//                         "pair" => {
//                             if let Meta::List(list) = attr.meta {
//                                 for token in list.tokens {
//                                     if let Ident(identifier) = token {
//                                         pairs.push(quote! { #identifier });
//                                     }
//                                 }
//                             }
//                         }
//                         "operator" => {
//                             if let Meta::List(list) = attr.meta {
//                                 let mut last_identifier = String::new();
//                                 for token in list.tokens {
//                                     match token {
//                                         Ident(identifier) => {
//                                             last_identifier = identifier.to_string();
//                                             operator_idents.push(quote! { #var_ident });
//                                         }
//                                         Literal(lit) => {
//                                             if let "precedence" = last_identifier.as_str() {
//                                                 operator_precedences.push(quote! { #lit });
//                                             }
//                                         }
//                                         _ => (),
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {}
//                     }
//                 }
//             }
//         }
//     }

//     // log.write(format!("{:#?}\n", idents).as_bytes()).unwrap();
//     // log.write(format!("{:#?}\n", words).as_bytes()).unwrap();
//     // log.write(format!("{:#?}\n", pairs).as_bytes()).unwrap();

//     let match_arms = quote! { #(#words => Self::#word_idents),* };
//     // log.write(format!("{:#?}\n", match_arms.to_string()).as_bytes()).unwrap();

//     let from_str = quote! {
//         impl core::convert::From<&str> for #ident {
//             fn from(value: &str) -> Self {
//                 match value {
//                     #match_arms,
//                     _ => Self::Unknown,
//                 }
//             }
//         }
//     };

//     // let match_arms = quote! { #(#chars => Self::#char_idents),* };
//     // let from_char = quote! {
//     //     impl core::convert::From<&char> for #ident {
//     //         fn from(value: &char) -> Self {
//     //             match *value {
//     //                 #match_arms,
//     //                 _ => panic!("Tried parsing character, but no token matched {}", value),
//     //             }
//     //         }
//     //     }
//     // };

//     let match_arms = quote! { #(Self::#pairs => Some(Self::#idents)),* };
//     let get_pair = quote! {
//         fn get_pair(&self) -> Option<Self> {
//             match self {
//                 #match_arms,
//                 _ => None
//             }
//         }
//     };

//     // log.write(format!("{:#?}\n", operator_idents).as_bytes()).unwrap();
//     // log.write(format!("{:#?}\n", operator_precedences).as_bytes()).unwrap();

//     let match_arms = quote! { #(Self::#operator_idents => Some(#operator_precedences)),* };
//     let get_precedence = quote! {
//         // Returns None if not an operator type
//         fn get_precedence(&self) -> Option<u8> {
//             match self {
//                 #match_arms,
//                 _ => None
//             }
//         }
//     };

//     let output = quote! {
//         #from_str

//         impl #ident {
//             pub #get_pair
//             pub #get_precedence
//         }
//     };

//     output.into()
// }
