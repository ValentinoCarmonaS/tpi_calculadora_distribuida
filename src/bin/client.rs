use tpi_calculadora_distribuida::{errors::CalculatorErrors, response::Response};

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return Response::Error(CalculatorErrors::ArgsLenFailure).eprint();
    }

    let stream = match TcpStream::connect(&args[1]) {
        Ok(stream) => stream,
        Err(_) => return Response::Error(CalculatorErrors::SocketFailure).eprint(),
    };

    read_file(&args[2], stream);
}

fn read_file(path: &str, mut stream: TcpStream) {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Response::Error(CalculatorErrors::FileOpenFailure).eprint(),
    };

    let reader_file = BufReader::new(file);

    for _line in reader_file.lines() {
        let line = match _line {
            Ok(line) => line,
            Err(_) => {
                Response::Error(CalculatorErrors::ReadLineFailure).eprint();
                continue;
            }
        };

        send_request(&mut stream, line);
        read_response(&mut stream); // <------------ solo deve leer al final, sacar.
    }

    send_request(&mut stream, "GET".to_owned());
    read_response(&mut stream);
}

fn send_request(stream: &mut TcpStream, data: String) {
    if let Err(_) = stream.write_all(format!("{}\n", data).as_bytes()) {
            return Response::Error(CalculatorErrors::WritingFailure).eprint();
        };

        if let Err(_) = stream.flush() {
            return Response::Error(CalculatorErrors::WritingFailure).eprint();
        };
}

fn read_response(stream: &mut TcpStream) {
    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    
    if let Err(_) = reader.read_line(&mut response) {
        return Response::Error(CalculatorErrors::ListeningFailure).eprint();
    };
    
    println!("{}", response)
}
