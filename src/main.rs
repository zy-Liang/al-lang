pub mod syntax;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub parser, "/parser.rs");

fn main() {
    let to_parse = vec!["1.90", "-1.0", "1+1", "2*5"];
    for parse in to_parse {
        if let Ok(res) = parser::PlusMinusParser::new().parse(parse) {
            println!("{:?}", res);
        } else {
            println!("parse failed: {}", parse)
        }
    }
}
