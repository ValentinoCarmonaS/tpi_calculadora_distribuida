use crate::errors::CalculatorErrors;
use std::{io::Write, net::TcpStream};

/// An enum representing the possible responses that the server can send to a client.
///
/// Each variant corresponds to a specific type of response, such as a success message,
/// the current value of the calculator, or an error message.
pub enum Response {
    /// Indicates that the operation was successful.
    Ok,
    /// Returns the current value of the calculator.
    Value(u8),
    /// Indicates that an error occurred, with the associated error details.
    Error(CalculatorErrors),
}

impl Response {
    /// Sends the response to the client over the given TCP stream.
    ///
    /// # Arguments:
    ///
    /// * `stream` - A mutable reference to the TCP stream used to communicate with the client.
    pub fn send_response(&self, mut stream: &TcpStream) {
        let response = format!("{}\n", self.get_message());
        if stream.write_all(response.as_bytes()).is_err() {
            Self::Error(CalculatorErrors::WritingFailure).eprint();
        };

        if stream.flush().is_err() {
            Self::Error(CalculatorErrors::WritingFailure).eprint();
        }
    }

    /// Prints the error message to the standard error output.
    pub fn eprint(&self) {
        eprintln!("{}\n", self.get_message())
    }

    /// Returns the response message as a string.
    ///
    /// # Returns:
    ///
    /// A string representation of the response message, formatted for display.
    fn get_message(&self) -> String {
        match self {
            Self::Ok => "OK".to_owned(),
            Self::Value(value) => format!("VALUE {}", value),
            Self::Error(e) => e.get_message().to_owned(),
        }
    }
}
