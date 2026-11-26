use std::collections::HashMap;

use crate::Value;

pub type StdCall = fn(Vec<Value>) -> Result<Value, String>;

pub static STDCALL_STRS: [&str; 2] = ["eq", "error"];
pub static STDCALL_FNS: [StdCall; 2] = [eq, error];

pub fn new_stdcall_table(names: &[&str], funcs: &[StdCall]) -> HashMap<String, StdCall> {
    let mut stdcalls: HashMap<String, StdCall> = HashMap::new();

    for (name, func) in names.iter().zip(funcs.iter()) {
        stdcalls.insert(String::from(*name), *func);
    }

    stdcalls
}

pub fn eq(args: Vec<Value>) -> Result<Value, String> {
    let result = match args.as_slice() {
        [Value::Str(a), Value::Str(b)] => a == b,
        [Value::Num(a), Value::Num(b)] => a == b,
        [Value::Bool(a), Value::Bool(b)] => a == b,
        [Value::None, Value::None] => true,
        [_, _] => false,
        [_] => false,
        _ => false,
    };

    Ok(Value::Bool(result))
}

pub fn strlen(args: Vec<Value>) -> Value {
    match args.as_slice() {
        [Value::Str(s)] => Value::Num(s.len()),
        [_] => panic!("error: argument is not a string"),
        _ => panic!("error: arity mismatch"),
    }
}

pub fn error(args: Vec<Value>) -> Result<Value, String> {
    let error = match args.as_slice() {
        [Value::Str(category), Value::Str(err)] => format!("{}: {}", category, err),
        _ => panic!("error: error() called with invalid arguments"),
    };

    Err(error)
}
