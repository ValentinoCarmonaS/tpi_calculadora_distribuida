use tpi_calculadora_distribuida::{calculator::Calculator, operation::Operation};

use std::str::FromStr;

#[test]
fn test_parse_and_apply_add() {
    let op = Operation::from_str("OP + 10").unwrap();
    let mut calculator = Calculator::default();
    calculator.apply(op).unwrap();
    assert_eq!(calculator.value(), 10);
}

#[test]
fn test_parse_and_apply_sub() {
    let op = Operation::from_str("OP - 10").unwrap();
    let mut calculator = Calculator::default();
    calculator.apply(op).unwrap();
    assert_eq!(calculator.value(), 246);
}

#[test]
fn test_parse_and_apply_mul() {
    let op = Operation::from_str("OP * 10").unwrap();
    let mut calculator = Calculator::default();
    calculator.apply(op).unwrap();
    assert_eq!(calculator.value(), 0);
}

#[test]
fn test_parse_and_apply_div() {
    let op = Operation::from_str("OP / 10").unwrap();
    let mut calculator = Calculator::default();
    calculator.apply(op).unwrap();
    assert_eq!(calculator.value(), 0);
}

#[test]
fn test_parse_and_apply_add_border_255() {
    let op = Operation::from_str("OP + 255").unwrap();
    let mut calculator = Calculator::default();
    calculator.apply(op).unwrap();
    assert_eq!(calculator.value(), 255);
}

#[test]
fn test_parse_and_apply_add_overflow() {
    let mut calculator = Calculator::default();
    calculator
        .apply(Operation::from_str("OP + 250").unwrap())
        .unwrap();
    calculator
        .apply(Operation::from_str("OP + 10").unwrap())
        .unwrap();
    assert_eq!(calculator.value(), 4);
}

#[test]
fn test_parse_and_apply_sub_underflow() {
    let op = Operation::from_str("OP - 1").unwrap();
    let mut calculator = Calculator::default();
    calculator.apply(op).unwrap();
    assert_eq!(calculator.value(), 255);
}

#[test]
fn test_parse_and_apply_mul_by_zero() {
    let mut calculator = Calculator::default();
    calculator
        .apply(Operation::from_str("OP + 200").unwrap())
        .unwrap();
    calculator
        .apply(Operation::from_str("OP * 0").unwrap())
        .unwrap();
    assert_eq!(calculator.value(), 0);
}

#[test]
fn test_parse_and_apply_mul_overflow() {
    let mut calculator = Calculator::default();
    calculator
        .apply(Operation::from_str("OP + 200").unwrap())
        .unwrap();
    calculator
        .apply(Operation::from_str("OP * 2").unwrap())
        .unwrap();
    assert_eq!(calculator.value(), 144);
}

#[test]
fn test_parse_and_apply_div_exact() {
    let mut calculator = Calculator::default();
    calculator
        .apply(Operation::from_str("OP + 100").unwrap())
        .unwrap();
    calculator
        .apply(Operation::from_str("OP / 25").unwrap())
        .unwrap();
    assert_eq!(calculator.value(), 4);
}

#[test]
fn test_parse_and_apply_div_round_down() {
    let mut calculator = Calculator::default();
    calculator
        .apply(Operation::from_str("OP + 7").unwrap())
        .unwrap();
    calculator
        .apply(Operation::from_str("OP / 2").unwrap())
        .unwrap();
    assert_eq!(calculator.value(), 3);
}

#[test]
fn test_parse_and_apply_combined() {
    let mut calculator = Calculator::default();
    calculator
        .apply(Operation::from_str("OP + 50").unwrap())
        .unwrap();
    calculator
        .apply(Operation::from_str("OP - 20").unwrap())
        .unwrap();
    calculator
        .apply(Operation::from_str("OP * 3").unwrap())
        .unwrap();
    calculator
        .apply(Operation::from_str("OP / 2").unwrap())
        .unwrap();
    assert_eq!(calculator.value(), 45);
}
