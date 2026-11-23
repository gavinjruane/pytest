mod defs;
mod stdcall;

use defs::{Expr, Value};

fn main() {
    let items: Vec<String> = vec![
        String::from("true"),
        String::from("false"),
        String::from("none"),
        String::from("eq"),
    ];
    let values: Vec<Value> = vec![
        Value::Bool(true),
        Value::Bool(false),
        Value::None,
        Value::StdCall(String::from("eq")),
    ];

    let environment = defs::new_environment(&items);
}

fn interpret(expr: &Expr) -> Value {
    match expr {
        Expr::Num(n) => Value::Num(*n),
        Expr::Str(s) => Value::Str(String::from(s)),
        Expr::Id(_) => panic!("error: unhandled type \"id\""),
        Expr::Cond {
            cond,
            if_true,
            if_false,
        } => match interpret(&*cond) {
            Value::Bool(true) => interpret(&*if_true),
            Value::Bool(false) => interpret(&*if_false),
            _ => panic!("error: condition did not evaluate to a bool"),
        },
        Expr::Func { parameters, body } => Value::Closure {
            parameters: parameters.clone(),
            body: body.clone(),
        },
        Expr::Appl { expr, .. } => match interpret(&*expr) {
            Value::StdCall(_) => panic!("error: StdCall not implemented"),
            Value::Closure { .. } => {
                panic!("error: closure handling not implemented")
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
        let prog = Expr::Num(100);
        assert_eq!(Value::Num(100), interpret(&prog));
    }

    #[test]
    fn test_str() {
        let prog = Expr::Str(String::from("Hello!"));
        assert_eq!(Value::Str(String::from("Hello!")), interpret(&prog));
    }

    // #[test]
    // fn test_id() {
    //     let prog = Expr::Id(String::from("x"));
    //     assert
    // }

    // #[test]
    // fn test_cond() {
    //     let prog = Expr::Cond(CondC {
    //         cond: Box::new(Expr::Num(3)),
    //         if_true: Box::new(Expr::Num(5)),
    //         if_false: Box::new(Expr::Num(7)),
    //     });
    //     assert_eq!(Value::Bool(true), interpret(&prog));
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

    // #[test]
    // fn test_appl() {
    //     let prog = Expr::Appl(ApplC {
    //         expr: Box::new(Expr::Num(3)),
    //         arguments: vec![Expr::Id(String::from("x"))],
    //     });
    //     assert_eq!(
    //         "Expression: Num: 3, Arguments: [\"Id: x\"]",
    //         interpret_str(&prog)
    //     );
    // }
}
