//! FratmScript CLI - JavaScript, but the way it should be 🤌

use clap::{Parser as ClapParser, Subcommand};
use colored::*;
use esegui::Outcome;
use fratm_core::{compile, errors, CompileOptions};
use std::fs;
use std::io::{self, BufRead, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

mod esegui;
mod mcp;
mod nuovo;

#[derive(ClapParser)]
#[command(name = "fratm")]
#[command(author = "Federico")]
#[command(version = fratm_core::version())]
#[command(about = "🤌 FratmScript - JavaScript, but the way it should be", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile and launch a .fratm file (`-` reads it from stdin)
    #[command(name = "lancia", alias = "run")]
    Lancia {
        file: PathBuf,
        #[arg(long)]
        sourcemap: bool,
        /// Print one JSON object: errors, captured stdout/stderr, runtime error
        #[arg(long)]
        json: bool,
        /// Stop the program after this many seconds
        #[arg(long, value_name = "SECONDS")]
        timeout: Option<f64>,
    },
    /// Compile a .fratm file to JavaScript (`-` reads it from stdin)
    Build {
        file: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long)]
        sourcemap: bool,
        /// Print the JavaScript instead of writing a file
        #[arg(long)]
        stdout: bool,
        /// Print one JSON object with the JavaScript or the errors
        #[arg(long)]
        json: bool,
    },
    /// Check .fratm files for errors without writing any output
    Check {
        #[arg(required = true)]
        files: Vec<PathBuf>,
        /// Print one JSON object with the errors of every file
        #[arg(long)]
        json: bool,
    },
    /// Print the FratmScript language reference for AI agents
    #[command(name = "guida", alias = "reference")]
    Guida,
    /// Run a Model Context Protocol server on stdio, for AI agents
    Mcp,
    /// Format .fratm files in place
    Fmt {
        #[arg(required = true)]
        files: Vec<PathBuf>,
        /// Only report files that are not formatted (non-zero exit if any)
        #[arg(long)]
        check: bool,
    },
    /// Create a new project (a React app with Vite; --completa adds an API and a database, --senza-react or --semplice for less)
    #[command(name = "nuovo", alias = "new")]
    Nuovo {
        /// Folder to create
        dir: PathBuf,
        /// A Vite app with plain DOM code instead of React
        #[arg(long, conflicts_with_all = ["semplice", "completa"])]
        senza_react: bool,
        /// A full-stack app: React, a Hono API and a SQLite database (Node 22.13+)
        #[arg(long, conflicts_with = "semplice")]
        completa: bool,
        /// Only a main.fratm to run with `fratm lancia`, no Vite
        #[arg(long)]
        semplice: bool,
    },
    /// Interactive REPL
    Repl,
    /// Show tokens (debug)
    Tokens { file: PathBuf },
    /// Show AST (debug)
    Ast { file: PathBuf },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Lancia {
            file,
            sourcemap,
            json,
            timeout,
        } => run_file(&file, sourcemap, json, timeout),
        Commands::Build {
            file,
            output,
            sourcemap,
            stdout,
            json,
        } => build_file(&file, output, sourcemap, stdout, json),
        Commands::Check { files, json } => check_files(&files, json),
        Commands::Guida => print!("{}", GUIDA),
        Commands::Mcp => mcp::serve(),
        Commands::Fmt { files, check } => format_files(&files, check),
        Commands::Nuovo {
            dir,
            senza_react,
            completa,
            semplice,
        } => new_project(&dir, senza_react, completa, semplice),
        Commands::Repl => run_repl(),
        Commands::Tokens { file } => show_tokens(&file),
        Commands::Ast { file } => show_ast(&file),
    }
}

/// The language reference for AI agents, also served as `llms-full.txt`.
const GUIDA: &str = include_str!("guida.md");

/// Name shown for a program read from stdin.
const STDIN_NAME: &str = "<stdin>";

