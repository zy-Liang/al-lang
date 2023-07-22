use crate::parser;

#[test]

fn t1() {
    let parse = "1";
    let res = parser::ExprParser::new().parse(parse).unwrap();
    println!("{}, {:?}", parse, res);
}

#[test]
fn t2() {
    let parse = "1.01";
    let res = parser::ExprParser::new().parse(parse).unwrap();
    println!("{}, {:?}", parse, res);
}

#[test]
fn t3() {
    let parse = "-1.01";
    let res = parser::ExprParser::new().parse(parse).unwrap();
    println!("{}, {:?}", parse, res);
}

#[test]
fn t4() {
    let parse = "1+1";
    let res = parser::ExprParser::new().parse(parse).unwrap();
    println!("{}, {:?}", parse, res);
}

#[test]
fn t5() {
    let parse = "1+1+1";
    let res = parser::ExprParser::new().parse(parse).unwrap();
    println!("{}, {:?}", parse, res);
}

#[test]
fn t6() {
    let parse = "1-1";
    let res = parser::ExprParser::new().parse(parse).unwrap();
    println!("{}, {:?}", parse, res);
}

#[test]
fn t7() {
    let parse = "1-1-1";
    let res = parser::ExprParser::new().parse(parse).unwrap();
    println!("{}, {:?}", parse, res);
}

#[test]
fn t8() {
    let parse = "1+1-1";
    let res = parser::ExprParser::new().parse(parse).unwrap();
    println!("{}, {:?}", parse, res);
}

#[test]
fn t9() {
    let parse = "1*1";
    let res = parser::ExprParser::new().parse(parse).unwrap();
    println!("{}, {:?}", parse, res);
}

#[test]
fn t10() {
    let parse = "1*1*1";
    let res = parser::ExprParser::new().parse(parse).unwrap();
    println!("{}, {:?}", parse, res);
}

#[test]
fn t11() {
    let parse = "1*1+1";
    let res = parser::ExprParser::new().parse(parse).unwrap();
    println!("{}, {:?}", parse, res);
}

#[test]
fn t12() {
    let parse = "1+1*1";
    let res = parser::ExprParser::new().parse(parse).unwrap();
    println!("{}, {:?}", parse, res);
}

#[test]
fn t13() {
    let parse = "---1";
    let res = parser::ExprParser::new().parse(parse).unwrap();
    println!("{}, {:?}", parse, res);
}

#[test]
fn t14() {
    let parse = "1+1+1-1*1*-1";
    let res = parser::ExprParser::new().parse(parse).unwrap();
    println!("{}, {:?}", parse, res);
}

#[test]
fn t15() {
    let parse = "(1)";
    let res = parser::ExprParser::new().parse(parse).unwrap();
    println!("{}, {:?}", parse, res);
}

#[test]
fn t16() {
    let parse = "1+(1)";
    let res = parser::ExprParser::new().parse(parse).unwrap();
    println!("{}, {:?}", parse, res);
}

#[test]
fn t17() {
    let parse = "1+(1+1)";
    let res = parser::ExprParser::new().parse(parse).unwrap();
    println!("{}, {:?}", parse, res);
}

#[test]
fn t18() {
    let parse = "-(1+1)";
    let res = parser::ExprParser::new().parse(parse).unwrap();
    println!("{}, {:?}", parse, res);
}
