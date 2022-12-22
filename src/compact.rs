use tan::parser::expr::Expr;

pub fn format_compact(expr: &Expr) -> String {
    match expr {
        Expr::One => "()".to_owned(),
        Expr::Bool(b) => b.to_string(),
        Expr::Int(n) => n.to_string(),
        Expr::Float(n) => n.to_string(),
        Expr::Symbol(s) => s.clone(),
        Expr::String(s) => s.clone(),
        Expr::List(terms) => {
            format!(
                "({})",
                terms
                    .iter()
                    .map(|term| format!("{}", term.as_ref()))
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        }
        Expr::Func(..) => "#<func>".to_owned(),
    }
}