/// Reads a program from `path`, or from stdin when `path` is `-`.
fn read_program(path: &Path) -> io::Result<String> {
    if path.as_os_str() == "-" {
        let mut source = String::new();
        io::stdin().read_to_string(&mut source)?;
        Ok(source)
    } else {
        fs::read_to_string(path)
    }
}

/// How a path is shown in messages and JSON.
fn display_name(path: &Path) -> String {
    if path.as_os_str() == "-" {
        STDIN_NAME.to_string()
    } else {
        path.display().to_string()
    }
}

/// Prints `value` as one line of JSON and exits with 1 unless it is `ok`.
fn print_json(value: serde_json::Value) -> ! {
    println!("{}", value);
    std::process::exit(if value["ok"] == true { 0 } else { 1 });
}

/// JSON for a file that cannot be read.
fn read_error_json(path: &Path, e: &io::Error) -> serde_json::Value {
    serde_json::json!({
        "message": format!("cannot read {}: {}", display_name(path), e),
        "line": null,
        "column": null,
        "suggestion": null,
    })
}

/// `--sourcemap` is kept for compatibility: `lancia` always maps runtime
/// errors back to the `.fratm` source now.
fn run_file(path: &Path, _sourcemap: bool, json: bool, timeout: Option<f64>) {
    let source = match read_program(path) {
        Ok(s) => s,
        Err(e) if json => print_json(serde_json::json!({
            "ok": false,
            "errors": [read_error_json(path, &e)],
            "stdout": "",
            "stderr": "",
            "exitCode": null,
            "runtimeError": null,
        })),
        Err(e) => {
            eprintln!("{} {}", "Error: file not found:".red().bold(), e);
            std::process::exit(1);
        }
    };
    let dir = match path.parent() {
        Some(dir) if path.as_os_str() != "-" && !dir.as_os_str().is_empty() => dir,
        _ => Path::new("."),
    };
    let outcome = esegui::run(
        &source,
        esegui::RunOptions {
            dir,
            capture: json,
            timeout: timeout.map(Duration::from_secs_f64),
        },
    );
    if json {
        print_json(esegui::outcome_json(&outcome, &source));
    }
    match outcome {
        Outcome::CompileError(e) => {
            print_error(&source, &e);
            std::process::exit(1);
        }
        Outcome::Failed(message) => {
            eprintln!("{} {}", "Error:".red().bold(), message);
            std::process::exit(1);
        }
        Outcome::Ran {
            exit_code,
            runtime_error,
            timed_out,
            ..
        } => {
            if let Some(e) = runtime_error {
                print_runtime_error(&source, &e);
                std::process::exit(1);
            }
            if timed_out {
                eprintln!(
                    "\n{} {}",
                    "✗".red().bold(),
                    "'O programma steva ancora jenno: l'aggio fermato.".red()
                );
                std::process::exit(124);
            }
            if exit_code != Some(0) {
                std::process::exit(exit_code.unwrap_or(1));
            }
        }
    }
}

/// Prints an uncaught runtime error in Neapolitan, at its `.fratm` line.
fn print_runtime_error(source: &str, error: &esegui::RuntimeError) {
    match (error.line, error.column) {
        (Some(line), Some(column)) => {
            eprintln!(
                "\n{} Riga {}, colonna {}: {}",
                "✗ Errore a runtime:".red().bold(),
                line,
                column,
                error.message
            );
            print_source_line(source, line, Some(column));
        }
        _ => eprintln!("\n{} {}", "✗ Errore a runtime:".red().bold(), error.message),
    }
    eprintln!("\n{}", errors::random_encouragement().dimmed());
}

