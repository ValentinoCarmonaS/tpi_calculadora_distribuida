use std::process::{Child, Command, ExitStatus};
use std::thread;
use std::time::Duration;

struct TestServer {
    process: Child,
}

impl TestServer {
    fn start(addr: &str) -> Result<Self, &'static str> {
        let process = match Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg("server")
            .arg("--")
            .arg(addr)
            .spawn()
        {
            Ok(output) => output,
            Err(_) => return Err("Error starting server"),
        };

        // Dar tiempo al servidor para inicializar y bindear el socket
        thread::sleep(Duration::from_millis(500));

        Ok(TestServer { process })
    }

    fn stop(mut self) {
        let _ = self.process.kill();
    }
}

fn run_client_with_input_file(addr: &str, input_path: &str) -> Result<(ExitStatus, String), &'static str> {
    let output = match Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("client")
        .arg("--")
        .arg(addr)
        .arg(input_path)
        .output()  // <- .output() espera a que el proceso termine
    {
        Ok(output) => output,
        Err(_) => return Err("Error in command execution"),
    };

    let stdout = match String::from_utf8(output.stdout) {
        Ok(stdout) => stdout,
        Err(_) => return Err("Error in converting stdout to string"),
    };

    Ok((output.status, stdout))
}

fn run_multiple_clients_concurrent(addr: &str, inputs_paths: Vec<&str>) -> Result<Vec<(ExitStatus, String)>, &'static str> {
    let mut handles = vec![];
    for input_path in inputs_paths {
        let addr_owned = addr.to_string();
        let input_path_owned = input_path.to_string();

        let handle = thread::spawn(move || {
            run_client_with_input_file(&addr_owned, &input_path_owned)
        });
        handles.push(handle);
    }

    let mut results = vec![];
    for handle in handles {
        match handle.join() {
            Ok(result) => match result {
                Ok(result) => results.push(result),
                Err(e) => return Err(e),
            }
            Err(_) => return Err("Thread join failed"),
        }
    }
    
    Ok(results)
}

#[test]
fn test_one_client_e_file() {
    let expected = "VALUE 5";
    let server = TestServer::start("127.0.0.1:8080").expect("Failed to start server");
    let (status, stdout) = run_client_with_input_file("127.0.0.1:8080", "data/e.txt").unwrap();
    
    assert!(status.success(), "The program should have succeeded");
    assert!(
        stdout.contains(expected),
        "The stdout doesn't contain the expected value. Expected: '{}', Got: '{}'",
        expected,
        stdout
    );
    
    server.stop();
}
