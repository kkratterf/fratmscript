//! WebAssembly bindings for FratmScript
//!
//! Enables running the compiler in the browser for the playground.

use wasm_bindgen::prelude::*;
use fratm_core::{compile as core_compile, CompileOptions};

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
    let options = CompileOptions {
        source_map: generate_source_map,
        filename: Some("input.fratm".to_string()),
        minify: false,
    };

    match core_compile(source, options) {
        Ok(result) => {
            let response = CompileResponse {
                success: true,
                code: Some(result.code),
                source_map: result.source_map.map(|sm| sm.to_json()),
                error: None,
                line: None,
                column: None,
                suggestion: None,
            };
            serde_wasm_bindgen::to_value(&response).unwrap_or(JsValue::NULL)
        }
        Err(e) => {
            let response = CompileResponse {
                success: false,
                code: None,
                source_map: None,
                error: Some(format!("{}", e)),
                line: e.line(),
                column: e.column(),
                suggestion: fratm_core::errors::get_suggestion(&e),
            };
            serde_wasm_bindgen::to_value(&response).unwrap_or(JsValue::NULL)
        }
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
        let result = compile("chist è x = 42", false);
        assert!(!result.is_null());
    }
}
