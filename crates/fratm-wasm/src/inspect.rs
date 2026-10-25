//! Tokens and syntax tree of a program, for the playground's Token and
//! Tree tabs.
//!
//! Positions are UTF-16 offsets (`from`..`to`), the unit JavaScript strings
//! and the editor use; the compiler's spans are byte offsets.

use fratm_core::lexer::{Lexer, TokenKind};
use fratm_core::parser::Parser;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct Inspection {
    pub tokens: Vec<TokenView>,
    /// `None` when the program does not parse
    pub tree: Option<TreeNode>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct TokenView {
    /// Token kind without its value, e.g. `Chist`, `Identifier`
    pub kind: String,
    /// `keyword`, `name`, `literal`, `punct` or `invalid`, for coloring
    pub category: &'static str,
    pub text: String,
    pub line: usize,
    pub column: usize,
    pub from: usize,
    pub to: usize,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct TreeNode {
    /// Field name in the parent (`left`, `value`, ...) or the index in a list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Node type (`VariableDecl`, `Binary`, ...) or `[n]` for lists
    pub label: String,
    /// Scalar value of a leaf (`"x"`, `1`, `Add`, `true`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<usize>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<TreeNode>,
}

pub fn inspect(source: &str) -> Inspection {
    let utf16 = Utf16Offsets::new(source);
    let tokens = Lexer::new(source).tokenize();
    let views = tokens
        .iter()
        .filter(|t| !matches!(t.kind, TokenKind::Newline | TokenKind::Eof))
        .map(|t| TokenView {
            kind: kind_name(&t.kind),
            category: category(&t.kind),
            text: t.literal.clone(),
            line: t.span.line,
            column: t.span.column,
            from: utf16.at(t.span.start),
            to: utf16.at(t.span.end),
        })
        .collect();

    let (tree, error) = match Parser::new(tokens).parse() {
        Ok(program) => match serde_json::to_value(&program) {
            Ok(value) => (Some(node(None, &value, &utf16, "Program")), None),
            Err(e) => (None, Some(e.to_string())),
        },
        Err(errors) => (
            None,
            errors.first().map(|e| {
                format!(
                    "Riga {}, colonna {}: {}",
                    e.span.line, e.span.column, e.message
                )
            }),
        ),
    };
    Inspection {
        tokens: views,
        tree,
        error,
    }
}

/// `Identifier("x")` → `Identifier`, `Template { .. }` → `Template`
fn kind_name(kind: &TokenKind) -> String {
    format!("{:?}", kind)
        .chars()
        .take_while(char::is_ascii_alphanumeric)
        .collect()
}

fn category(kind: &TokenKind) -> &'static str {
    match kind {
        TokenKind::Identifier(_) | TokenKind::PrivateName(_) => "name",
        TokenKind::String(_)
        | TokenKind::Number(_)
        | TokenKind::Template { .. }
        | TokenKind::Regex { .. }
        | TokenKind::Jsx(_)
        | TokenKind::Overo
        | TokenKind::Sfols
        | TokenKind::Nisciun
        | TokenKind::Boh => "literal",
        TokenKind::Invalid(_) => "invalid",
        // Keywords print as words (`chist`, `e`, `'o pate`), operators as symbols
        _ if kind
            .to_string()
            .starts_with(|c: char| c.is_alphabetic() || c == '\'') =>
        {
            "keyword"
        }
        _ => "punct",
    }
}

/// Builds a display node from the serde form of the AST: enum variants are
/// `{"Variant": {...}}`, structs are objects with a `span` field, unit
/// variants (operators) are strings.
fn node(key: Option<String>, value: &Value, utf16: &Utf16Offsets, label: &str) -> TreeNode {
    let leaf = |text: String| TreeNode {
        key: key.clone(),
        label: String::new(),
        value: Some(text),
        from: None,
        to: None,
        children: Vec::new(),
    };
    match value {
        Value::Null => leaf("nisciun".to_string()),
        Value::Bool(b) => leaf(b.to_string()),
        Value::Number(n) => leaf(n.to_string()),
        Value::String(s) => leaf(s.clone()),
        Value::Array(items) => TreeNode {
            key,
            label: format!("[{}]", items.len()),
            value: None,
            from: None,
            to: None,
            children: items
                .iter()
                .enumerate()
                .map(|(i, item)| node(Some(i.to_string()), item, utf16, ""))
                .collect(),
        },
        Value::Object(map) => {
            // Externally tagged enum variant: the variant name is the label
            if map.len() == 1 {
                let (variant, inner) = map.iter().next().unwrap();
                if variant.starts_with(|c: char| c.is_ascii_uppercase()) {
                    return match inner {
                        Value::Object(_) => node(key, inner, utf16, variant),
                        // Tuple variant or newtype around a scalar
                        _ => {
                            let mut n = node(key, inner, utf16, variant);
                            n.label = variant.clone();
                            n
                        }
                    };
                }
            }
            let span = map.get("span");
            let offset = |field: &str| {
                span.and_then(|s| s.get(field))
                    .and_then(Value::as_u64)
                    .map(|b| utf16.at(b as usize))
            };
            TreeNode {
                key,
                label: label.to_string(),
                value: None,
                from: offset("start"),
                to: offset("end"),
                children: map
                    .iter()
                    .filter(|(k, _)| k.as_str() != "span")
                    .map(|(k, v)| node(Some(k.clone()), v, utf16, ""))
                    .collect(),
            }
        }
    }
}

/// Byte offset → UTF-16 offset.
struct Utf16Offsets {
    /// (byte offset, utf-16 offset) at every char boundary
    starts: Vec<(usize, usize)>,
    len: (usize, usize),
}

impl Utf16Offsets {
    fn new(source: &str) -> Self {
        let mut starts = Vec::with_capacity(source.len());
        let mut units = 0;
        for (byte, c) in source.char_indices() {
            starts.push((byte, units));
            units += c.len_utf16();
        }
        Self {
            starts,
            len: (source.len(), units),
        }
    }

    fn at(&self, byte: usize) -> usize {
        if byte >= self.len.0 {
            return self.len.1;
        }
        match self.starts.binary_search_by_key(&byte, |&(b, _)| b) {
            Ok(i) => self.starts[i].1,
            // Inside a multi-byte char: its start
            Err(i) => self.starts[i.saturating_sub(1)].1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find<'a>(node: &'a TreeNode, label: &str) -> Option<&'a TreeNode> {
        if node.label == label {
            return Some(node);
        }
        node.children.iter().find_map(|c| find(c, label))
    }

    #[test]
    fn tokens_have_utf16_positions() {
        let source = "chist è x = \"è\"";
        let result = inspect(source);
        let kinds: Vec<_> = result.tokens.iter().map(|t| t.kind.as_str()).collect();
        assert_eq!(kinds, ["Chist", "E", "Identifier", "Equal", "String"]);
        let categories: Vec<_> = result.tokens.iter().map(|t| t.category).collect();
        assert_eq!(
            categories,
            ["keyword", "keyword", "name", "punct", "literal"]
        );
        let template = inspect("`a${1}`");
        assert_eq!(
            (
                template.tokens[0].kind.as_str(),
                template.tokens[0].category
            ),
            ("Template", "literal")
        );
        let x = &result.tokens[2];
        assert_eq!((x.text.as_str(), x.from, x.to), ("x", 8, 9));
        // 'è' is two bytes but one UTF-16 unit
        let s = &result.tokens[4];
        assert_eq!((s.from, s.to), (12, 15));
        let utf16: Vec<u16> = source.encode_utf16().collect();
        assert_eq!(String::from_utf16(&utf16[s.from..s.to]).unwrap(), "\"è\"");
    }

    #[test]
    fn tree_shows_node_types_fields_and_ranges() {
        let result = inspect("chist è x = 1 + 2");
        assert!(result.error.is_none());
        let tree = result.tree.unwrap();
        assert_eq!(tree.label, "Program");
        let decl = find(&tree, "VariableDecl").unwrap();
        assert_eq!((decl.from, decl.to), (Some(0), Some(17)));
        let binary = find(&tree, "Binary").unwrap();
        assert_eq!(binary.key.as_deref(), Some("value"));
        assert_eq!((binary.from, binary.to), (Some(12), Some(17)));
        let operator = binary
            .children
            .iter()
            .find(|c| c.key.as_deref() == Some("operator"))
            .unwrap();
        assert_eq!(operator.value.as_deref(), Some("Add"));
        let left = find(binary, "Number").unwrap();
        assert_eq!(left.children[0].value.as_deref(), Some("1.0"));
        // No raw span objects in the tree
        assert!(find(&tree, "span").is_none());
        assert!(!serde_json::to_string(&tree).unwrap().contains("\"span\""));
    }

    #[test]
    fn parse_errors_keep_the_tokens() {
        let result = inspect("chist è x = ");
        assert!(result.tree.is_none());
        assert!(result.error.unwrap().starts_with("Riga 1"));
        assert_eq!(result.tokens.len(), 4);
    }
}
