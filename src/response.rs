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
        if let Err(_) = stream.write_all(response.as_bytes()) {
            Self::Error(CalculatorErrors::WritingFailure).eprint();
        };

        if let Err(_) = stream.flush() {
            Self::Error(CalculatorErrors::WritingFailure).eprint();
        }
    }

    pub fn eprint(&self) {
        eprintln!("{}\n", self.get_message())
    }

    fn get_message(&self) -> String {
        match self {
            Self::Ok => return "OK".to_owned(),
            Self::Value(value) => return format!("VALUE {}", value),
            Self::Error(e) => return e.get_message().to_owned(),
        }
    }
}
