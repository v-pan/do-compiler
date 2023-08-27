// use crate::{span::Span, parse::SyntaxError};
//
// pub enum Token {
//     // Either keywords, identifiers or literals
//     Word(Span),
//
//     OpenBracket(Bracket, Span),
//     CloseBracket(Bracket, Span),
//
//     OpenQuote(Quote, Span),
//     CloseQuote(Quote, Span),
//
//     Period(Span),
//     Comma(Span),
//
//     // +, -, /, *, etc.
//     Operator(Operator, Span),
//
//     // :
//     TypeSpecifier(Span)
// }
//
// pub enum Bracket {
//     Round,
//     Angle,
//     Curly
// }
//
// pub enum Quote {
//     Double,
//     Single,
//     Backtick
// }
//
// pub enum Operator {
//     Add,
//     Subtract,
//     Multiply,
//     Divide,
//     LessThan,
//     LessThanEq,
//     GreaterThan,
//     GreaterThanEq,
//     Equality,
//     AND,
//     OR,
//     NOT,
//     XOR
// }
//
// fn is_operator(op: &str) -> bool {
//     match op {
//         "+" => true,
//         "-" => true,
//         "/" => true,
//         "*" => true,
//         "<" => true,
//         "<=" => true,
//         ">" => true,
//         ">=" => true,
//         "=" => true,
//         "==" => true,
//         _ => false,
//     }
// }
//
// /**
// True if c is one of + , - . / : ; < = > ?
// */
// fn is_symbol(c: char) -> bool {
//     (43 < (c as u8)) && ((c as u8) < 47) && (58 < (c as u8)) && ((c as u8) < 63)
// }
//
// pub struct TokenParser;
// impl TokenParser {
//     pub fn parse(target: &str, mut span: Span) -> Result<Vec<Token>, SyntaxError> {
//         let mut tokens: Vec<Token> = vec![];
//
//         let mut last_idx = 0;
//
//         let mut stack: Vec<&mut Vec<Token>> = vec![&mut tokens];
//         let mut scope_span: &mut Span;
//
//         for (byte_idx, byte) in target.bytes().enumerate() {
//             match byte {
//                 b' ' => {
//                     if let Some(keyword_token) = parse_keyword(&target[last_idx .. byte_idx], Span::new(span.start + last_idx, 0)) {
//                         stack.last().unwrap().push(keyword_token);
//                     }
//                 },
//                 _ => { continue; }
//             }
//             
//             last_idx = byte_idx;
//         }
//
//         Ok(tokens)
//     }
// }
//
// // fn parse_keyword(target: &str, mut span: Span) -> Option<Token> {
// //     match target {
// //         // "include" => { /* TODO */ }
// //         "fun" => {
// //             let len = "fun".len();
// //
// //             span.extend(len);
// //
// //             Some(Token::Word(Keyword::FunctionDeclaration, span))
// //         }
// //         // "struct" => { /* TODO */ }
// //         // "class" => { /* TODO */ }
// //         // "val" => { /* TODO */ }
// //         // "var" => { /* TODO */ }
// //         _ => { None }
// //     }
// // }
