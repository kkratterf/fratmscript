//! `--json` output and stdin input, for programs and AI agents (needs Node.js).

use std::io::Write;
use std::process::{Command, Stdio};

/// Runs `fratm <args>` with `stdin`, returning the parsed stdout and exit code.
fn fratm(args: &[&str], stdin: &str) -> (serde_json::Value, Option<i32>) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_fratm"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("fratm runs");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(stdin.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value = serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("{}: {}", e, stdout));
    (value, output.status.code())
}

#[test]
fn lancia_json_captures_the_output() {
    let (value, code) = fratm(&["lancia", "--json", "-"], "stamm a dì(1 + 1)\n");
    assert_eq!(
        value,
        serde_json::json!({
            "ok": true,
            "errors": [],
            "stdout": "2\n",
            "stderr": "",
            "exitCode": 0,
            "runtimeError": null,
        })
    );
    assert_eq!(code, Some(0));
}

#[test]
fn lancia_json_reports_compile_errors_with_a_hint() {
    let (value, code) = fratm(&["lancia", "--json", "-"], "const x = 1\n");
    assert_eq!(value["ok"], false);
    assert_eq!(value["errors"][0]["line"], 1);
    assert_eq!(value["errors"][0]["column"], 7);
    assert_eq!(
        value["errors"][0]["suggestion"],
        "In FratmScript 'const' se scrive 'chist è'"
    );
    assert_eq!(code, Some(1));
}

#[test]
fn lancia_json_maps_runtime_errors_to_the_fratm_line() {
    let (value, _) = fratm(
        &["lancia", "--json", "-"],
        "stamm a dì(\"primma\")\nchist è x = 1\nx = 2\n",
    );
    assert_eq!(value["ok"], false);
    assert_eq!(value["stdout"], "primma\n");
    assert_eq!(value["runtimeError"]["line"], 3);
    let message = value["runtimeError"]["message"].as_str().unwrap();
    assert!(
        message.starts_with("Nun se pò cagnà na costante"),
        "{}",
        message
    );
}

#[test]
fn lancia_timeout_stops_endless_programs() {
    let (value, code) = fratm(
        &["lancia", "--json", "--timeout", "0.5", "-"],
        "mentre che (overo) { }\n",
    );
    assert_eq!(value["ok"], false);
    assert_eq!(value["timedOut"], true);
    assert_eq!(code, Some(1));
}

#[test]
fn build_json_returns_the_javascript() {
    let (value, code) = fratm(&["build", "--json", "-"], "chist è x = 42\n");
    assert_eq!(
        value,
        serde_json::json!({ "ok": true, "code": "const x = 42;\n", "errors": [] })
    );
    assert_eq!(code, Some(0));
}

#[test]
fn check_json_lists_every_file() {
    let (value, code) = fratm(&["check", "--json", "-"], "tien = 1\n");
    assert_eq!(value["ok"], false);
    assert_eq!(value["files"][0]["file"], "<stdin>");
    assert_eq!(value["files"][0]["errors"][0]["line"], 1);
    assert_eq!(code, Some(1));
}
