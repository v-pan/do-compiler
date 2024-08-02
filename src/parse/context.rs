use string_interner::backend::Backend;

pub enum ParseCtx<B: string_interner::backend::Backend> {
    ModifierCtx(usize),
    FunctionCtx(FunctionCtx),
    VariableCtx,
    IdentifierCtx(<B as Backend>::Symbol),
    EqualsCtx,
    PlusCtx,
}

#[derive(Debug)]
pub struct FunctionCtx {
    
}

impl FunctionCtx {
    pub fn new<B: string_interner::backend::Backend>() -> ParseCtx<B> {
        ParseCtx::FunctionCtx(FunctionCtx { })
    }
}

pub struct ModifierCtx {
    pub placeholder_idx: usize,
}
