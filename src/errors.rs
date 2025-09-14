/// An enum representing the possible errors that can occur in the calculator
#[derive(Debug)]
pub enum CalculatorErrors {
    // Client errors:
    /// A division by zero was attempted
    ///
    /// client: OP / 0
    DivisionByZero,
    /// An invalid operation massage was recived
    ///
    /// client: OP multiplicar 0
    InvalidOperation(String),
    /// An invalid integer massage was recived
    ///
    /// client: OP + cinco
    InvalidInteger(String),
    /// An unexpected message was received
    ///
    /// client: OK
    UnexpectedMessage(String),

    // Server errors:
    /// A thread failed to join
    JoinFailure,
    /// A mutex failed to lock
    LockFailure,
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
    /// Returns the error message according to the protocol specification
    pub fn get_message(&self) -> String {
        match self {
            Self::DivisionByZero => "ERROR \"division by zero\"".to_owned(),
            Self::InvalidOperation(message) => {
                format!("ERROR \"parsing error: unknown operation: {}\"", message)
            }
            Self::InvalidInteger(message) => {
                format!("ERROR \"parsing error: invalid integer: {}\"", message)
            }
            Self::UnexpectedMessage(target) => format!("ERROR \"unexpected message: {}\"", target),

            Self::JoinFailure => "ERROR \"thread join failure\"".to_owned(),
            Self::LockFailure => "ERROR \"mutex lock failure\"".to_owned(),
            Self::WritingFailure => "ERROR \"writing failure\"".to_owned(),
            Self::ListeningFailure => "ERROR \"reading failure\"".to_owned(),
            Self::SocketFailure => "ERROR \"socket failure\"".to_owned(),
            Self::FileOpenFailure => "ERROR \"file open failure\"".to_owned(),
            Self::ReadLineFailure => "ERROR \"line reading failure\"".to_owned(),
            Self::ArgsLenFailure => "ERROR \"invalid number of arguments\"".to_owned(),
        }
    }
}
