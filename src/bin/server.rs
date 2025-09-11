#[path = "./../calculator.rs"]
mod calculator;
#[path = "./../errors.rs"]
mod errors;
#[path = "./../operation.rs"]
mod operation;
#[path = "./../response.rs"]
mod response;

use std::{
    env, io::{BufRead, BufReader}, net::{TcpListener, TcpStream}, str::FromStr, sync::{Arc, Mutex}, thread
};

use crate::calculator::Calculator;
use crate::errors::CalculatorErrors;
use crate::operation::Operation;
use crate::response::Response;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Response::Error(CalculatorErrors::ArgsLenFailure).eprint();
    }

    server(&args[1]);
}

fn server(addr: &str) {
    let listener = match TcpListener::bind(addr) {
        Ok(listener) => listener,
        Err(_) => {
            Response::Error(CalculatorErrors::SocketFailure).eprint();
            return;
        } // Error irrecuperable ??????????????????????????????????????????
    };

    println!("Server alive and listening in port: {}", addr); //....................................................
    let counter = Arc::new(Mutex::new(Calculator::default()));
    let mut handles = vec![];

    for stream in listener.incoming() {
        println!("hearing something"); //....................................................

        match stream {
            Ok(stream) => {
                println!("hearing a stream"); //....................................................
                let counter = Arc::clone(&counter);
                let handle = thread::spawn(move || {
                    handle_client(stream, counter);
                });
                handles.push(handle);
            }
            Err(_) => Response::Error(CalculatorErrors::ListeningFailure).eprint(), // Error NO irrecuperable ??????????????????????????????????????????
        }
    }

    for handle in handles {
        match handle.join() {
            Ok(_) => (),
            Err(_) => {
                Response::Error(CalculatorErrors::JoinFailure).eprint();
                return;
            }
        }
    }
}

fn handle_op(
    line: String,
    counter: &Arc<Mutex<Calculator>>,
) -> Result<Option<u8>, CalculatorErrors> {
    let op = Operation::from_str(&line)?;

    let mut calculator = match counter.lock() {
        Ok(calculator) => calculator,
        Err(_) => return Err(CalculatorErrors::LockFailure),
    };

    Ok(calculator.apply(op)?)
}

fn handle_client(stream: TcpStream, counter: Arc<Mutex<Calculator>>) {
    let reader = BufReader::new(&stream);

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => {
                Response::Error(CalculatorErrors::ParseFailure);
                continue;
            } // Error en el parseo de la linea, ( irrecuperable para la linea?????????? ) ??????????????????????????????????????????
        };

        println!("hearing: {}", line); //....................................................

        match handle_op(line, &counter) {
            Ok(ans) => match ans {
                Some(value_ans) => Response::Value(value_ans).send_response(&stream),
                None => Response::Ok.send_response(&stream),
            },
            Err(e) => Response::Error(e).send_response(&stream),
        };
    }
}
