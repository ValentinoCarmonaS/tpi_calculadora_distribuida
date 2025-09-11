/// An enum representing the possible errors that can occur in the calculator
#[derive(Debug)]
pub enum CalculatorErrors {
    /// The operation is not valid
    InvalidOperation,
    /// A division by zero was attempted
    DivisionByZero,
    /// A thread failed to join
    JoinFailure,
    /// A mutex failed to lock
    LockFailure,
    /// A string failed to parse into an operation
    ParseFailure,
}

impl CalculatorErrors {
    /// Prints the error message to the console
    pub fn display(&self) {
        match self {
            Self::InvalidOperation => eprintln!("ERROR \"Operacion invalida\""),
            Self::DivisionByZero => eprintln!("ERROR \"Division por cero\""),
            Self::JoinFailure => eprintln!("ERROR \"Fallo en el Join\""),
            Self::LockFailure => eprintln!("ERROR \"Fallo en el Lock\""),
            Self::ParseFailure => eprintln!("ERROR \"Fallo en el Parseo\""),
        }
    }
}
