#[derive(Debug, PartialEq)]
pub enum Expr {
    Var(String),
    Let{
        binder: String,
        e1: Box<Expr>,
        e2: Box<Expr>,
    },
    Num(f64),
    Neg(Box<Expr>),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Times(Box<Expr>, Box<Expr>),
}

// let x = 1 + 2 in x + 1
#[derive(Default, Debug)]
pub struct Ctx {
}