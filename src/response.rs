use crate::errors::CalculatorErrors;
use std::{io::Write, net::TcpStream};

pub enum Response {
    Ok,
    Value(u8),
    Error(CalculatorErrors),
}

impl Response {
    pub fn send_response(&self, mut stream: &TcpStream) {
        let response = format!("{}\n", self.get_message());
        if stream.write_all(response.as_bytes()).is_err() {
            Self::Error(CalculatorErrors::WritingFailure).eprint();
        };

        if stream.flush().is_err() {
            Self::Error(CalculatorErrors::WritingFailure).eprint();
        }
    }

    pub fn eprint(&self) {
        eprintln!("{}\n", self.get_message())
    }

    fn get_message(&self) -> String {
        match self {
            Self::Ok => "OK".to_owned(),
            Self::Value(value) => format!("VALUE {}", value),
            Self::Error(e) => e.get_message().to_owned(),
        }
    }
}
