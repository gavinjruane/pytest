use std::collections::HashMap;
use std::vec::Vec;

pub type Store = Vec<Value>;

pub type Environment = HashMap<String, u32>;

pub fn new_environment(items: &Vec<String>) -> Environment {
    let mut environment: Environment = HashMap::new();
    for (index, item) in items.into_iter().enumerate() {
        let loc: u32 = u32::try_from(index).expect("error: location is too large");
        environment.insert(String::from(item), loc);
    }
    environment
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Num(u32),
    Str(String),
    Id(String),
    Cond {
        cond: Box<Expr>,
        if_true: Box<Expr>,
        if_false: Box<Expr>,
    },
    Func {
        parameters: Vec<String>,
        body: Box<Expr>,
    },
    Appl {
        expr: Box<Expr>,
        arguments: Vec<Expr>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Num(u32),
    Str(String),
    Bool(bool),
    StdCall(String),
    Closure {
        parameters: Vec<String>,
        body: Box<Expr>,
    },
    None,
    Undefined,
}
