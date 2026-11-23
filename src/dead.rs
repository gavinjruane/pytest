fn interpret_str(expr: &Expr) -> String {
    match expr {
        Expr::Num(n) => format!("Num: {}", n),
        Expr::Str(s) => format!("Str: {}", s),
        Expr::Id(i) => format!("Id: {}", i),
        Expr::Cond(c) => format!(
            "Cond: {}, If-True: {}, If-False: {}",
            interpret_str(&*c.cond),
            interpret_str(&*c.if_true),
            interpret_str(&*c.if_false)
        ),
        Expr::Func(f) => format!(
            "Parameters: {:?}, Body: {}",
            f.parameters,
            interpret_str(&*f.body)
        ),
        Expr::Appl(a) => format!(
            "Expression: {}, Arguments: {:?}",
            interpret_str(&*a.expr),
            a.arguments
                .iter()
                .map(|arg| interpret_str(arg))
                .collect::<Vec<String>>()
        ),
        // _ => format!("error: unknown type"),
    }
}
