#![allow(unused_imports)]
#![allow(dead_code)]

use crate::lexer::Lexer;
use crate::parser;
use crate::evaluator;
use crate::syntax::Ctx;


fn test(expr: &str) -> f64 {
    let mut ctx = Ctx::default();
    let e = parser::ExprParser::new().parse(expr, &mut ctx, Lexer::new(&expr)).unwrap();
    evaluator::evaluate(e, &mut ctx)
}

#[test]

fn t1() {
    assert_eq!(test("1"), 1.0);
}

#[test]
fn t2() {
    assert_eq!(test("1.01"), 1.01);
}

#[test]
fn t3() {
    assert_eq!(test("-1.01"), -1.01);
}

#[test]
fn t4() {
    assert_eq!(test("1+1"), 2.0);
}

#[test]
fn t5() {
    assert_eq!(test("1+1+1"), 3.0);
}

#[test]
fn t6() {
    assert_eq!(test("1-1"), 0.0);
}

#[test]
fn t7() {
    assert_eq!(test("1-1-1"), -1.0);
}

#[test]
fn t8() {
    assert_eq!(test("1+1-1"), 1.0);
}

#[test]
fn t9() {
    assert_eq!(test("1*1"), 1.0);
}

#[test]
fn t10() {
    assert_eq!(test("1*1*1"), 1.0);
}

#[test]
fn t11() {
    assert_eq!(test("1*1+1"), 2.0);
}

#[test]
fn t12() {
    assert_eq!(test("1+1*1"), 2.0);
}

#[test]
fn t13() {
    assert_eq!(test("---1"), -1.0);
}

#[test]
fn t14() {
    assert_eq!(test("1+1+1-1*1*-1"), 4.0);
}

#[test]
fn t15() {
    assert_eq!(test("(1)"), 1.0);
}

#[test]
fn t16() {
    assert_eq!(test("1+(1)"), 2.0);
}

#[test]
fn t17() {
    assert_eq!(test("1+(1+1)"), 3.0);
}

#[test]
fn t18() {
    assert_eq!(test("-(1+1)"), -2.0);
}
