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
    /// A failure in writing the message
    WritingFailure,
    /// A failure in listening the message
    ListeningFailure,
    /// A failure in open the socket
    SocketFailure,
    /// A failure in open the file
    FileOpenFailure,
    /// A failure in read the line in the file
    ReadLineFailure,
    /// A failure in lenght of arguments
    ArgsLenFailure,
}

impl CalculatorErrors {
    /// Prints the error message to the console
    pub fn get_message(&self) -> &str {
        match self {
            Self::InvalidOperation => return "ERROR \"Operacion invalida\"",
            Self::DivisionByZero => return "ERROR \"Division por cero\"",
            Self::JoinFailure => return "ERROR \"Fallo en el Join\"",
            Self::LockFailure => return "ERROR \"Fallo en el Lock\"",
            Self::ParseFailure => return "ERROR \"Fallo en el Parseo\"",
            Self::WritingFailure => return "ERROR \"Fallo la escritura\"",
            Self::ListeningFailure => return "ERROR \"Fallo la lectura\"",
            Self::SocketFailure => return "ERROR \"Fallo el Socket\"",
            Self::FileOpenFailure => return "ERROR \"Fallo la apertura del archivo\"",
            Self::ReadLineFailure => return "ERROR \"Fallo la lectura de la linea\"",
            Self::ArgsLenFailure => return "ERROR \"Fallo la cantidad de argumentos\"",
        }
    }
}