fn build_file(path: &Path, output: Option<PathBuf>, sourcemap: bool, stdout: bool, json: bool) {
    let source = match read_program(path) {
        Ok(s) => s,
        Err(e) if json => print_json(serde_json::json!({
            "ok": false,
            "code": null,
            "errors": [read_error_json(path, &e)],
        })),
        Err(e) => {
            eprintln!("{} {}", "Error: file not found:".red().bold(), e);
            std::process::exit(1);
        }
    };

    let options = CompileOptions {
        source_map: sourcemap,
        filename: Some(display_name(path)),
        minify: false,
        jsx_import_source: None,
    };

    let result = match compile(&source, options) {
        Ok(result) => result,
        Err(e) if json => print_json(serde_json::json!({
            "ok": false,
            "code": null,
            "errors": [esegui::error_json(&e, &source)],
        })),
        Err(e) => {
            print_error(&source, &e);
            std::process::exit(1);
        }
    };
    // With --json or --stdout, a file is written only when -o asks for one
    let to_terminal = json || stdout || path.as_os_str() == "-";
    if to_terminal && output.is_none() {
        if json {
            print_json(serde_json::json!({ "ok": true, "code": result.code, "errors": [] }));
        }
        print!("{}", result.code);
        return;
    }
    let out_path = output.unwrap_or_else(|| path.with_extension("js"));
    let mut output_content = result.code.clone();
    if sourcemap {
        if let Some(sm) = &result.source_map {
            let map_path = out_path.with_extension("js.map");
            if let Err(e) = fs::write(&map_path, sm.to_json_pretty()) {
                eprintln!("{} {}", "Warning: cannot write source map:".yellow(), e);
            } else {
                output_content.push_str(&format!(
                    "\n//# sourceMappingURL={}",
                    map_path.file_name().unwrap().to_string_lossy()
                ));
                if !json {
                    println!("  {} {}", "Source map:".dimmed(), map_path.display());
                }
            }
        }
    }
    if let Err(e) = fs::write(&out_path, &output_content) {
        eprintln!("{} {}", "Error: cannot write file:".red().bold(), e);
        std::process::exit(1);
    }
    if json {
        print_json(serde_json::json!({ "ok": true, "code": result.code, "errors": [] }));
    }
    println!(
        "{} {} → {}",
        errors::success_message().green().bold(),
        path.display(),
        out_path.display()
    );
}

fn check_files(paths: &[PathBuf], json: bool) {
    if json {
        let files: Vec<serde_json::Value> = paths
            .iter()
            .map(|path| {
                let errors = match read_program(path) {
                    Ok(source) => match compile(&source, Default::default()) {
                        Ok(_) => vec![],
                        Err(e) => vec![esegui::error_json(&e, &source)],
                    },
                    Err(e) => vec![read_error_json(path, &e)],
                };
                serde_json::json!({
                    "file": display_name(path),
                    "ok": errors.is_empty(),
                    "errors": errors,
                })
            })
            .collect();
        let ok = files.iter().all(|f| f["ok"] == true);
        print_json(serde_json::json!({ "ok": ok, "files": files }));
    }
    let mut failed = 0;
    for path in paths {
        let source = match read_program(path) {
            Ok(source) => source,
            Err(e) => {
                eprintln!(
                    "{} {}: {}",
                    "Error: cannot read file".red().bold(),
                    display_name(path),
                    e
                );
                failed += 1;
                continue;
            }
        };
        if let Err(e) = compile(&source, Default::default()) {
            eprintln!("{}", display_name(path).bold());
            print_error(&source, &e);
            failed += 1;
        }
    }
    if failed > 0 {
        eprintln!(
            "{}",
            format!("✗ {} of {} files have errors", failed, paths.len())
                .red()
                .bold()
        );
        std::process::exit(1);
    }
    println!(
        "{} {} file(s) checked",
        errors::success_message().green().bold(),
        paths.len()
    );
}

