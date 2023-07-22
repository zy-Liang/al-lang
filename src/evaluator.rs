
use crate::syntax::*;

pub fn evaluate(e: Expr) -> f64 {
    match e {
        Expr::Num(n) => n,
        Expr::Neg(e) => -1.0 * evaluate(*e),
        Expr::Plus(l, r) => evaluate(*l) + evaluate(*r),
        Expr::Minus(l, r) => evaluate(*l) - evaluate(*r),
        Expr::Times(l, r) => evaluate(*l) * evaluate(*r),
    }
}
