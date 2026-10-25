//! `fratm mcp`: a Model Context Protocol server over stdio, so AI agents
//! (Claude Code, Cursor, Codex, VS Code...) can read the language reference
//! and check, compile, format and run FratmScript.
//!
//! The transport is newline-delimited JSON-RPC 2.0 on stdin/stdout.

use crate::esegui::{self, RunOptions};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Protocol version answered when the client asks for one we do not know.
const PROTOCOL_VERSION: &str = "2025-06-18";
const KNOWN_VERSIONS: &[&str] = &["2024-11-05", "2025-03-26", "2025-06-18"];

/// URI of the language reference resource.
const GUIDA_URI: &str = "fratm://guida";

/// Seconds `fratm_run` lets a program run when the call does not say.
const DEFAULT_TIMEOUT: f64 = 10.0;

const INSTRUCTIONS: &str = "FratmScript is JavaScript with Neapolitan keywords (.fratm files). \
Before writing FratmScript, read the reference (tool fratm_reference or resource fratm://guida): \
JavaScript keywords like const, function, return, if, new, this, true are NOT valid. \
Then check or run your code with fratm_check / fratm_run and fix it from the errors \
(they carry line, column and a hint).";

/// Serves requests from stdin until it is closed.
pub fn serve() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        if line.trim().is_empty() {
            continue;
        }
        let response = match serde_json::from_str::<Value>(&line) {
            Ok(Value::Array(batch)) => {
                let responses: Vec<Value> = batch.iter().filter_map(handle).collect();
                (!responses.is_empty()).then_some(Value::Array(responses))
            }
            Ok(message) => handle(&message),
            Err(e) => Some(error(Value::Null, -32700, &format!("Parse error: {}", e))),
        };
        if let Some(response) = response {
            if writeln!(stdout, "{}", response)
                .and_then(|_| stdout.flush())
                .is_err()
            {
                break;
            }
        }
    }
}

/// Answers one JSON-RPC message; notifications get no answer.
pub fn handle(message: &Value) -> Option<Value> {
    let id = message.get("id")?.clone();
    let method = message["method"].as_str().unwrap_or_default();
    let params = &message["params"];
    let result = match method {
        "initialize" => Ok(initialize(params)),
        "ping" => Ok(json!({})),
        "tools/list" => Ok(json!({ "tools": tools() })),
        "tools/call" => call_tool(params),
        "resources/list" => Ok(json!({ "resources": [guida_resource()] })),
        "resources/read" => read_resource(params),
        "prompts/list" => Ok(json!({ "prompts": [] })),
        _ => Err((-32601, format!("Method not found: {}", method))),
    };
    Some(match result {
        Ok(result) => json!({ "jsonrpc": "2.0", "id": id, "result": result }),
        Err((code, text)) => error(id, code, &text),
    })
}

fn error(id: Value, code: i64, message: &str) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } })
}

fn initialize(params: &Value) -> Value {
    let asked = params["protocolVersion"].as_str().unwrap_or_default();
    let version = if KNOWN_VERSIONS.contains(&asked) {
        asked
    } else {
        PROTOCOL_VERSION
    };
    json!({
        "protocolVersion": version,
        "capabilities": { "tools": {}, "resources": {}, "prompts": {} },
        "serverInfo": { "name": "fratm", "title": "FratmScript", "version": fratm_core::version() },
        "instructions": INSTRUCTIONS,
    })
}

fn guida_resource() -> Value {
    json!({
        "uri": GUIDA_URI,
        "name": "guida",
        "title": "FratmScript reference for AI agents",
        "description": "Keywords, rules, examples, error glossary and tools",
        "mimeType": "text/markdown",
    })
}

fn read_resource(params: &Value) -> Result<Value, (i64, String)> {
    match params["uri"].as_str() {
        Some(GUIDA_URI) => Ok(json!({
            "contents": [{ "uri": GUIDA_URI, "mimeType": "text/markdown", "text": crate::GUIDA }]
        })),
        other => Err((
            -32002,
            format!("Resource not found: {}", other.unwrap_or("")),
        )),
    }
}

/// Input schema of the tools that take a program.
fn program_schema(extra: Value) -> Value {
    let mut properties = json!({
        "source": { "type": "string", "description": "FratmScript source code" },
        "path": { "type": "string", "description": "Path of a .fratm file, instead of source" },
    });
    if let (Some(properties), Some(extra)) = (properties.as_object_mut(), extra.as_object()) {
        properties.extend(extra.clone());
    }
    json!({ "type": "object", "properties": properties })
}

