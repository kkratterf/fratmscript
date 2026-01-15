//! FratmScript CLI - JavaScript, but the way it should be 🤌

use clap::{Parser as ClapParser, Subcommand};
use colored::*;
use fratm_core::{compile, CompileOptions, errors};
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::process::Command;

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
    /// Compile and run a .fratm file
    Run {
        file: PathBuf,
        #[arg(long)]
        sourcemap: bool,
    },
    /// Compile a .fratm file to JavaScript
    Build {
        file: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long)]
        sourcemap: bool,
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
        Commands::Run { file, sourcemap } => run_file(&file, sourcemap),
        Commands::Build { file, output, sourcemap } => build_file(&file, output, sourcemap),
        Commands::Repl => run_repl(),
        Commands::Tokens { file } => show_tokens(&file),
        Commands::Ast { file } => show_ast(&file),
    }
}

fn run_file(path: &PathBuf, sourcemap: bool) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => { eprintln!("{} {}", "Error: file not found:".red().bold(), e); std::process::exit(1); }
    };

    let options = CompileOptions { source_map: sourcemap, filename: Some(path.display().to_string()), minify: false };

    match compile(&source, options) {
        Ok(result) => {
            let temp_path = std::env::temp_dir().join("fratm_temp.js");
            let mut output = result.code;
            if sourcemap { if let Some(sm) = &result.source_map { output.push_str("\n"); output.push_str(&sm.to_data_url()); } }
            if let Err(e) = fs::write(&temp_path, &output) { eprintln!("{} {}", "Error: cannot write file:".red().bold(), e); std::process::exit(1); }
            let cmd_output = Command::new("node").arg(&temp_path).output();
            match cmd_output {
                Ok(out) => {
                    io::stdout().write_all(&out.stdout).unwrap();
                    io::stderr().write_all(&out.stderr).unwrap();
                    if !out.status.success() { std::process::exit(out.status.code().unwrap_or(1)); }
                }
                Err(e) => { eprintln!("{} {}", "Error: Node.js failed:".red().bold(), e); std::process::exit(1); }
            }
        }
        Err(e) => { print_error(&source, &e); std::process::exit(1); }
    }
}

fn build_file(path: &PathBuf, output: Option<PathBuf>, sourcemap: bool) {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => { eprintln!("{} {}", "Error: file not found:".red().bold(), e); std::process::exit(1); }
    };

    let options = CompileOptions { source_map: sourcemap, filename: Some(path.display().to_string()), minify: false };

    match compile(&source, options) {
        Ok(result) => {
            let out_path = output.unwrap_or_else(|| { let mut p = path.clone(); p.set_extension("js"); p });
            let mut output_content = result.code;
            if sourcemap {
                if let Some(sm) = &result.source_map {
                    let map_path = out_path.with_extension("js.map");
                    if let Err(e) = fs::write(&map_path, sm.to_json_pretty()) {
                        eprintln!("{} {}", "Warning: cannot write source map:".yellow(), e);
                    } else {
                        output_content.push_str(&format!("\n//# sourceMappingURL={}", map_path.file_name().unwrap().to_string_lossy()));
                        println!("  {} {}", "Source map:".dimmed(), map_path.display());
                    }
                }
            }
            if let Err(e) = fs::write(&out_path, &output_content) { eprintln!("{} {}", "Error: cannot write file:".red().bold(), e); std::process::exit(1); }
            println!("{} {} → {}", errors::success_message().green().bold(), path.display(), out_path.display());
        }
        Err(e) => { print_error(&source, &e); std::process::exit(1); }
    }
}

fn run_repl() {
    println!("{}", "🤌 FratmScript REPL - Write JavaScript the way it should be".cyan().bold());
    println!("{}", format!("   Version {} - Type 'exit' to quit\n", fratm_core::version()).dimmed());
    let stdin = io::stdin();
    let mut accumulated = String::new();
    loop {
        let prompt = if accumulated.is_empty() { "fratm> " } else { "  ...> " };
        print!("{}", prompt.green());
        io::stdout().flush().unwrap();
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).is_err() { break; }
        let trimmed = line.trim();
        if trimmed == "esci" || trimmed == "exit" { println!("{}", "Goodbye! 👋".cyan()); break; }
        if trimmed.is_empty() { continue; }
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
                        if !stdout.is_empty() { print!("{}", stdout); }
                        if !stderr.is_empty() { eprint!("{}", stderr.red()); }
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
    let source = match fs::read_to_string(path) { Ok(s) => s, Err(e) => { eprintln!("{} {}", "Error: file not found:".red().bold(), e); std::process::exit(1); } };
    let mut lexer = fratm_core::lexer::Lexer::new(&source);
    let tokens = lexer.tokenize();
    println!("{}", "Tokens:".cyan().bold());
    for token in tokens { println!("  {:20} @ {}:{}", format!("{:?}", token.kind).yellow(), token.span.line.to_string().dimmed(), token.span.column.to_string().dimmed()); }
}

fn show_ast(path: &PathBuf) {
    let source = match fs::read_to_string(path) { Ok(s) => s, Err(e) => { eprintln!("{} {}", "Error: file not found:".red().bold(), e); std::process::exit(1); } };
    let mut lexer = fratm_core::lexer::Lexer::new(&source);
    let tokens = lexer.tokenize();
    let mut parser = fratm_core::parser::Parser::new(tokens);
    match parser.parse() {
        Ok(program) => { println!("{}", "AST:".cyan().bold()); println!("{}", serde_json::to_string_pretty(&program).unwrap_or_default()); }
        Err(errors) => { for e in errors { println!("{} {}", "✗".red().bold(), e.message.red()); } }
    }
}

fn print_error(source: &str, error: &fratm_core::errors::CompileError) {
    let lines: Vec<&str> = source.lines().collect();
    eprintln!("\n{} {}", "✗ Error:".red().bold(), error);
    if let Some(line_num) = error.line() {
        if line_num > 0 && line_num <= lines.len() {
            let line = lines[line_num - 1];
            eprintln!("  {} │ {}", line_num.to_string().dimmed(), line);
            if let Some(col) = error.column() {
                let pointer = " ".repeat(col.saturating_sub(1)) + "^";
                eprintln!("  {} │ {}", " ".repeat(line_num.to_string().len()), pointer.red());
            }
        }
    }
    if let Some(suggestion) = errors::get_suggestion(error) { eprintln!("\n{}", suggestion.yellow()); }
    eprintln!("\n{}", errors::random_encouragement().dimmed());
}