fn new_project(dir: &Path, senza_react: bool, completa: bool, semplice: bool) {
    let template = if semplice {
        nuovo::Template::Simple
    } else if completa {
        nuovo::Template::Full
    } else if senza_react {
        nuovo::Template::Vite
    } else {
        nuovo::Template::React
    };
    let files = match nuovo::create(dir, template, fratm_core::version()) {
        Ok(files) => files,
        Err(e) => {
            eprintln!("{} {}", "✗".red().bold(), e);
            std::process::exit(1);
        }
    };
    for file in &files {
        println!("  {} {}", "+".green(), file.display());
    }
    println!(
        "\n{} {}",
        "🤌 Progetto pronto:".green().bold(),
        dir.display()
    );
    let cd = format!("cd {}", dir.display());
    if semplice {
        println!("\n  {}\n  fratm lancia main.fratm", cd);
    } else {
        println!("\n  {}\n  npm install\n  npm run dev", cd);
    }
}

fn format_files(paths: &[PathBuf], check: bool) {
    let mut failed = 0;
    let mut unformatted = 0;
    for path in paths {
        let Some(source) = read_source(path) else {
            failed += 1;
            continue;
        };
        let formatted = match fratm_core::format(&source) {
            Ok(formatted) => formatted,
            Err(e) => {
                eprintln!("{}", path.display().to_string().bold());
                print_error(&source, &e);
                failed += 1;
                continue;
            }
        };
        if formatted == source {
            continue;
        }
        unformatted += 1;
        if check {
            println!("{} {}", "Not formatted:".yellow(), path.display());
        } else if let Err(e) = fs::write(path, formatted) {
            eprintln!("{} {}", "Error: cannot write file:".red().bold(), e);
            failed += 1;
        } else {
            println!("{} {}", "Formatted:".green(), path.display());
        }
    }
    if failed > 0 || (check && unformatted > 0) {
        std::process::exit(1);
    }
    if unformatted == 0 {
        println!(
            "{} {} file(s) already formatted",
            errors::success_message().green().bold(),
            paths.len()
        );
    }
}

fn read_source(path: &PathBuf) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!(
                "{} {}: {}",
                "Error: cannot read file".red().bold(),
                path.display(),
                e
            );
            None
        }
    }
}

fn run_repl() {
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
    let mut accumulated = String::new();
    loop {
        let prompt = if accumulated.is_empty() {
            "fratm> "
        } else {
            "  ...> "
        };
        print!("{}", prompt.green());
        io::stdout().flush().unwrap();
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).is_err() {
            break;
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
                println!("{}", "─".repeat(40).dimmed());
                println!("{}", result.code.trim().blue());
                println!("{}", "─".repeat(40).dimmed());
                let temp_path = std::env::temp_dir().join("fratm_repl.js");
                if fs::write(&temp_path, &result.code).is_ok() {
                    if let Ok(output) = Command::new("node").arg(&temp_path).output() {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        if !stdout.is_empty() {
                            print!("{}", stdout);
                        }
                        if !stderr.is_empty() {
                            eprint!("{}", stderr.red());
                        }
                    }
                }
                accumulated.clear();
            }
            Err(e) => {
                let msg = format!("{}", e);
                if !msg.contains("'}'") && !msg.contains("')'") {
                    println!("{} {}", "✗".red().bold(), msg.red());
                    accumulated.clear();
                }
            }
        }
    }
}

fn show_tokens(path: &PathBuf) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} {}", "Error: file not found:".red().bold(), e);
            std::process::exit(1);
        }
    };
    let mut lexer = fratm_core::lexer::Lexer::new(&source);
    let tokens = lexer.tokenize();
    println!("{}", "Tokens:".cyan().bold());
    for token in tokens {
        println!(
            "  {:20} @ {}:{}",
            format!("{:?}", token.kind).yellow(),
            token.span.line.to_string().dimmed(),
            token.span.column.to_string().dimmed()
        );
    }
}