fn tools() -> Value {
    let read_only = json!({ "readOnlyHint": true, "openWorldHint": false });
    json!([
        {
            "name": "fratm_reference",
            "title": "FratmScript reference",
            "description": "The FratmScript language reference for AI agents: keywords and their JavaScript equivalents, rules that differ from JavaScript, examples, error glossary. Read it before writing FratmScript.",
            "inputSchema": { "type": "object", "properties": {} },
            "annotations": read_only,
        },
        {
            "name": "fratm_check",
            "title": "Check FratmScript",
            "description": "Compile FratmScript without running it. Returns {ok, errors}; each error has message (Neapolitan), line, column and suggestion.",
            "inputSchema": program_schema(json!({})),
            "annotations": read_only,
        },
        {
            "name": "fratm_compile",
            "title": "Compile FratmScript to JavaScript",
            "description": "Compile FratmScript to JavaScript (an ES module). Returns {ok, code, errors}.",
            "inputSchema": program_schema(json!({})),
            "annotations": read_only,
        },
        {
            "name": "fratm_run",
            "title": "Run FratmScript",
            "description": "Compile FratmScript and run it with Node.js. Returns {ok, errors, stdout, stderr, exitCode, runtimeError}; runtimeError has a Neapolitan message (original JavaScript error in brackets) and the .fratm line and column. The program gets no stdin.",
            "inputSchema": program_schema(json!({
                "timeout_seconds": {
                    "type": "number",
                    "description": "Stop the program after this many seconds (default 10)",
                },
            })),
            "annotations": { "readOnlyHint": false, "destructiveHint": false, "openWorldHint": true },
        },
        {
            "name": "fratm_format",
            "title": "Format FratmScript",
            "description": "Format FratmScript source like `fratm fmt` (comments kept). Returns {ok, code, errors}. Does not write files.",
            "inputSchema": program_schema(json!({})),
            "annotations": read_only,
        },
        {
            "name": "fratm_gomorra",
            "title": "Translate a GomorraSQL query",
            "description": "Translate a GomorraSQL query (the text inside gomorra`…`, without ${…}) to SQL, to check it before using it. Returns {ok, sql, text, error}: sql uses ? placeholders, text uses $1; error has a Neapolitan message, line and column inside the query.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "GomorraSQL, e.g. ripigliammo nomme mmiez 'a pizze arò prezzo < 10",
                    },
                },
                "required": ["query"],
            },
            "annotations": read_only,
        },
    ])
}

/// The program a tool call is about: its source, and the file it came from.
fn program(arguments: &Value) -> Result<(String, Option<PathBuf>), String> {
    if let Some(source) = arguments["source"].as_str() {
        return Ok((source.to_string(), None));
    }
    let Some(path) = arguments["path"].as_str() else {
        return Err("give 'source' (FratmScript code) or 'path' (a .fratm file)".to_string());
    };
    let path = PathBuf::from(path);
    std::fs::read_to_string(&path)
        .map(|source| (source, Some(path.clone())))
        .map_err(|e| format!("cannot read {}: {}", path.display(), e))
}

