//! `fratm repl` end to end (needs Node.js, like the golden tests).

use std::io::Write;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Feeds `input` to the REPL and returns its stdout; panics if the REPL does
/// not exit on its own once the input ends.
fn repl(input: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_fratm"))
        .arg("repl")
        .env("NO_COLOR", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("fratm runs");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let start = Instant::now();
    while child.try_wait().unwrap().is_none() {
        if start.elapsed() > Duration::from_secs(20) {
            let _ = child.kill();
            panic!("the REPL did not exit at the end of its input");
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn keeps_variables_and_functions_between_inputs() {
    let stdout = repl(
        "chist è a = 2\nstamm a dì(a * 21)\nfacc doppio(x) {\n    piglie x * 2\n}\nstamm a dì(doppio(a))\n",
    );
    assert!(stdout.contains("\n42\n"), "{}", stdout);
    assert!(stdout.contains("\n4\n"), "{}", stdout);
}

#[test]
fn keeps_going_after_a_runtime_error() {
    let stdout = repl("stamm a dì(nomme)\nstamm a dì(\"ancora vivo\")\n");
    assert!(stdout.contains("ancora vivo"), "{}", stdout);
}

#[test]
fn exits_at_end_of_input() {
    let stdout = repl("stamm a dì(1)\n");
    assert!(stdout.contains("\n1\n"), "{}", stdout);
}
