//! Running compiled programs with Node.js, and the JSON shapes that
//! `--json` prints for programs and AI agents.

use fratm_core::errors::{self, CompileError};
use fratm_core::{compile, CompileOptions};
use serde_json::{json, Value};
use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

/// Loaded with `node -r` before the program: reports an uncaught error as
/// JSON in the file named by `FRATM_ERROR_FILE`, so `fratm lancia` can print
/// it in Neapolitan at the `.fratm` line (the stack is source-mapped).
const ERROR_HOOK: &str = r#"const fs = require('fs')
function report(e) {
  const isError = e !== null && typeof e === 'object'
  fs.writeFileSync(process.env.FRATM_ERROR_FILE, JSON.stringify({
    name: isError && e.name ? String(e.name) : 'Error',
    message: isError && 'message' in e ? String(e.message) : String(e),
    stack: isError && e.stack ? String(e.stack) : '',
  }))
  process.exit(1)
}
process.on('uncaughtException', report)
process.on('unhandledRejection', report)
"#;

/// Source map name of the program, as it appears in source-mapped stacks.
const SOURCE_NAME: &str = "input.fratm";

/// An uncaught error thrown while the program ran, in Neapolitan.
pub struct RuntimeError {
    pub message: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

/// What happened when a program was compiled and run.
pub enum Outcome {
    /// The program did not compile.
    CompileError(CompileError),
    /// Node.js could not be started, or the program could not be written.
    Failed(String),
    /// The program ran. Output is only captured when asked for.
    Ran {
        stdout: String,
        stderr: String,
        exit_code: Option<i32>,
        runtime_error: Option<RuntimeError>,
        timed_out: bool,
    },
}

/// How to run a program.
pub struct RunOptions<'a> {
    /// Folder the temporary `.js` file goes in, so relative imports work.
    pub dir: &'a Path,
    /// Capture stdout and stderr instead of sharing the terminal; the
    /// program then reads no stdin.
    pub capture: bool,
    /// Kill the program after this long.
    pub timeout: Option<Duration>,
}

/// Compiles `source` and runs it with Node.js.
pub fn run(source: &str, options: RunOptions) -> Outcome {
    let compile_options = CompileOptions {
        source_map: true,
        filename: Some(SOURCE_NAME.to_string()),
        minify: false,
        jsx_import_source: None,
    };
    let result = match compile(source, compile_options) {
        Ok(result) => result,
        Err(e) => return Outcome::CompileError(e),
    };

    let id = std::process::id();
    let temp_path = options.dir.join(format!(".fratm_temp_{}.js", id));
    let hook_path = std::env::temp_dir().join(format!("fratm_hook_{}.cjs", id));
    let error_path = std::env::temp_dir().join(format!("fratm_error_{}.json", id));
    let mut output = result.code;
    if let Some(sm) = &result.source_map {
        output.push('\n');
        output.push_str(&sm.to_data_url());
    }
    if let Err(e) = fs::write(&temp_path, &output).and_then(|_| fs::write(&hook_path, ERROR_HOOK)) {
        return Outcome::Failed(format!("cannot write file: {}", e));
    }
    let _ = fs::remove_file(&error_path);

    let mut command = Command::new("node");
    command
        .arg("--enable-source-maps")
        .arg("--require")
        .arg(&hook_path)
        .arg(&temp_path)
        .env("FRATM_ERROR_FILE", &error_path);
    if options.capture {
        command
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
    }
    let finished = command
        .spawn()
        .map_err(|e| format!("Node.js failed: {}", e))
        .and_then(|child| wait(child, options.timeout));

    let _ = fs::remove_file(&temp_path);
    let _ = fs::remove_file(&hook_path);
    let report = fs::read_to_string(&error_path).ok();
    let _ = fs::remove_file(&error_path);

    match finished {
        Ok((exit_code, stdout, stderr, timed_out)) => Outcome::Ran {
            stdout,
            stderr,
            exit_code,
            runtime_error: report.map(|r| runtime_error(&r)),
            timed_out,
        },
        Err(e) => Outcome::Failed(e),
    }
}

/// Waits for `child`, reading its piped output, and kills it at `timeout`.
fn wait(
    mut child: std::process::Child,
    timeout: Option<Duration>,
) -> Result<(Option<i32>, String, String, bool), String> {
    fn drain<R: Read + Send + 'static>(pipe: Option<R>) -> thread::JoinHandle<String> {
        thread::spawn(move || {
            let mut bytes = Vec::new();
            if let Some(mut pipe) = pipe {
                let _ = pipe.read_to_end(&mut bytes);
            }
            String::from_utf8_lossy(&bytes).into_owned()
        })
    }
    let stdout = drain(child.stdout.take());
    let stderr = drain(child.stderr.take());
    let start = Instant::now();
    let mut timed_out = false;
    let status = loop {
        match child.try_wait().map_err(|e| e.to_string())? {
            Some(status) => break status,
            None if timeout.is_some_and(|t| start.elapsed() >= t) => {
                let _ = child.kill();
                timed_out = true;
                break child.wait().map_err(|e| e.to_string())?;
            }
            None => thread::sleep(Duration::from_millis(10)),
        }
    };
    let stdout = stdout.join().unwrap_or_default();
    let stderr = stderr.join().unwrap_or_default();
    Ok((status.code(), stdout, stderr, timed_out))
}

