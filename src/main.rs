pub mod syntax;
pub mod evaluator;
pub mod lexer;

mod test {
    mod test_parse;
    mod test_eval;
}

use lalrpop_util::lalrpop_mod;
use crate::syntax::Ctx;

lalrpop_mod!(pub parser, "/parser.rs");

fn main() {
    let source  = "let x = 1 + 2 + 3 + 4 in 1 + x";
    let mut ctx = Ctx::default();
    println!("{:?}", parser::ExprParser::new().parse(&source, &mut ctx, lexer::Lexer::new(source)).unwrap());
    // println!("{:?}", parser::ExprParser::new().parse("xyz_12xyz").unwrap());
    // println!("{:?}", parser::ExprParser::new().parse("p1234edwef").unwrap());
}

