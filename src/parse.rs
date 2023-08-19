use crate::span::Span;

use std::vec;

#[derive(Debug)]
pub struct Error {
    description: String
}
impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.description.as_ref()
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse token.")
    }
}

pub trait Parse: Sized {
    fn parse(contents: &mut String, span: Span) -> Result<Self, Error>;
}

pub enum Node {
    Fun(Function),
    Unknown,
}

#[derive(Clone, Debug)]
pub struct Identifier {
    pub ident: String,
    pub span: Span
}
impl Identifier {
    pub fn new(ident: &str, mut span: Span) -> Self {
        let size = ident.len();
        span.size = size;
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
    fn parse(contents: &mut String, mut span: Span) -> Result<Self, Error> {
        println!("Type parser received {}", contents);
        match contents.trim() {
            "" => Ok(Type::Inferred),
            _ => {
                // TODO: Handle things like Type<T, U> and Namespace.Type or Namespace::Type, etc.
                // contents.find(&["<"])
                if let Some(end) = contents.find(&[',', ';', ')', '{']) {
                    let ty_string: String = contents.drain(..end).collect();
                    span.end = span.start + end;

                    println!("Removing type ident {}", ty_string);
                    
                    Ok(Type::Explicit(Identifier::new(ty_string.trim(), span), span))
                } else {
                    panic!("Failed to handle contents");
                }
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
    // TODO: Require that a function is either entirely statically typed or dynamically typed.
    fn parse(contents: &mut String, parent_span: Span) -> Result<Self, Error> {
        if contents.starts_with(')') {
            panic!("No arguments to parse");
        }
        if contents.starts_with(&[':', ',']) {
            panic!("Expected an identifier");
        }
        
        let mut arguments: Vec<FunctionArgument> = vec![];

        // We expect a comma separated list of 
        //   <ident>: <type>
        // | <ident>
        // terminated eventually with a )
        
        let mut last_loc = Span::after(parent_span);
        loop {
            println!("contents: {}", contents);

            // Expect a valid identifier
            if contents.starts_with(&[':', ',', ')']) {
                panic!("Too many separators");
            }
            
            // Look for next separator or end of argument list
            if let Some(ident_end) = contents.find(&[':', ',', ')']) {
                if ident_end == 0 { // Need an identifier
                    panic!("No identifier found before ':'");
                }

                let mut ident_string: String = contents.drain(..ident_end+1).collect();
                let sep = ident_string.pop().expect("There should always be a separator");

                println!("Removing ident {}", ident_string);

                let mut ident_span = Span::after(last_loc);
                ident_span.end = ident_string.len() + 1;

                let ident = Identifier::new(ident_string.trim(), ident_span);

                match sep {
                    // Expect a type
                    ':' => {
                        println!("Type contents: {}", contents);
                        // Parse the type
                        let ty = Type::parse(contents, Span::after(ident.span))?;
                        
                        println!("Contents after type parsing: {}", contents);

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

                        // Check if type was followed by a ')' to break out of parsing
                        let end = contents.find(&[',', ')']).expect("Expected at least a ')' before end of arguments list");
                        let ty_sep = contents.drain(..end + 1).last().expect("Expected a seperator");

                        if ty_sep == ')' {
                            break;
                        } else if ty_sep == ',' {} else {
                            panic!("Expected either ',' or ')'");
                        }
                    }
                    // Push the identifier and continue parsing
                    ',' => {
                        arguments.push(FunctionArgument { identifier: ident, ty: Type::Inferred, span: ident_span });
                        last_loc = Span::after(ident_span);
                    }
                    ')' => { break; }
                    _ => { panic!("Got an unexpected separator") }
                }
            } else {
                panic!("Expected either ':', ',' or ')'");
            }

        }

        let mut args_span = Span::after(parent_span);

        let end = if let Some(arg) = arguments.last() {
            arg.span.end
        } else {
            panic!("Expected there to be a parsed argument");
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
    fn parse(contents: &mut String, span: Span) -> Result<Self, Error> {
        let mut fun_span = Span::after(span);

        // let contents = &mut contents.replace(" ", "");

        // Expect `fun`
        if contents.starts_with("fun") {
            fun_span.extend(contents.drain(.."fun".len()).collect::<Vec<_>>().len());
        } else {
            panic!("Didn't find `fun`!");
            // return Err(Error) // Should never happen
        }

        // Expect identifier 
        let identifier = if let Some(b_idx) = contents.find("(") {
            let mut ident_string: String = contents.drain(..b_idx+1).collect();

            // TODO: Explicitly check this was the expected open bracket
            ident_string.pop().expect("Expected to remove an ("); // Remove (

            println!("Removing ident {}", ident_string);

            let i = Identifier::new(ident_string.trim(), Span::after(fun_span));

            fun_span.extend(b_idx+1);

            i
        } else {
            panic!("Didn't find an identifier!");
            // Function needs an identifier (right now anyways)
            // return Err(Error)
        };

        // Optional args list
        let arguments: Option<FunctionArguments> = if contents.starts_with(")") {
            // There are no arguments, we can continue
            None
        } else {
            // We need to parse the argument list
            Some(FunctionArguments::parse(contents, Span::after(fun_span))?)
        };

        // Move the span forward to the end of the argument list, before the `)`
        if let Some(args) = &arguments {
            fun_span.extend(args.span.size);

            // let _: Vec<_> = contents.drain(..args.span.size).collect();
        }

        // Expect Type or `{`
        let return_type = if let Some(body_start) = contents.find('{') {
            let mut pre_body: String = contents.drain(1..body_start+1).collect();

            Type::parse(&mut pre_body, Span::after(fun_span))?
        } else {
            panic!("Expected a function body");
        };

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
