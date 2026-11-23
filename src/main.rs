mod defs;
mod stdcall;

use defs::{ClosureV, Expr, Value};

fn main() {
    // let prog1 = Expr::Num(10);
    // let cond1 = CondC {
    //     cond: Box::new(Expr::Num(1)),
    //     if_true: Box::new(Expr::Num(10)),
    //     if_false: Box::new(Expr::Num(30)),
    // };
    // let prog2 = Expr::Cond(cond1);

    // println!("{}", interpret_str(&prog1));
    // println!("{}", interpret_str(&prog2));
}

fn interpret(expr: &Expr) -> Value {
    match expr {
        Expr::Num(n) => Value::Num(*n),
        Expr::Str(s) => Value::Str(String::from(s)),
        Expr::Id(_) => panic!("error: unhandled type \"id\""),
        Expr::Cond(c) => match interpret(&*c.cond) {
            Value::Bool(true) => interpret(&*c.if_true),
            Value::Bool(false) => interpret(&*c.if_false),
            _ => panic!("error: condition did not evaluate to a bool"),
        },
        Expr::Func(f) => Value::Closure(ClosureV::new(f)),
        Expr::Appl(a) => match interpret(&*a.expr) {
            Value::StdCall(_) => panic!("error: StdCall not implemented"),
            Value::Closure(_) => panic!("error: closure handling not implemented"),
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

    #[test]
    fn test_func() {
        let prog = Expr::Func(FuncC {
            parameters: vec![String::from("x")],
            body: Box::new(Expr::Num(3)),
        });
        assert_eq!(
            Value::Closure(ClosureV {
                parameters: vec![String::from("x")],
                body: Box::new(Expr::Num(3)),
            }),
            interpret(&prog)
        );
    }

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
