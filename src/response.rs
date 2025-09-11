use crate::errors::CalculatorErrors;
use std::{io::Write, net::TcpStream};

pub enum Response {
    Ok,
    Value(u8),
    Error(CalculatorErrors),
}

impl Response {
    pub fn send_response(&self, mut stream: &TcpStream) {
        match stream.write_all(self.get_message().as_bytes()) {
            Ok(_) => return,
            Err(_) => return,
        }; // Es asi ????????????????????????????????????????????
    }

    pub fn eprint(&self) {
        eprintln!("{}", self.get_message())
    }

    fn get_message(&self) -> String {
        match self {
            Self::Ok => return "OK".to_owned(),
            Self::Value(value) => return format!("VALUE {}", value),
            Self::Error(e) => return e.get_message().to_owned(),
        }
    }
}