/// Reads the report written by [`ERROR_HOOK`].
fn runtime_error(report: &str) -> RuntimeError {
    let value: Value = serde_json::from_str(report).unwrap_or_default();
    let field = |key: &str| value[key].as_str().unwrap_or_default().to_string();
    let message =
        fratm_core::runtime::napoletanize_runtime_error(&field("name"), &field("message"));
    let position = runtime_position(&field("stack"));
    RuntimeError {
        message,
        line: position.map(|p| p.0),
        column: position.map(|p| p.1),
    }
}

/// Line and column of the innermost stack frame inside the `.fratm` program.
pub fn runtime_position(stack: &str) -> Option<(usize, usize)> {
    stack.lines().find_map(|frame| {
        let at = frame.find(SOURCE_NAME)? + SOURCE_NAME.len();
        let mut parts = frame[at..].trim_start_matches(':').split(':');
        let line = parts.next()?.parse().ok()?;
        let column = parts.next()?.trim_end_matches(')').parse().ok()?;
        Some((line, column))
    })
}

/// The message of a compile error without the `Riga N, colonna M:` prefix.
pub fn error_message(error: &CompileError) -> String {
    match error {
        CompileError::LexerError { message, .. }
        | CompileError::ParseError { message, .. }
        | CompileError::SemanticError { message, .. } => errors::napoletanize_error(message),
        CompileError::CodeGenError { .. } => error.to_string(),
    }
}

/// A compile error as JSON: `{ message, line, column, suggestion }`.
pub fn error_json(error: &CompileError, source: &str) -> Value {
    let suggestion =
        errors::suggest(error, source).map(|s| s.trim_start_matches("💡").trim_start().to_string());
    json!({
        "message": error_message(error),
        "line": error.line(),
        "column": error.column(),
        "suggestion": suggestion,
    })
}

/// The result of a run as JSON, as printed by `fratm lancia --json`.
pub fn outcome_json(outcome: &Outcome, source: &str) -> Value {
    match outcome {
        Outcome::CompileError(e) => json!({
            "ok": false,
            "errors": [error_json(e, source)],
            "stdout": "",
            "stderr": "",
            "exitCode": null,
            "runtimeError": null,
        }),
        Outcome::Failed(message) => json!({
            "ok": false,
            "errors": [{ "message": message, "line": null, "column": null, "suggestion": null }],
            "stdout": "",
            "stderr": "",
            "exitCode": null,
            "runtimeError": null,
        }),
        Outcome::Ran {
            stdout,
            stderr,
            exit_code,
            runtime_error,
            timed_out,
        } => {
            let mut value = json!({
                "ok": runtime_error.is_none() && !timed_out && *exit_code == Some(0),
                "errors": [],
                "stdout": stdout,
                "stderr": stderr,
                "exitCode": exit_code,
                "runtimeError": runtime_error.as_ref().map(|e| json!({
                    "message": e.message,
                    "line": e.line,
                    "column": e.column,
                })),
            });
            if *timed_out {
                value["timedOut"] = json!(true);
            }
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_fratm_frame_in_a_mapped_stack() {
        let stack = "TypeError: x\n    at f (/tmp/input.fratm:4:20)\n    at Object.<anonymous> (/tmp/input.fratm:6:1)";
        assert_eq!(runtime_position(stack), Some((4, 20)));
        let top_level =
            "ReferenceError: y\n    at /home/a b/input.fratm:2:12\n    at node:internal";
        assert_eq!(runtime_position(top_level), Some((2, 12)));
        assert_eq!(
            runtime_position("Error: z\n    at node:internal/x:1:1"),
            None
        );
    }

    #[test]
    fn compile_errors_are_json_with_position_and_hint() {
        let source = "const x = 1";
        let error = compile(source, Default::default()).unwrap_err();
        assert_eq!(
            error_json(&error, source),
            json!({
                "message": "Ccà ce sta 'x' ma l'istruzione era già fernuta! Va' a capo o miette ';'.",
                "line": 1,
                "column": 7,
                "suggestion": "In FratmScript 'const' se scrive 'chist è'",
            })
        );
    }
}
