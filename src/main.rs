pub mod syntax;

mod test {
    mod test_parse;
}

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub parser, "/parser.rs");

fn main() {
}

