use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
};

use crate::calculator::Calculator;
use crate::errors::CalculatorErrors;
use crate::operation::Operation;

pub fn server(addr: &str) {
    let listener = match TcpListener::bind(addr) {
        Ok(listener) => listener,
        Err(_) => return, // Error irrecuperable.........................................................
    };

    let counter = Arc::new(Mutex::new(Calculator::default()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let counter = Arc::clone(&counter);
                thread::spawn(move || {
                    handle_client(stream, counter);
                });
            }
            Err(_) => return, // Error (no se si es irrecuperable o no).....................................................
        }
    }
}

fn handle_op(line: String, counter: &Arc<Mutex<Calculator>>) -> Result<Option<u8>, CalculatorErrors> {
    let op = Operation::from_str(&line)?;
    let mut calculator = match counter.lock() {
        Ok(calculator) => calculator,
        Err(_) => return Err(CalculatorErrors::LockFailure)
    };
    Ok(calculator.apply(op)?)
}

fn handle_client(stream: TcpStream, counter: Arc<Mutex<Calculator>>) {
    // let op = read_operation(&stream)?;
    // let mut calculator = match counter.lock() {
    //     Ok(calculator) => calculator,
    //     Err(_) => return Err(CalculatorErrors::LockFailure)
    // };
    // return calculator.apply(op);

    let reader = BufReader::new(stream);

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => {
                ;
                continue;
            } // Error en el parseo de la linea, ( irrecuperable para la linea?????????? )....................................................................
        };

        match handle_op(line, &counter) {
            Ok(ans) => match ans {
                Some(value_ans) => 
            },
            Err(e) => ,
        };
    }
}
