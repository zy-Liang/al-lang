pub mod syntax;
pub mod evaluator;

mod test {
    mod test_parse;
    mod test_eval;
}

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub parser, "/parser.rs");

fn main() {
    
}

