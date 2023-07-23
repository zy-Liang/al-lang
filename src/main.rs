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
    let file_name = std::env::args().nth(1).unwrap();
    let source = std::fs::read_to_string(file_name).unwrap();
    let mut ctx = Ctx::default();
    let expr = parser::ExprParser::new().parse(&source, &mut ctx, lexer::Lexer::new(&source)).unwrap();
    // println!("{:?}", parser::ExprParser::new().parse("xyz_12xyz").unwrap());
    // println!("{:?}", parser::ExprParser::new().parse("p1234edwef").unwrap());
    println!("{}", evaluator::evaluate(expr, &mut ctx));
}

