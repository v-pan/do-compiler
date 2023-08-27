use crate::Span;
use super::{SyntaxError, Parse};
use super::identifier::Identifier;
use super::list::List;
use super::r#type::Type;

use std::io::{BufReader, Read};
use std::fs::File;

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
impl FunctionArgument {
    fn from(value: String, span: Span) -> Self {
        let parts: Vec<_> = value.split(":").collect();

        let identifier = Identifier::new(parts[0], span);
        let ty = Type::from(parts[1], span);

        let span = Span::new(span.start, value.len());

        FunctionArgument { identifier, ty, span }
    }
}

#[derive(Debug)]
pub struct FunctionArguments {
    pub arguments: Vec<FunctionArgument>,
    pub span: Span
}

impl Parse for FunctionArguments {
    fn parse(reader: &mut BufReader<&File>, parent_span: Span) -> Result<Self, SyntaxError> {
        let mut arguments: Vec<FunctionArgument> = vec![];
        
        // Read in the opening (
        reader.read(&mut [0;1]).unwrap();

        let list = List::parse(reader, parent_span)?;

        let mut last_span = Span::after(parent_span);
        for item in list.items {
            let arg = FunctionArgument::from(item, last_span);
            last_span = arg.span;
            arguments.push(arg);
        }

        Ok(FunctionArguments { arguments, span: Span::new(parent_span.start, last_span.start + last_span.size) })
    }
}
impl std::fmt::Display for FunctionArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut args_string = String::new();
        for arg in &self.arguments {
            args_string += format!("{}", arg.identifier).as_ref();
            if let Type::Explicit(ty_ident, _) = &arg.ty {
                args_string += format!(": {ty_ident},").as_ref();
            } else {
                args_string += ","
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
    fn parse(reader: &mut BufReader<&File>, span: Span) -> Result<Self, SyntaxError> {
        let identifier = Identifier::parse(reader, span)?;

        // Parse args list
        let arguments = FunctionArguments::parse(reader, Span::after(identifier.span))?;

        // Parse return type
        // TODO


        let mut fun_span = Span::after(span);
        fun_span.size = (arguments.span.start + arguments.span.size) - fun_span.start;
        
        Ok(Function {
            identifier,
            arguments: Some(arguments),
            return_type: Type::Inferred,
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
