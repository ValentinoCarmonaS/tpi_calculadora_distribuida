#[path = "./../response.rs"]
mod response;
#[path = "./../errors.rs"]
mod errors;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

use crate::errors::CalculatorErrors;
use crate::response::Response;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return Response::Error(CalculatorErrors::ArgsLenFailure).eprint();
    }

    let mut stream = TcpStream::connect(&args[1]).unwrap();

    let file = match File::open(&args[2]) {
        Ok(file) => file,
        Err(_) => return Response::Error(CalculatorErrors::FileOpenFailure).eprint(),
    };

    let reader = BufReader::new(file);

    for _line in reader.lines() {
        let line = match _line {
            Ok(line) => line,
            Err(_) => {
                Response::Error(CalculatorErrors::ReadLineFailure).eprint();
                continue;
            }
        };

        match stream.write_all(line.as_bytes()) {
            Ok(_) => (),
            Err(_) => Response::Error(CalculatorErrors::WritingFailure).eprint(),
        };
    }
}
