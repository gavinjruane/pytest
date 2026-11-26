mod defs;
mod stdcall;

use defs::{Expr, Store, Value};

use crate::defs::Context;

fn main() {
    // let (context, mut store) = init();
}

fn init() -> (Context, Store) {
    let items: Vec<String> = vec![
        String::from("true"),
        String::from("false"),
        String::from("none"),
        String::from("eq"),
        String::from("error"),
    ];
    let values: Vec<Value> = vec![
        Value::Bool(true),
        Value::Bool(false),
        Value::None,
        Value::StdCall(String::from("eq")),
        Value::StdCall(String::from("error")),
    ];

    let store = Store::new(&values);

    (
        Context {
            environment: defs::new_environment(&items),
            stdcalls: stdcall::new_stdcall_table(&stdcall::STDCALL_STRS, &stdcall::STDCALL_FNS),
        },
        store,
    )
}

// fn interpret(expr: &Expr) -> Value {
fn interpret(expr: &Expr, context: &Context, store: &mut Store) -> Value {
    match expr {
        Expr::Num(n) => Value::Num(*n),
        Expr::Str(s) => Value::Str(String::from(s)),
        Expr::Id(id) => store.lookup(id, &context.environment).clone(),
        // Expr::Id(id) => match context.environment.get(id) {
        //     Some(loc) => match store.get(*loc) {
        //         Some(val) => val.clone(),
        //         None => panic!("error: invalid val"),
        //     },
        //     None => panic!("error: invalid id"),
        // },
        Expr::Cond {
            cond,
            if_true,
            if_false,
        } => match interpret(&*cond, context, store) {
            Value::Bool(true) => interpret(&*if_true, context, store),
            Value::Bool(false) => interpret(&*if_false, context, store),
            _ => panic!("error: condition did not evaluate to a bool"),
        },
        Expr::Func { parameters, body } => Value::Closure {
            parameters: parameters.clone(),
            body: body.clone(),
            closure_env: context.environment.clone(),
        },
        Expr::Appl { expr, arguments } => match interpret(&*expr, context, store) {
            Value::StdCall(name) => {
                let Some(stdcall) = context.stdcalls.get(&name) else {
                    panic!("error: no stdcall {}", name);
                };

                match stdcall(
                    arguments
                        .iter()
                        .map(|arg| interpret(arg, context, store))
                        .collect(),
                ) {
                    Ok(val) => val,
                    Err(err) => panic!("{}", err),
                }
            }
            Value::Closure {
                parameters,
                body,
                closure_env,
            } => {
                if parameters.len() == arguments.len() {
                    let args: Vec<Value> = arguments
                        .iter()
                        .map(|it| interpret(it, context, store))
                        .collect();
                    let new_env = parameters
                        .iter()
                        .zip(args.iter())
                        .fold(closure_env, |env, (param, arg)| {
                            store.bind(String::from(param), arg.clone(), env)
                        });
                    let new_context = Context {
                        environment: new_env,
                        stdcalls: context.stdcalls.clone(),
                    };
                    interpret(&*body, &new_context, store)
                } else {
                    panic!("error: arity mismatch to function");
                }
            }
            _ => panic!("error: not callable"),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num() {
        let (context, mut store) = init();
        let prog = Expr::Num(100);
        assert_eq!(Value::Num(100), interpret(&prog, &context, &mut store));
    }

    #[test]
    fn test_str() {
        let (context, mut store) = init();
        let prog = Expr::Str(String::from("Hello!"));
        assert_eq!(
            Value::Str(String::from("Hello!")),
            interpret(&prog, &context, &mut store)
        );
    }

    #[test]
    fn test_id() {
        let (context, mut store) = init();
        let prog = Expr::Id(String::from("true"));
        assert_eq!(Value::Bool(true), interpret(&prog, &context, &mut store));
    }

    // #[test]
    // fn test_cond() {
    //     let context = init();
    //     let prog = Expr::Cond {
    //         cond: Box::new(),
    //         if_true: Box::new(Expr::Num(5)),
    //         if_false: Box::new(Expr::Num(7)),
    //     };
    //     assert_eq!(Value::Bool(true), interpret(&prog, &context));
    // }

    // #[test]
    // fn test_func() {
    //     let prog = Expr::Func(FuncC {
    //         parameters: vec![String::from("x")],
    //         body: Box::new(Expr::Num(3)),
    //     });
    //     assert_eq!(
    //         Value::Closure(ClosureV {
    //             parameters: vec![String::from("x")],
    //             body: Box::new(Expr::Num(3)),
    //         }),
    //         interpret(&prog)
    //     );
    // }

    #[test]
    fn test_appl_false() {
        let (context, mut store) = init();
        let prog = Expr::Appl {
            expr: Box::new(Expr::Id(String::from("eq"))),
            arguments: vec![Expr::Num(3), Expr::Num(4)],
        };
        assert_eq!(Value::Bool(false), interpret(&prog, &context, &mut store));
    }

    #[test]
    fn test_appl_true() {
        let (context, mut store) = init();
        let prog = Expr::Appl {
            expr: Box::new(Expr::Id(String::from("eq"))),
            arguments: vec![Expr::Num(5), Expr::Num(5)],
        };
        assert_eq!(Value::Bool(true), interpret(&prog, &context, &mut store));
    }

    #[test]
    #[should_panic]
    fn test_error_stdcall() {
        let (context, mut store) = init();
        let prog = Expr::Appl {
            expr: Box::new(Expr::Id(String::from("error"))),
            arguments: vec![
                Expr::Str(String::from("FileNotFoundErrror")),
                Expr::Str(String::from("file not found")),
            ],
        };
        interpret(&prog, &context, &mut store);
    }

    // #[test]
    // fn test_appl_() {
    //     let (context, mut store) = init();
    //     let prog = Expr::Appl {
    //         expr: Box::new(Expr::Id(String::from("eq"))),
    //         arguments: vec![Expr::Num(5), Expr::Num(5)],
    //     };
    //     assert_eq!(Value::Bool(true), interpret(&prog, &context, &mut store));
    // }
}
