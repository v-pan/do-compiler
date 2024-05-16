#[derive(Debug)]
pub enum ParseCtx {
    ModifierCtx(usize),
    FunctionCtx(FunctionCtx),
}

#[derive(Debug)]
pub struct FunctionCtx {
    
}

impl FunctionCtx {
    pub fn new() -> ParseCtx {
        ParseCtx::FunctionCtx(FunctionCtx { })
    }
}

pub struct ModifierCtx {
    pub placeholder_idx: usize,
}
