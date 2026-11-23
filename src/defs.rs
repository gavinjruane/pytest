use std::vec::Vec;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Num(u32),
    Str(String),
    Id(String),
    Cond(CondC),
    Func(FuncC),
    Appl(ApplC),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Num(u32),
    Str(String),
    Bool(bool),
    StdCall(String),
    Closure(ClosureV),
    None,
    Undefined,
}

pub type ExprBox = Box<Expr>;

#[derive(Clone, Debug, PartialEq)]
pub struct CondC {
    cond: ExprBox,
    if_true: ExprBox,
    if_false: ExprBox,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FuncC {
    parameters: Vec<String>,
    body: ExprBox,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ApplC {
    expr: ExprBox,
    arguments: Vec<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClosureV {
    parameters: Vec<String>,
    body: ExprBox,
}

impl ClosureV {
    pub fn new(func: &FuncC) -> Self {
        ClosureV {
            parameters: func.parameters.clone(),
            body: func.body.clone(),
        }
    }
}
