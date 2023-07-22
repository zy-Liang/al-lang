#[derive(Debug)]
pub enum Expr {
    Num(f64),
    Neg(Box<Expr>),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Times(Box<Expr>, Box<Expr>),
}
