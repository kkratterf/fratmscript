//! `fratm repl`: compiles each input and runs it in one long-lived Node.js
//! process, so variables, functions and classes stay defined between inputs.

use colored::*;
use fratm_core::compile;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

/// Marks the end of one input's output on the Node side's stdout.
const DONE: &str = "\u{0}fratm-done";

/// Runs each line it reads (JSON-encoded JavaScript) as a script in the same
/// global scope, so top-level `const`/`let`/`function` survive, then prints
/// `DONE`. Promises are awaited first so their output lands before it.
const RUNNER: &str = r#"
const vm = require('node:vm');
globalThis.require = require;
const rl = require('node:readline').createInterface({ input: process.stdin, terminal: false });
rl.on('line', async (line) => {
  try {
    const result = vm.runInThisContext(JSON.parse(line), { filename: 'repl.js' });
    if (result && typeof result.then === 'function') await result;
  } catch (e) {
    console.error(e && e.name && e.message ? `${e.name}: ${e.message}` : String(e));
  }
  process.stdout.write('\u0000fratm-done\n');
});
"#;

struct Node {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl Node {
    fn start() -> io::Result<Node> {
        let mut child = Command::new("node")
            .arg("-e")
            .arg(RUNNER)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;
        let stdin = child.stdin.take().expect("piped stdin");
        let stdout = BufReader::new(child.stdout.take().expect("piped stdout"));
        Ok(Node {
            child,
            stdin,
            stdout,
        })
    }

    /// Runs `code` and copies its output to our stdout until it is done.
    /// An error means the process is gone (for example the code called
    /// `process.exit`).
    fn run(&mut self, code: &str) -> io::Result<()> {
        let encoded = serde_json::to_string(code).map_err(io::Error::other)?;
        writeln!(self.stdin, "{encoded}")?;
        self.stdin.flush()?;
        let mut line = String::new();
        loop {
            line.clear();
            if self.stdout.read_line(&mut line)? == 0 {
                return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
            }
            if line.trim_end_matches(['\r', '\n']) == DONE {
                return Ok(());
            }
            print!("{line}");
            io::stdout().flush()?;
        }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

pub fn run_repl() {
    println!(
        "{}",
        "🤌 FratmScript REPL - Write JavaScript the way it should be"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        format!(
            "   Version {} - Type 'exit' to quit\n",
            fratm_core::version()
        )
        .dimmed()
    );
    let stdin = io::stdin();
    let mut node: Option<Node> = None;
    let mut accumulated = String::new();
    loop {
        let prompt = if accumulated.is_empty() {
            "fratm> "
        } else {
            "  ...> "
        };
        print!("{}", prompt.green());
        let _ = io::stdout().flush();
        let mut line = String::new();
        // 0 bytes is end of input (Ctrl-D, or the end of a piped file)
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => {
                println!();
                break;
            }
            Ok(_) => {}
        }
        let trimmed = line.trim();
        if trimmed == "esci" || trimmed == "exit" {
            println!("{}", "Goodbye! 👋".cyan());
            break;
        }
        if trimmed.is_empty() {
            continue;
        }
        accumulated.push_str(&line);
        match compile(&accumulated, Default::default()) {
            Ok(result) => {
                accumulated.clear();
                println!("{}", "─".repeat(40).dimmed());
                println!("{}", result.code.trim().blue());
                println!("{}", "─".repeat(40).dimmed());
                if node.is_none() {
                    match Node::start() {
                        Ok(started) => node = Some(started),
                        Err(e) => {
                            eprintln!(
                                "{} {}",
                                "✗".red().bold(),
                                format!("Nun trovo Node.js: {e}").red()
                            );
                            continue;
                        }
                    }
                }
                if let Some(running) = node.as_mut() {
                    if running.run(&result.code).is_err() {
                        // The process ended: start a fresh one next time
                        node = None;
                        eprintln!(
                            "{}",
                            "Node.js s'è fermato: 'e variabile so' state perse.".yellow()
                        );
                    }
                }
            }
            Err(e) => {
                let msg = format!("{}", e);
                // A missing ')' or '}' means the input continues on the next line
                if !msg.contains("'}'") && !msg.contains("')'") {
                    println!("{} {}", "✗".red().bold(), msg.red());
                    accumulated.clear();
                }
            }
        }
    }
}