fn call_tool(params: &Value) -> Result<Value, (i64, String)> {
    let name = params["name"].as_str().unwrap_or_default();
    let arguments = &params["arguments"];
    if name == "fratm_reference" {
        return Ok(json!({ "content": [{ "type": "text", "text": crate::GUIDA }] }));
    }
    if name == "fratm_gomorra" {
        let Some(query) = arguments["query"].as_str() else {
            return Ok(
                json!({ "content": [{ "type": "text", "text": "give 'query' (GomorraSQL)" }], "isError": true }),
            );
        };
        let result = match crate::gomorra::translate_query(query) {
            Ok(q) => json!({ "ok": true, "sql": q.sql, "text": q.text, "error": null }),
            Err(e) => {
                let (line, column) = crate::gomorra::error_position(&e);
                json!({
                    "ok": false,
                    "sql": null,
                    "text": null,
                    "error": { "message": e.message, "line": line, "column": column },
                })
            }
        };
        let text = serde_json::to_string_pretty(&result).unwrap_or_default();
        return Ok(json!({
            "content": [{ "type": "text", "text": text }],
            "structuredContent": result,
            "isError": false,
        }));
    }
    if !["fratm_check", "fratm_compile", "fratm_run", "fratm_format"].contains(&name) {
        return Err((-32602, format!("Unknown tool: {}", name)));
    }
    let (source, path) = match program(arguments) {
        Ok(program) => program,
        Err(message) => {
            return Ok(json!({ "content": [{ "type": "text", "text": message }], "isError": true }))
        }
    };
    let result = match name {
        "fratm_check" => match fratm_core::compile(&source, Default::default()) {
            Ok(_) => json!({ "ok": true, "errors": [] }),
            Err(e) => json!({ "ok": false, "errors": [esegui::error_json(&e, &source)] }),
        },
        "fratm_compile" => match fratm_core::compile(&source, Default::default()) {
            Ok(result) => json!({ "ok": true, "code": result.code, "errors": [] }),
            Err(e) => {
                json!({ "ok": false, "code": null, "errors": [esegui::error_json(&e, &source)] })
            }
        },
        "fratm_format" => match fratm_core::format(&source) {
            Ok(code) => json!({ "ok": true, "code": code, "errors": [] }),
            Err(e) => {
                json!({ "ok": false, "code": null, "errors": [esegui::error_json(&e, &source)] })
            }
        },
        _ => {
            let timeout = arguments["timeout_seconds"]
                .as_f64()
                .filter(|t| *t > 0.0)
                .unwrap_or(DEFAULT_TIMEOUT);
            let temp = std::env::temp_dir();
            let dir = match path.as_deref().and_then(Path::parent) {
                Some(dir) if !dir.as_os_str().is_empty() => dir,
                Some(_) => Path::new("."),
                None => temp.as_path(),
            };
            let outcome = esegui::run(
                &source,
                RunOptions {
                    dir,
                    capture: true,
                    timeout: Some(Duration::from_secs_f64(timeout)),
                },
            );
            esegui::outcome_json(&outcome, &source)
        }
    };
    let text = serde_json::to_string_pretty(&result).unwrap_or_default();
    Ok(json!({
        "content": [{ "type": "text", "text": text }],
        "structuredContent": result,
        "isError": false,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(method: &str, params: Value) -> Value {
        handle(&json!({ "jsonrpc": "2.0", "id": 1, "method": method, "params": params })).unwrap()
    }

    fn call(name: &str, arguments: Value) -> Value {
        request(
            "tools/call",
            json!({ "name": name, "arguments": arguments }),
        )["result"]
            .clone()
    }

    #[test]
    fn initialize_answers_the_asked_version() {
        let response = request("initialize", json!({ "protocolVersion": "2025-03-26" }));
        assert_eq!(response["result"]["protocolVersion"], "2025-03-26");
        assert_eq!(response["result"]["serverInfo"]["name"], "fratm");
        let response = request("initialize", json!({ "protocolVersion": "1999-01-01" }));
        assert_eq!(response["result"]["protocolVersion"], PROTOCOL_VERSION);
    }

    #[test]
    fn notifications_get_no_answer() {
        let notification = json!({ "jsonrpc": "2.0", "method": "notifications/initialized" });
        assert_eq!(handle(&notification), None);
    }

    #[test]
    fn unknown_methods_are_errors() {
        assert_eq!(request("nope", json!({}))["error"]["code"], -32601);
    }

    #[test]
    fn lists_the_tools() {
        let tools = request("tools/list", json!({}))["result"]["tools"].clone();
        let names: Vec<&str> = tools
            .as_array()
            .unwrap()
            .iter()
            .map(|t| t["name"].as_str().unwrap())
            .collect();
        assert_eq!(
            names,
            [
                "fratm_reference",
                "fratm_check",
                "fratm_compile",
                "fratm_run",
                "fratm_format",
                "fratm_gomorra"
            ]
        );
    }

    #[test]
    fn gomorra_translates_or_explains_a_query() {
        let result = call(
            "fratm_gomorra",
            json!({ "query": "ripigliammo nomme mmiez 'a pizze nun cchiù 'e 3" }),
        );
        assert_eq!(
            result["structuredContent"]["sql"],
            "SELECT nomme FROM pizze LIMIT 3"
        );
        let result = call("fratm_gomorra", json!({ "query": "ripigliammo nomme arò" }));
        assert_eq!(result["structuredContent"]["ok"], false);
        assert_eq!(result["structuredContent"]["error"]["column"], 19);
        assert_eq!(call("fratm_gomorra", json!({}))["isError"], true);
    }

    #[test]
    fn reference_tool_and_resource_return_the_guide() {
        let text = call("fratm_reference", json!({}))["content"][0]["text"].clone();
        assert!(text.as_str().unwrap().contains("chist è"));
        let read = request("resources/read", json!({ "uri": GUIDA_URI }));
        assert_eq!(read["result"]["contents"][0]["text"], crate::GUIDA);
    }

    #[test]
    fn check_reports_errors_with_a_hint() {
        let result = call("fratm_check", json!({ "source": "facc f() { return 1 }" }));
        let errors = &result["structuredContent"]["errors"];
        assert_eq!(result["structuredContent"]["ok"], false);
        assert_eq!(errors[0]["line"], 1);
        assert_eq!(
            errors[0]["suggestion"],
            "In FratmScript 'return' se scrive 'piglie'"
        );
    }

    #[test]
    fn compile_returns_javascript() {
        let result = call("fratm_compile", json!({ "source": "stamm a dì(overo)" }));
        assert_eq!(result["structuredContent"]["code"], "console.log(true);\n");
    }

    #[test]
    fn format_returns_formatted_source() {
        let result = call(
            "fratm_format",
            json!({ "source": "si (overo) {\nstamm a dì(1)\n}\n" }),
        );
        assert_eq!(
            result["structuredContent"]["code"],
            "si (overo) {\n    stamm a dì(1)\n}\n"
        );
    }

    #[test]
    fn missing_source_is_a_tool_error() {
        let result = call("fratm_check", json!({}));
        assert_eq!(result["isError"], true);
    }
}
