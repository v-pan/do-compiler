use crate::span::Span;

use std::{vec, process::id};

#[derive(Debug)]
pub struct SyntaxError {
    description: String
}
impl std::error::Error for SyntaxError {
    fn description(&self) -> &str {
        self.description.as_ref()
    }
}
impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse token.")
    }
}

pub trait Parse: Sized {
    fn parse(contents: &str, span: Span) -> Result<Self, SyntaxError>;
}

#[derive(Clone, Debug)]
pub struct Identifier {
    pub ident: String,
    pub span: Span
}
impl Identifier {
    pub fn new(ident: &str, mut span: Span) -> Self {
        if ident.contains(&['(', ')', '{', '}', ';', ':', ',']) {
            panic!("Invalid identifier {}", ident);
        }

        span.size = ident.len();
        Identifier { ident: ident.to_string(), span }
    }
}
impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ident)
    }
}

#[derive(Clone, Debug)]
pub enum Type {
    Explicit(Identifier, Span),
    Inferred,
}
impl Parse for Type {
    fn parse(contents: &str, mut span: Span) -> Result<Self, SyntaxError> {
        match contents.trim() {
            "" => Ok(Type::Inferred),
            _ => {
                // TODO: Handle things like Type<T, U> and Namespace.Type or Namespace::Type, etc.
                let ident = Identifier::new(contents.trim(), span);

                Ok(Type::Explicit(ident, span))
            }
        }
    }
}

#[derive(Debug)]
pub struct FunctionArgument {
    pub identifier: Identifier,
    pub ty: Type,
    pub span: Span
}
impl std::fmt::Display for FunctionArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ident = &self.identifier;
        let type_string = match &self.ty {
            Type::Explicit(ty_ident, _) => {
                format!(": {ty_ident}")
            }
            Type::Inferred => {
                format!("")
            }
        };
        write!(f, "{ident}{type_string}")
    }
}

#[derive(Debug)]
pub struct FunctionArguments {
    pub arguments: Vec<FunctionArgument>,
    pub span: Span
}

impl Parse for FunctionArguments {
    fn parse(contents: &str, parent_span: Span) -> Result<Self, SyntaxError> {
        let mut contents = contents.trim();
        let mut arguments: Vec<FunctionArgument> = vec![];

        println!("Received contents {}", contents);

        // We expect a comma separated list of 
        //   <ident>: <type>
        // | <ident>
        
        let mut last_loc = Span::after(parent_span);
        loop {
            // Expect a valid identifier
            
            // Look for next marker
            if let Some(ident_end) = contents.find(&[':', ',']) {
                if ident_end == 0 { // Need an identifier
                    panic!("Expected an identifier before ':'");
                }

                let mut ident_span = Span::after(last_loc);

                let ident_str = contents[..ident_end].trim();
                println!("Got ident {ident_str}");

                ident_span.extend(ident_str.len());
                ident_span.end = ident_end - 1;

                let ident = Identifier::new(ident_str, ident_span);

                let remaining = contents.strip_prefix(ident_str).unwrap();

                // Expect a type
                if remaining.starts_with(':') {
                    println!("Remaining: {}", remaining);

                    let type_span = Span::after(ident_span);

                    // Parse the type
                    let ty = if let Some(type_end) = remaining.find(',') {
                        let type_str = remaining[1..type_end].trim();
                        Type::parse(type_str, type_span)?
                    } else { // Expect end of argument list
                        Type::parse(remaining[1..].trim(), type_span)?
                    };

                    // Push the Ident and Type to the argument list
                    if let Type::Explicit(_, ty_span) = ty {
                        // Compute the total span across the argument
                        let mut span = Span::default();
                        span.start = ident_span.start;
                        span.end = ty_span.end;

                        arguments.push(FunctionArgument { identifier: ident, ty, span });
                        last_loc = Span::after(span);
                    } else {
                        panic!("Tried parsing an explicit type, got an inferred type");
                    }
                } else if remaining.starts_with(',') {
                    // Push the argument as inferred type and continue parsing
                    arguments.push(FunctionArgument { identifier: ident, ty: Type::Inferred, span: ident_span });

                    // Move the contents slice and last_loc span forward
                    contents = &remaining[1..];
                    last_loc.end += 1;
                } else if remaining.starts_with(')') {
                    // We reached the end of the argument list
                    break;
                } else {
                    panic!("Expected a closing )");
                }
            } else {
                panic!("Expected either ':', ','");
            }

        }

        let mut args_span = Span::after(parent_span);

        let end = if let Some(arg) = arguments.last() {
            arg.span.end
        } else {
            panic!("Expected a parsed argument");
        };

        args_span.size = end;
        args_span.end = end;

        Ok(FunctionArguments { arguments, span: args_span })
    }
}
impl std::fmt::Display for FunctionArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut args_string = String::new();
        for arg in &self.arguments {
            args_string += format!("{}", arg.identifier).as_ref();
            if let Type::Explicit(ty_ident, _) = &arg.ty {
                args_string += format!(": {ty_ident}").as_ref();
            }
        }
        write!(f, "{args_string}")
    }
}

#[derive(Debug)]
pub struct Function {
    pub identifier: Identifier,
    pub arguments: Option<FunctionArguments>,
    pub return_type: Type,
    pub span: Span
}
impl Parse for Function {
    fn parse(contents: &str, span: Span) -> Result<Self, SyntaxError> {
        // We no longer expect to parse 'fun'
        //// Expect `fun`
        //// if contents.starts_with("fun") {
        ////     fun_span.extend(contents.drain(.."fun".len()).collect::<Vec<_>>().len());
        //// } else {
        ////     panic!("Didn't find `fun`!");
        ////     // return Err(Error) // Should never happen
        //// }

        let mut idx = 0;

        // Expect identifier 
        let identifier = if let Some(b_idx) = contents.find('(') {
            let mut ident_span = Span::after(span);
            
            let ident_str = &contents[..b_idx].trim();
            println!("Found function ident {ident_str}");

            ident_span.extend(b_idx);
            idx += b_idx+1;

            Identifier::new(ident_str, ident_span)
        } else {
            panic!("Expected an opening (");
        };

        // Parse args list
        let arguments: Option<FunctionArguments> = if contents[identifier.span.end..].starts_with(')') {
            // There are no arguments, we can continue
            None
        } else {
            // We need to parse the argument list
            Some(FunctionArguments::parse(&contents[idx..], Span::after(identifier.span))?)
        };

        let last_span = if let Some(args) = &arguments {
            args.span
        } else {
            identifier.span
        };

        let return_type = if contents[last_span.end..].starts_with(')') {
            if let Some(body_start) = contents[last_span.end..].find('{') {
                let pre_body = &contents[last_span.end+1 .. body_start].trim();
                if pre_body.starts_with(':') {
                    Type::parse(&pre_body[1..], Span::after(last_span))?
                } else {
                    Type::Inferred
                }
            } else {
                panic!("Expected a function body");
            }
        } else {
            panic!("Expected a closing )");
        };

        let len = contents.len();
        let fun_span = Span::new(span.start, len, len);

        Ok(Function {
            identifier,
            arguments,
            return_type,
            span: fun_span
        })
    }
}
impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fun_ident = format!("{}", &self.identifier);
        let fun_args = &self.arguments;

        let args_string = if let Some(args) = fun_args {
            format!("{args}")
        } else {
            format!("")
        };

        let type_string = match &self.return_type {
            Type::Explicit(ret_ident, _) => {
                format!(": {ret_ident}")
            }
            Type::Inferred => {
                format!("")
            }
        };

        write!(f, "{fun_ident}({args_string}){type_string}")
    }
}
