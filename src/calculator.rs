use crate::{errors::CalculatorErrors, operation::Operation};

/// A basic calculator that operates on an 8-bit unsigned integer (u8).
///
/// The calculator supports basic arithmetic operations (addition, subtraction, multiplication, division)
/// and maintains a single value. Operations that exceed the range of u8 (0-255) will wrap around.
#[derive(Default, Debug)]
pub struct Calculator {
    value: u8,
}

impl Calculator {
    /// Returns the current value of the calculator.
    ///
    /// # Returns:
    ///
    /// The current value as a `u8`.
    pub fn value(&self) -> u8 {
        self.value
    }

    /// Applies an operation to the calculator.
    ///
    /// # Arguments:
    ///
    /// * `op` - The operation to apply, represented as an `Operation` enum.
    ///
    /// # Errors:
    ///
    /// Returns `CalculatorErrors::DivisionByZero` if a division by zero is attempted.
    ///
    /// # Returns:
    ///
    /// - `Ok(Some(u8))` if the operation is `Get`.
    /// - `Ok(None)` for other operations.
    /// - `Err(CalculatorErrors)` if an error occurs.
    pub fn apply(&mut self, op: Operation) -> Result<Option<u8>, CalculatorErrors> {
        match op {
            Operation::Add(operand) => self.value = self.value.wrapping_add(operand),
            Operation::Sub(operand) => self.value = self.value.wrapping_sub(operand),
            Operation::Mul(operand) => self.value = self.value.wrapping_mul(operand),
            Operation::Div(operand) => {
                if operand == 0 {
                    return Err(CalculatorErrors::DivisionByZero);
                }
                self.value = self.value.wrapping_div(operand)
            }
            Operation::Get => return Ok(Some(self.value)),
        }
        Ok(None)
    }
}

#[test]
fn test_create_calculator() {
    let calculator = Calculator::default();
    assert_eq!(calculator.value(), 0);
}

#[test]
fn test_apply_add() {
    let mut calculator = Calculator::default();
    calculator.apply(Operation::Add(10)).unwrap();
    assert_eq!(calculator.value(), 10);
}

#[test]
fn test_apply_sub_10_current_value_10() {
    let mut calculator = Calculator::default();
    calculator.apply(Operation::Add(10)).unwrap();
    calculator.apply(Operation::Sub(10)).unwrap();
    assert_eq!(calculator.value(), 0);
}

#[test]
fn test_apply_sub_0_current_value_0() {
    let mut calculator = Calculator::default();
    calculator.apply(Operation::Sub(0)).unwrap();
    assert_eq!(calculator.value(), 0);
}

#[test]
fn test_apply_sub_0_current_value_10() {
    let mut calculator = Calculator::default();
    calculator.apply(Operation::Add(10)).unwrap();
    calculator.apply(Operation::Sub(0)).unwrap();
    assert_eq!(calculator.value(), 10);
}

#[test]
fn test_apply_sub_10_current_value_0() {
    let mut calculator = Calculator::default();
    calculator.apply(Operation::Sub(10)).unwrap();
    assert_eq!(calculator.value(), 246);
}

#[test]
fn test_apply_mul() {
    let mut calculator = Calculator::default();
    calculator.apply(Operation::Add(10)).unwrap();
    calculator.apply(Operation::Mul(2)).unwrap();
    assert_eq!(calculator.value(), 20);
}

#[test]
fn test_apply_mul_1() {
    let mut calculator = Calculator::default();
    calculator.apply(Operation::Add(10)).unwrap();
    calculator.apply(Operation::Mul(1)).unwrap();
    assert_eq!(calculator.value(), 10);
}

#[test]
fn test_apply_mul_0() {
    let mut calculator = Calculator::default();
    calculator.apply(Operation::Add(10)).unwrap();
    calculator.apply(Operation::Mul(0)).unwrap();
    assert_eq!(calculator.value(), 0);
}

#[test]
fn test_apply_div() {
    let mut calculator = Calculator::default();
    calculator.apply(Operation::Add(10)).unwrap();
    calculator.apply(Operation::Div(2)).unwrap();
    assert_eq!(calculator.value(), 5);
}

#[test]
fn test_apply_div_division_by_zero() {
    let mut calculator = Calculator::default();
    match calculator.apply(Operation::Div(0)) {
        Ok(_) => panic!("Should throw an error"),
        Err(e) => match e {
            CalculatorErrors::DivisionByZero => (),
            _ => panic!("Should throw DivisionByZero error"),
        },
    }
}

#[test]
fn test_apply_div_current_value_10_divisor_1() {
    let mut calculator = Calculator::default();
    calculator.apply(Operation::Add(10)).unwrap();
    calculator.apply(Operation::Div(1)).unwrap();
    assert_eq!(calculator.value(), 10);
}

#[test]
fn test_apply_div_current_value_0_divisor_10() {
    let mut calculator = Calculator::default();
    calculator.apply(Operation::Div(10)).unwrap();
    assert_eq!(calculator.value(), 0);
}

#[test]
fn test_apply_div_current_value_10_divisor_10() {
    let mut calculator = Calculator::default();
    calculator.apply(Operation::Add(10)).unwrap();
    calculator.apply(Operation::Div(10)).unwrap();
    assert_eq!(calculator.value(), 1);
}

#[test]
fn test_combined_operations() {
    let mut calculator = Calculator::default();
    calculator.apply(Operation::Add(10)).unwrap();
    calculator.apply(Operation::Add(5)).unwrap();
    calculator.apply(Operation::Sub(3)).unwrap();
    calculator.apply(Operation::Mul(2)).unwrap();
    calculator.apply(Operation::Div(4)).unwrap();
    assert_eq!(calculator.value(), 6)
}
