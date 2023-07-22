#![allow(unused_imports)]
#![allow(dead_code)]

use crate::{parser, syntax::Expr::{*, self}};


fn test(expr: &str) -> Expr {
    parser::ExprParser::new().parse(expr).unwrap()
}

#[test]
fn t1() {
    assert_eq!(test("1"), Num(1.0));
}

#[test]
fn t2() {
    assert_eq!(test("1.01"), Num(1.01));
}

#[test]
fn t3() {
    assert_eq!(
        test("-1.01"), 
        Neg(Box::new(Num(1.01)))
    );
}

#[test]
fn t4() {
    assert_eq!(
        test("1+1"),
        Plus(Box::new(Num(1.0)), Box::new(Num(1.0)))
    );
}

#[test]
fn t5() {
    assert_eq!(
        test("1+1+1"),
        Plus(
            Box::new(
                Plus(Box::new(Num(1.0)), Box::new(Num(1.0)))
            ),
            Box::new(Num(1.0))
        )
    );
}

#[test]
fn t6() {
    assert_eq!(
        test("1-1"), Minus(Box::new(Num(1.0)), Box::new(Num(1.0)))
    );
}

#[test]
fn t7() {
    assert_eq!(
        test("1-1-1"),
        Minus(
            Box::new(
                Minus(Box::new(Num(1.0)), Box::new(Num(1.0)))
            ),
            Box::new(Num(1.0))
        )
    );
}

#[test]
fn t8() {
    assert_eq!(
        test("1+1-1"),
        Minus(
            Box::new(
                Plus(Box::new(Num(1.0)), Box::new(Num(1.0)))
            ),
            Box::new(Num(1.0))
        )
    );
}

#[test]
fn t9() {
    assert_eq!(
        test("1*1"), Times(Box::new(Num(1.0)), Box::new(Num(1.0)))
    );
}

#[test]
fn t10() {
    assert_eq!(
        test("1*1*1"),
        Times(
            Box::new(
                Times(Box::new(Num(1.0)), Box::new(Num(1.0)))
            ),
            Box::new(Num(1.0))
        )
    );
}

#[test]
fn t11() {
    assert_eq!(
        test("1*1+1"),
        Plus(
            Box::new(
                Times(Box::new(Num(1.0)), Box::new(Num(1.0)))
            ),
            Box::new(Num(1.0))
        )
    );
}

#[test]
fn t12() {
    assert_eq!(
        test("1+1*1"),
        Plus(
            Box::new(Num(1.0)),
            Box::new(
                Times(Box::new(Num(1.0)), Box::new(Num(1.0)))
            )
        )
    );
}

#[test]
fn t13() {
    assert_eq!(
        test("---1"), 
        Neg(Box::new(Neg(Box::new(Neg(Box::new(Num(1.0)))))))
    );
}

#[test]
fn t14() {
    assert_eq!(
        test("1+1+1-1*1*-1"),
        Minus(
            Box::new(
                Plus(
                    Box::new(Plus(Box::new(Num(1.0)), Box::new(Num(1.0)))), 
                    Box::new(Num(1.0))
                )
            ), 
            Box::new(
                Times(
                    Box::new(Times(Box::new(Num(1.0)), Box::new(Num(1.0)))), 
                    Box::new(Neg(Box::new(Num(1.0))))
                )
            )
        )
    )
}

#[test]
fn t15() {
    assert_eq!(test("(1)"), Num(1.0));
}

#[test]
fn t16() {
    assert_eq!(
        test("1+(1)"), Plus(Box::new(Num(1.0)), Box::new(Num(1.0)))
    );
}

#[test]
fn t17() {
    assert_eq!(
        test("1+(1+1)"),
        Plus(
            Box::new(Num(1.0)),
            Box::new(
                Plus(Box::new(Num(1.0)), Box::new(Num(1.0)))
            )
        )
    );
}

#[test]
fn t18() {
    assert_eq!(
        test("-(1+1)"),
        Neg(
            Box::new(Plus(Box::new(Num(1.0)), Box::new(Num(1.0))))
        )
    )
}
