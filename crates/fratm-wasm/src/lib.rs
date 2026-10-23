//! WebAssembly bindings for FratmScript
//!
//! Enables running the compiler in the browser for the playground.

use fratm_core::{compile as core_compile, CompileOptions};
use wasm_bindgen::prelude::*;

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
#[wasm_bindgen]
pub fn compile(source: &str, generate_source_map: bool) -> JsValue {
    let response = compile_to_response(source, generate_source_map);
    serde_wasm_bindgen::to_value(&response).unwrap_or(JsValue::NULL)
}

/// Pure-Rust core of `compile`, kept separate so it can be tested natively
/// (`serde_wasm_bindgen` needs a JS host and panics outside wasm32).
fn compile_to_response(source: &str, generate_source_map: bool) -> CompileResponse {
    let options = CompileOptions {
        source_map: generate_source_map,
        filename: Some("input.fratm".to_string()),
        minify: false,
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
        Err(e) => CompileResponse {
            success: false,
            code: None,
            source_map: None,
            error: Some(format!("{}", e)),
            line: e.line(),
            column: e.column(),
            suggestion: fratm_core::errors::get_suggestion(&e),
        },
    }
}

/// Get the compiler version
#[wasm_bindgen]
pub fn version() -> String {
    fratm_core::version().to_string()
}

/// Tokenize source code (for syntax highlighting)
#[wasm_bindgen]
pub fn tokenize(source: &str) -> JsValue {
    let mut lexer = fratm_core::lexer::Lexer::new(source);
    let tokens = lexer.tokenize();
    serde_wasm_bindgen::to_value(&tokens).unwrap_or(JsValue::NULL)
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
        let result = compile_to_response("chist è x = 42", false);
        assert!(result.success);
        assert!(result.code.unwrap().contains("const x = 42"));
        assert!(result.error.is_none());
    }

    #[test]
    fn test_compile_source_map() {
        let result = compile_to_response("chist è x = 42", true);
        assert!(result.success);
        assert!(result.source_map.is_some());
    }

    #[test]
    fn test_compile_error_is_napoletano() {
        let result = compile_to_response("chist è x = ", false);
        assert!(!result.success);
        assert!(result.code.is_none());
        let error = result.error.unwrap();
        assert!(error.starts_with("Riga 1"), "{error}");
        assert!(error.contains("Ma che staje scrivenn?!"), "{error}");
        assert_eq!(result.line, Some(1));
    }
}
