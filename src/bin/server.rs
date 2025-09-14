use tpi_calculadora_distribuida::{
    calculator::Calculator, errors::CalculatorErrors, operation::Operation, response::Response,
};

use std::{
    env,
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
};

/// The entry point for the server application.
///
/// The server listens for incoming client connections and processes their requests.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Response::Error(CalculatorErrors::ArgsLenFailure).eprint();
    }

    server(&args[1]);
}

/// Starts the server and binds it to the specified address.
///
/// # Arguments:
///
/// * `addr` - The address to bind the server to.
fn server(addr: &str) {
    let listener = match TcpListener::bind(addr) {
        Ok(listener) => listener,
        Err(_) => {
            Response::Error(CalculatorErrors::SocketFailure).eprint();
            return;
        }
    };

    server_listening(listener);
}

/// Listens for incoming client connections and spawns a new thread for each connection.
///
/// # Arguments:
///
/// * `listener` - The TCP listener used to accept incoming connections.
fn server_listening(listener: TcpListener) {
    let counter = Arc::new(Mutex::new(Calculator::default()));
    let mut handles = vec![];

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let counter = Arc::clone(&counter);
                let handle = thread::spawn(move || {
                    handle_client(stream, counter);
                });
                handles.push(handle);
            }
            Err(_) => Response::Error(CalculatorErrors::ListeningFailure).eprint(),
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

/// Handles a single client connection.
///
/// # Arguments:
///
/// * `stream` - The TCP stream used to communicate with the client.
/// * `counter` - A shared reference to the calculator instance.
fn handle_client(stream: TcpStream, counter: Arc<Mutex<Calculator>>) {
    let reader = BufReader::new(&stream);

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => {
                Response::Error(CalculatorErrors::ReadLineFailure).send_response(&stream);
                continue;
            }
        };

        match handle_op(line, &counter) {
            Ok(ans) => match ans {
                Some(value_ans) => Response::Value(value_ans).send_response(&stream),
                None => Response::Ok.send_response(&stream),
            },
            Err(e) => Response::Error(e).send_response(&stream),
        };
    }
}

/// Processes an operation received from the client.
///
/// # Arguments:
///
/// * `line` - The operation received from the client as a string.
/// * `counter` - A shared reference to the calculator instance.
///
/// # Returns:
///
/// A `Response` indicating the result of the operation.
fn handle_op(
    line: String,
    counter: &Arc<Mutex<Calculator>>,
) -> Result<Option<u8>, CalculatorErrors> {
    let op = Operation::from_str(&line)?;

    let mut calculator = match counter.lock() {
        Ok(calculator) => calculator,
        Err(_) => return Err(CalculatorErrors::LockFailure),
    };

    calculator.apply(op)
}
