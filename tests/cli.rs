use std::process::{Command, ExitStatus};
use std::thread;
use std::time::Duration;

fn run_test<'a>(addr: &'a str, input_path: &'a str) -> Result<(ExitStatus, String), &'a str> {
    let mut server = match Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("server")
        .arg("--")
        .arg(addr)
        .spawn()
    {
        Ok(output) => output,
        Err(_) => return Err("Error in command execution"),
    };

    // Dar tiempo al servidor para inicializar y bindear el socket
    thread::sleep(Duration::from_millis(500));

    let (status, stdout) = match run_client_with_input_file(addr, input_path) {
        Ok(ans) => ans,
        Err(e) => {
            let _ = server.kill();
            return Err(e);
        },
    };

    // Dar tiempo al cliente para terminar completamente antes de matar el servidor
    thread::sleep(Duration::from_millis(100));
    
    let _ = server.kill();
    Ok((status, stdout))
}

fn run_client_with_input_file<'a>(addr: &'a str, input_path: &'a str) -> Result<(ExitStatus, String), &'a str> {
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

#[test]
fn test_one_client_e_file() {
    // ((0 + 1) * 3) + 2 = 5
    let expected = "VALUE 5";
    
    match run_test("127.0.0.1:8080", "data/e.txt") {
        Ok((status, stdout)) => {
            assert!(status.success(), "The program should have succeeded");
            assert!(
                stdout.contains(expected),
                "The stdout doesn't contain the expected value. Expected: '{}', Got: '{}'",
                expected,
                stdout
            );
        }
        Err(e) => panic!("{}", e),
    }
}