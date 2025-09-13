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
            Self::InvalidOperation => "ERROR \"Operacion invalida\"",
            Self::DivisionByZero => "ERROR \"Division por cero\"",
            Self::JoinFailure => "ERROR \"Fallo en el Join\"",
            Self::LockFailure => "ERROR \"Fallo en el Lock\"",
            Self::ParseFailure => "ERROR \"Fallo en el Parseo\"",
            Self::WritingFailure => "ERROR \"Fallo la escritura\"",
            Self::ListeningFailure => "ERROR \"Fallo la lectura\"",
            Self::SocketFailure => "ERROR \"Fallo el Socket\"",
            Self::FileOpenFailure => "ERROR \"Fallo la apertura del archivo\"",
            Self::ReadLineFailure => "ERROR \"Fallo la lectura de la linea\"",
            Self::ArgsLenFailure => "ERROR \"Fallo la cantidad de argumentos\"",
        }
    }
}
