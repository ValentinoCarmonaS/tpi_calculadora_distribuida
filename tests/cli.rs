use std::fs::{remove_file, write};
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

        thread::sleep(Duration::from_millis(500));

        Ok(TestServer { process })
    }

    fn stop(mut self) {
        let _ = self.process.kill();
    }
}

fn run_client_with_input_file(
    addr: &str,
    input_path: &str,
) -> Result<(ExitStatus, String), &'static str> {
    let output = match Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("client")
        .arg("--")
        .arg(addr)
        .arg(input_path)
        .output()
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

fn run_multiple_clients_concurrent(
    addr: &str,
    inputs_paths: Vec<&str>,
) -> Result<Vec<(ExitStatus, String)>, &'static str> {
    let mut handles = vec![];
    for input_path in inputs_paths {
        let addr_owned = addr.to_string();
        let input_path_owned = input_path.to_string();

        let handle =
            thread::spawn(move || run_client_with_input_file(&addr_owned, &input_path_owned));
        handles.push(handle);
    }

    let mut results = vec![];
    for handle in handles {
        match handle.join() {
            Ok(result) => match result {
                Ok(result) => results.push(result),
                Err(e) => return Err(e),
            },
            Err(_) => return Err("Thread join failed"),
        }
    }

    Ok(results)
}

#[test]
fn test_one_client_a_file() {
    let expected = "31";
    let server = TestServer::start("127.0.0.1:8084").unwrap();
    let (status, stdout) =
        run_client_with_input_file("127.0.0.1:8084", "tests/data/a.txt").unwrap();

    server.stop();

    assert!(status.success(), "The program should have succeeded");
    assert!(
        stdout.contains(expected),
        "The stdout doesn't contain the expected value. Expected: '{}', Got: '{}'",
        expected,
        stdout
    );
}

#[test]
fn test_one_client_b_file() {
    let expected = "0";
    let server = TestServer::start("127.0.0.1:8081").unwrap();
    let (status, stdout) =
        run_client_with_input_file("127.0.0.1:8081", "tests/data/b.txt").unwrap();

    server.stop();

    assert!(status.success(), "The program should have succeeded");
    assert!(
        stdout.contains(expected),
        "The stdout doesn't contain the expected value. Expected: '{}', Got: '{}'",
        expected,
        stdout
    );
}

#[test]
fn test_one_client_c_file() {
    let expected = "0";
    let server = TestServer::start("127.0.0.1:8082").unwrap();
    let (status, stdout) =
        run_client_with_input_file("127.0.0.1:8082", "tests/data/c.txt").unwrap();

    server.stop();

    assert!(status.success(), "The program should have succeeded");
    assert!(
        stdout.contains(expected),
        "The stdout doesn't contain the expected value. Expected: '{}', Got: '{}'",
        expected,
        stdout
    );
}

#[test]
fn test_one_client_d_file() {
    let expected = "2";
    let server = TestServer::start("127.0.0.1:8083").unwrap();
    let (status, stdout) =
        run_client_with_input_file("127.0.0.1:8083", "tests/data/d.txt").unwrap();

    server.stop();

    assert!(status.success(), "The program should have succeeded");
    assert!(
        stdout.contains(expected),
        "The stdout doesn't contain the expected value. Expected: '{}', Got: '{}'",
        expected,
        stdout
    );
}

#[test]
fn test_multiple_clients_concurrent_simple() {
    let server = TestServer::start("127.0.0.1:8085").unwrap();

    let results = run_multiple_clients_concurrent(
        "127.0.0.1:8085",
        vec!["tests/data/a.txt", "tests/data/b.txt"],
    )
    .unwrap();

    server.stop();

    for (status, _) in &results {
        assert!(status.success(), "All clients should have succeeded");
    }
}

#[test]
fn test_arithmetic_overflow_underflow() {
    let server = TestServer::start("127.0.0.1:8086").unwrap();

    write("tests/data/overflow_test.txt", "+ 255\n+ 1\n").unwrap();

    let (status, stdout) =
        run_client_with_input_file("127.0.0.1:8086", "tests/data/overflow_test.txt").unwrap();

    server.stop();
    let _ = remove_file("tests/data/overflow_test.txt");

    assert!(
        status.success(),
        "The program should handle overflow gracefully"
    );
    assert!(
        stdout.contains("0"), // 255 + 1 = 0
        "Should handle u8 overflow with wrapping. Got: '{}'",
        stdout
    );
}

