use crate::errors::CalculatorErrors;
use std::str::FromStr;

/// An enum representing the possible operations that can be applied to the calculator
#[derive(PartialEq, Eq, Debug)]
pub enum Operation {
    /// Adds the given value to the calculator's value
    Add(u8),
    /// Subtracts the given value from the calculator's value
    Sub(u8),
    /// Multiplies the calculator's value by the given value
    Mul(u8),
    /// Divides the calculator's value by the given value
    Div(u8),
    /// Gets the calculator's value
    Get,
}

impl FromStr for Operation {
    type Err = CalculatorErrors;

    /// Parses a string into an "Operation"
    ///
    /// # Arguments:
    ///
    /// The string should be in the format "<operator> <operand>",
    /// where <operator> is one of { "+", "-", "*", "/" }, and <operand> is a `u8` value
    ///
    /// # Errors:
    ///
    /// Returns "CalculatorErrors::ParseFailure" if the string is not in the correct format,
    /// or "CalculatorErrors::InvalidOperation" if the operator is not valid
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the string into tokens separated by whitespace.
        let tokens: Vec<&str> = s.split_whitespace().collect();

        match tokens[0] {
            "GET" => {
                if tokens.len() != 1 {
                    return Err(CalculatorErrors::ParseFailure);
                }
                Ok(Self::Get)
            },
            "OP" => {
                if tokens.len() != 3 {
                    return Err(CalculatorErrors::ParseFailure);
                }

                let operation = tokens[1];
                let operand = tokens[2];

                // Parse the operand into an u8.
                let operand: u8 = match operand.parse() {
                    Ok(operand) => operand,
                    Err(_) => return Err(CalculatorErrors::ParseFailure)
                };

                match operation {
                    "+" => Ok(Self::Add(operand)),
                    "-" => Ok(Self::Sub(operand)),
                    "*" => Ok(Self::Mul(operand)),
                    "/" => Ok(Self::Div(operand)),
                    _ => Err(CalculatorErrors::InvalidOperation)
                }
            },
            _ => Err(CalculatorErrors::ParseFailure)
        }

        
    }
}

#[test]
fn test_parse_add() {
    let op = Operation::from_str("+ 10").unwrap();
    assert_eq!(op, Operation::Add(10));
}

#[test]
fn test_parse_sub() {
    let op = Operation::from_str("- 10").unwrap();
    assert_eq!(op, Operation::Sub(10));
}

#[test]
fn test_parse_mul() {
    let op = Operation::from_str("* 10").unwrap();
    assert_eq!(op, Operation::Mul(10));
}

#[test]
fn test_parse_div() {
    let op = Operation::from_str("/ 10").unwrap();
    assert_eq!(op, Operation::Div(10));
}

#[test]
fn test_parse_add_border_case_0() {
    let op = Operation::from_str("+ 0").unwrap();
    assert_eq!(op, Operation::Add(0));
}

#[test]
fn test_parse_add_border_case_255() {
    let op = Operation::from_str("+ 255").unwrap();
    assert_eq!(op, Operation::Add(255));
}

#[test]
fn test_parse_invalid_operation() {
    match Operation::from_str("% 10") {
        Ok(_) => panic!("Should throw an error"),
        Err(e) => match e {
            CalculatorErrors::InvalidOperation => (),
            _ => panic!("Should throw InvalidOperation error"),
        },
    }
}

#[test]
fn test_parse_invalid_line() {
    match Operation::from_str("+ - 10") {
        Ok(_) => panic!("Should throw an error"),
        Err(e) => match e {
            CalculatorErrors::ParseFailure => (),
            _ => panic!("Should throw ParseFailure error"),
        },
    }
}

#[test]
fn test_parse_invalid_negative_operand_1() {
    match Operation::from_str("+ -1") {
        Ok(_) => panic!("Should throw an error"),
        Err(e) => match e {
            CalculatorErrors::ParseFailure => (),
            _ => panic!("Should throw ParseFailure error"),
        },
    }
}

#[test]
fn test_parse_invalid_operand_256() {
    match Operation::from_str("+ 256") {
        Ok(_) => panic!("Should throw an error"),
        Err(e) => match e {
            CalculatorErrors::ParseFailure => (),
            _ => panic!("Should throw ParseFailure error"),
        },
    }
}
