use crate::token::Token;

#[derive(Clone, Copy)]
pub(super) enum ParseState<'t> {
    TopLevel,

    FunctionDecl,
    DeclNameAndParams,

    Expression(ExpressionState<'t>),
}

impl<'t> From<ExpressionState<'t>> for ParseState<'t> {
    fn from(v: ExpressionState<'t>) -> Self {
        Self::Expression(v)
    }
}

#[derive(Clone, Copy)]
pub(super) struct ExpressionState<'t> {
    pub(super) initiator: Token<'t>,
}

impl<'t> ExpressionState<'t> {
    pub(super) fn new(initiator: Token<'t>) -> Self {
        Self { initiator }
    }
}