fn show_ast(path: &PathBuf) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} {}", "Error: file not found:".red().bold(), e);
            std::process::exit(1);
        }
    };
    let mut lexer = fratm_core::lexer::Lexer::new(&source);
    let tokens = lexer.tokenize();
    let mut parser = fratm_core::parser::Parser::new(tokens);
    match parser.parse() {
        Ok(program) => {
            println!("{}", "AST:".cyan().bold());
            println!(
                "{}",
                serde_json::to_string_pretty(&program).unwrap_or_default()
            );
        }
        Err(errors) => {
            for e in errors {
                println!("{} {}", "✗".red().bold(), e.message.red());
            }
        }
    }
}

fn print_error(source: &str, error: &fratm_core::errors::CompileError) {
    eprintln!("\n{} {}", "✗ Error:".red().bold(), error);
    if let Some(line_num) = error.line() {
        print_source_line(source, line_num, error.column());
    }
    if let Some(suggestion) = errors::suggest(error, source) {
        eprintln!("\n{}", suggestion.yellow());
    }
    eprintln!("\n{}", errors::random_encouragement().dimmed());
}

/// Prints line `line_num` of `source` with a caret under `column`.
fn print_source_line(source: &str, line_num: usize, column: Option<usize>) {
    let lines: Vec<&str> = source.lines().collect();
    if line_num == 0 || line_num > lines.len() {
        return;
    }
    eprintln!(
        "  {} │ {}",
        line_num.to_string().dimmed(),
        lines[line_num - 1]
    );
    if let Some(col) = column {
        let pointer = " ".repeat(col.saturating_sub(1)) + "^";
        eprintln!(
            "  {} │ {}",
            " ".repeat(line_num.to_string().len()),
            pointer.red()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The FratmScript skill of the agent plugin.
    const SKILL: &str = include_str!("../../../packages/agent-plugin/skills/fratmscript/SKILL.md");

    /// Every ```fratm block of a Markdown text, with its first line.
    fn fratm_blocks(text: &str) -> Vec<(usize, String)> {
        let mut blocks = Vec::new();
        let mut current: Option<(usize, String)> = None;
        for (i, line) in text.lines().enumerate() {
            match (&mut current, line.trim_end()) {
                (None, "```fratm") => current = Some((i + 2, String::new())),
                (Some(_), "```") => blocks.push(current.take().unwrap()),
                (Some((_, code)), _) => {
                    code.push_str(line);
                    code.push('\n');
                }
                _ => {}
            }
        }
        blocks
    }

    #[test]
    fn every_example_in_the_agent_reference_compiles() {
        let blocks = fratm_blocks(GUIDA);
        assert!(blocks.len() >= 10, "found {} blocks", blocks.len());
        for (line, code) in blocks {
            if let Err(e) = compile(&code, Default::default()) {
                panic!("guida.md block at line {}: {}", line, e);
            }
        }
    }

    #[test]
    fn every_example_in_the_agent_skill_compiles() {
        let blocks = fratm_blocks(SKILL);
        assert!(!blocks.is_empty());
        for (line, code) in blocks {
            if let Err(e) = compile(&code, Default::default()) {
                panic!("SKILL.md block at line {}: {}", line, e);
            }
        }
    }

    #[test]
    fn the_agent_reference_lists_every_dialect_keyword() {
        for keyword in [
            "chist è",
            "tien",
            "facc",
            "piglie",
            "sinnò",
            "vedimm",
            "mentre che",
            "primm fa",
            "dint",
            "'ncopp",
            "rompe",
            "salta",
            "overo",
            "sfòls",
            "nisciun",
            "boh",
            "stamm a dì",
            "mo vir",
            "aspett",
            "caccia",
            "pruvamm",
            "e si schiatta",
            "aropp",
            "iett",
            "nu bell",
            "na famiglie",
            "figlio 'e",
            "'o pate",
            "stu cos",
            "fisso",
            "damme",
            "miette",
            "chiamm",
            "mann for",
            "leva",
            "fermete",
            "costruttore",
        ] {
            assert!(
                GUIDA.contains(&format!("`{}", keyword)),
                "missing {}",
                keyword
            );
        }
    }
}
