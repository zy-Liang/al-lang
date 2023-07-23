
use crate::syntax::*;

pub fn evaluate(e: Expr, ctx: &mut Ctx) -> f64 {
    match e {
        Expr::Num(n) => n,
        Expr::Neg(e) => -1.0 * evaluate(*e, ctx),
        Expr::Plus(l, r) => evaluate(*l, ctx) + evaluate(*r, ctx),
        Expr::Minus(l, r) => evaluate(*l, ctx) - evaluate(*r, ctx),
        Expr::Times(l, r) => evaluate(*l, ctx) * evaluate(*r, ctx),
        Expr::Var(x) => {
            if let Some(v) = ctx.lookup.get(&x) {
                *v
            } else {
                panic!("unbound variable: {}", x)
            }
        },
        Expr::Let { binder, e1, e2 } => {
            let v = evaluate(*e1, ctx);
            ctx.lookup.insert(binder.clone(), v);
            let r = evaluate(*e2, ctx);
            ctx.lookup.remove(&binder);
            r
        },
        
    }
}
