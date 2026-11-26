use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::vec::Vec;

use crate::stdcall::StdCall;

// pub type Store = Vec<Value>;
#[derive(Debug, Clone, PartialEq, Default)]
// pub struct Store(pub Vec<Value>);
pub struct Store {
    mem: Vec<Value>,
    next_free: usize,
}

impl Store {
    fn next_free(&self) -> usize {
        match self.get(0) {
            Some(Value::Num(n)) => *n,
            Some(_) => panic!("error: next free block was not a number"),
            None => panic!("error: invalid next free block"),
        }
    }

    fn set_next_free(&mut self, next: usize) -> () {
        if let Some(element) = self.get_mut(0) {
            *element = Value::Num(next);
        } else {
            panic!("error: out of bounds access for Store");
        }
    }

    fn allocate(&mut self, blocks: usize, value: &Value) -> usize {
        let mut next_free = self.next_free();
        let base = next_free;
        for _ in 0..blocks {
            if next_free >= self.mem.len() {
                panic!("error: out of memory");
            } else {
                self.mem[next_free] = value.clone();
                self.set_next_free(next_free + 1);
                next_free = self.next_free();
            }
        }

        base
    }

    pub fn bind(&mut self, id: String, value: Value, mut env: Environment) -> Environment {
        env.insert(id, self.allocate(1, &value));
        env
    }

    pub fn lookup(&self, id: &String, env: &Environment) -> &Value {
        let Some(loc) = env.get(id) else {
            panic!("error: invalid id {}", id);
        };

        let Some(val) = self.mem.get(*loc) else {
            panic!("error: no value at address {}", loc);
        };

        &val
    }

    pub fn new(values: &Vec<Value>) -> Store {
        let mut store = Store {
            mem: Vec::new(),
            next_free: 0,
        };
        for value in values.into_iter() {
            store.push(value.clone());
        }

        store
    }
}

impl Deref for Store {
    type Target = Vec<Value>;

    fn deref(&self) -> &Self::Target {
        &self.mem
    }
}

impl DerefMut for Store {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mem
    }
}

pub type Environment = HashMap<String, usize>;

pub fn new_environment(items: &Vec<String>) -> Environment {
    let mut environment: Environment = HashMap::new();
    for (index, item) in items.into_iter().enumerate() {
        environment.insert(String::from(item), index);
    }
    environment
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Num(usize),
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
    Num(usize),
    Str(String),
    Bool(bool),
    StdCall(String),
    Closure {
        parameters: Vec<String>,
        body: Box<Expr>,
        closure_env: Environment,
    },
    None,
    Undefined,
}

pub struct Context {
    pub environment: Environment,
    pub stdcalls: HashMap<String, StdCall>,
}
