//! WebAssembly bindings for FratmScript
//!
//! Enables running the compiler in the browser for the playground.

use fratm_core::{compile as core_compile, CompileOptions};
use wasm_bindgen::prelude::*;

mod inspect;

/// Initialize panic hook for better error messages in browser console
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Compile FratmScript source to JavaScript
///
/// Returns a JSON object with:
/// - `success`: boolean
/// - `code`: string (if success)
/// - `sourceMap`: string (if success and requested)
/// - `error`: string (if failure)
/// - `line`: number (if failure)
/// - `column`: number (if failure)
///
/// `jsx_import_source` picks the library of the JSX runtime (`react` when
/// left out).
#[wasm_bindgen]
pub fn compile(
    source: &str,
    generate_source_map: bool,
    jsx_import_source: Option<String>,
) -> JsValue {
    let response = compile_to_response(source, generate_source_map, jsx_import_source);
    serde_wasm_bindgen::to_value(&response).unwrap_or(JsValue::NULL)
}

/// Pure-Rust core of `compile`, kept separate so it can be tested natively
/// (`serde_wasm_bindgen` needs a JS host and panics outside wasm32).
fn compile_to_response(
    source: &str,
    generate_source_map: bool,
    jsx_import_source: Option<String>,
) -> CompileResponse {
    let options = CompileOptions {
        source_map: generate_source_map,
        filename: Some("input.fratm".to_string()),
        minify: false,
        jsx_import_source,
    };

    match core_compile(source, options) {
        Ok(result) => CompileResponse {
            success: true,
            code: Some(result.code),
            source_map: result.source_map.map(|sm| sm.to_json()),
            error: None,
            line: None,
            column: None,
            suggestion: None,
        },
        Err(e) => error_response(&e, source),
    }
}

/// Format FratmScript source like `fratm fmt`
///
/// Returns a JSON object with `success`, `code` (the formatted source) or
/// `error`, `line`, `column` and `suggestion`.
#[wasm_bindgen]
pub fn format(source: &str) -> JsValue {
    serde_wasm_bindgen::to_value(&format_to_response(source)).unwrap_or(JsValue::NULL)
}

fn format_to_response(source: &str) -> CompileResponse {
    match fratm_core::format(source) {
        Ok(code) => CompileResponse {
            success: true,
            code: Some(code),
            source_map: None,
            error: None,
            line: None,
            column: None,
            suggestion: None,
        },
        Err(e) => error_response(&e, source),
    }
}

fn error_response(e: &fratm_core::errors::CompileError, source: &str) -> CompileResponse {
    CompileResponse {
        success: false,
        code: None,
        source_map: None,
        error: Some(format!("{}", e)),
        line: e.line(),
        column: e.column(),
        suggestion: fratm_core::errors::suggest(e, source),
    }
}

/// Get the compiler version
#[wasm_bindgen]
pub fn version() -> String {
    fratm_core::version().to_string()
}

/// Neapolitan version of a JavaScript runtime error, with the original in
/// brackets (`'x' nun esiste … (ReferenceError: x is not defined)`)
#[wasm_bindgen(js_name = runtimeError)]
pub fn runtime_error(name: &str, message: &str) -> String {
    fratm_core::runtime::napoletanize_runtime_error(name, message)
}

/// The files of a new project, like `fratm nuovo`, as a JSON string:
/// `[{ path, contents }]`, or `null` for an unknown template (`react`,
/// `completa`, `senza-react`, `semplice`). Used by `npm create fratm`.
#[wasm_bindgen]
pub fn scaffold(template: &str, folder: &str, tailwind: bool) -> String {
    let Some(template) = fratm_core::template::Template::from_name(template) else {
        return "null".to_string();
    };
    let files: Vec<serde_json::Value> =
        fratm_core::template::files(template, folder, fratm_core::version(), tailwind)
            .into_iter()
            .map(|(path, contents)| serde_json::json!({ "path": path, "contents": contents }))
            .collect();
    serde_json::Value::Array(files).to_string()
}

/// Tokenize source code (for syntax highlighting)
#[wasm_bindgen]
pub fn tokenize(source: &str) -> JsValue {
    let mut lexer = fratm_core::lexer::Lexer::new(source);
    let tokens = lexer.tokenize();
    serde_wasm_bindgen::to_value(&tokens).unwrap_or(JsValue::NULL)
}

/// Tokens and syntax tree for the playground's Token and Tree tabs, as a
/// JSON string: `{ tokens, tree, error }` (positions in UTF-16 units)
#[wasm_bindgen]
pub fn inspect(source: &str) -> String {
    serde_json::to_string(&inspect::inspect(source)).unwrap_or_default()
}

/// Response structure for compile function
#[derive(serde::Serialize)]
struct CompileResponse {
    success: bool,
    code: Option<String>,
    #[serde(rename = "sourceMap")]
    source_map: Option<String>,
    error: Option<String>,
    line: Option<usize>,
    column: Option<usize>,
    suggestion: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile() {
        let result = compile_to_response("chist è x = 42", false, None);
        assert!(result.success);
        assert!(result.code.unwrap().contains("const x = 42"));
        assert!(result.error.is_none());
    }

    #[test]
    fn test_compile_jsx_import_source() {
        let code = |source: Option<&str>| {
            compile_to_response("x = <b />", false, source.map(String::from))
                .code
                .unwrap()
        };
        assert!(code(None).contains("from \"react/jsx-runtime\""));
        assert!(code(Some("preact")).contains("from \"preact/jsx-runtime\""));
    }

    #[test]
    fn test_compile_source_map() {
        let result = compile_to_response("chist è x = 42", true, None);
        assert!(result.success);
        assert!(result.source_map.is_some());
    }

    #[test]
    fn test_runtime_error() {
        assert!(runtime_error("ReferenceError", "x is not defined").starts_with("'x' nun esiste"));
    }

    #[test]
    fn test_format() {
        let result = format_to_response("si (overo) {\nstamm a dì(1)\n}");
        assert!(result.success);
        assert_eq!(result.code.unwrap(), "si (overo) {\n    stamm a dì(1)\n}\n");
        let result = format_to_response("tien = 1");
        assert!(!result.success);
        assert_eq!(result.line, Some(1));
    }

    #[test]
    fn test_compile_error_is_napoletano() {
        let result = compile_to_response("chist è x = ", false, None);
        assert!(!result.success);
        assert!(result.code.is_none());
        let error = result.error.unwrap();
        assert!(error.starts_with("Riga 1"), "{error}");
        assert!(error.contains("Ma che staje scrivenn?!"), "{error}");
        assert_eq!(result.line, Some(1));
    }
}