#[test]
fn test_client_invalid_arguments_count() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("client")
        .arg("--")
        .arg("127.0.0.1:8087")
        .output()
        .unwrap();

    assert!(output.status.success(), "Should end successful");

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("ERROR \"invalid number of arguments\""),
        "Should show argument length error. Got: '{}'",
        stderr
    );
}

#[test]
fn test_client_invalid_server_address() {
    write("tests/data/temp_test.txt", "+ 1\n").unwrap();

    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("client")
        .arg("--")
        .arg("invalid_address:999999")
        .arg("tests/data/temp_test.txt")
        .output()
        .expect("Failed to execute command");

    let _ = remove_file("tests/data/temp_test.txt");

    assert!(output.status.success(), "Should end successful");

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("ERROR \"socket failure\""),
        "Should show socket connection error. Got: '{}'",
        stderr
    );
}

#[test]
fn test_client_nonexistent_file() {
    let server = TestServer::start("127.0.0.1:8088").unwrap();

    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("client")
        .arg("--")
        .arg("127.0.0.1:8088")
        .arg("tests/data/e.txt")
        .output()
        .expect("Failed to execute command");

    server.stop();

    assert!(output.status.success(), "Should end successful");

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("ERROR \"file open failure\""),
        "Should show file error. Got: '{}'",
        stderr
    );
}

#[test]
fn test_server_invalid_arguments() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("server")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Should end successful");

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("ERROR \"invalid number of arguments\""),
        "Should show argument error. Got: '{}'",
        stderr
    );
}

#[test]
fn test_server_invalid_bind_address() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("server")
        .arg("--")
        .arg("invalid_address:999999")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Should end successful");

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("ERROR \"socket failure\""),
        "Should show socket binding error. Got: '{}'",
        stderr
    );
}

#[test]
fn test_division_by_zero_error_handling() {
    let server = TestServer::start("127.0.0.1:8089").unwrap();

    write("tests/data/div_zero_test.txt", "+ 10\n/ 0\n").unwrap();

    let (status, stdout) =
        run_client_with_input_file("127.0.0.1:8089", "tests/data/div_zero_test.txt").unwrap();

    let _ = remove_file("tests/data/div_zero_test.txt");
    server.stop();

    assert!(
        status.success(),
        "Client should complete despite division by zero"
    );
    assert!(
        stdout.contains("10"),
        "Value should remain unchanged after division by zero error. Got: '{}'",
        stdout
    );
}

#[test]
fn test_invalid_operations_in_file() {
    let server = TestServer::start("127.0.0.1:8090").unwrap();

    write(
        "tests/data/invalid_ops_test.txt",
        "+ 5\n% 3\n* 2\nINVALID COMMAND\n- 1\n",
    )
    .unwrap();

    let (status, stdout) =
        run_client_with_input_file("127.0.0.1:8090", "tests/data/invalid_ops_test.txt").unwrap();

    let _ = remove_file("tests/data/invalid_ops_test.txt");
    server.stop();

    assert!(
        status.success(),
        "Client should complete despite invalid operations"
    );
    assert!(
        stdout.contains("9"), // (5 * 2) - 1 = 9
        "Should process valid operations and ignore invalid ones. Got: '{}'",
        stdout
    );
}

#[test]
fn test_empty_file() {
    let server = TestServer::start("127.0.0.1:8091").unwrap();

    write("tests/data/empty_test.txt", "").unwrap();

    let (status, stdout) =
        run_client_with_input_file("127.0.0.1:8091", "tests/data/empty_test.txt").unwrap();

    let _ = remove_file("tests/data/empty_test.txt");
    server.stop();

    assert!(status.success(), "Client should handle empty file");
    assert!(
        stdout.contains("0"),
        "Should return initial calculator value (0). Got: '{}'",
        stdout
    );
}
