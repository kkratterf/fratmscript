//! Source formatter behind `fratm fmt`.
//!
//! The formatter works on the token stream, not on the AST, so it keeps
//! everything the AST throws away: comments, the keyword spelling the author
//! chose (`chist è` / `chist e`, `stamm a dì`), parentheses and blank lines.
//! It only rewrites whitespace:
//!
//! - every line is re-indented with four spaces per open block, bracket or
//!   parenthesis (switch `caso` bodies one level deeper than their label);
//! - runs of spaces collapse to one, with none inside `( )` and `[ ]`,
//!   none before `,` `;` and none around `.`, and one after `,` and `;`;
//! - trailing whitespace goes, at most one blank line is kept and the file
//!   ends with a single newline.
//!
//! Line breaks are never added or removed, so statement boundaries do not
//! move. As a safety net the result must compile to exactly the same
//! JavaScript as the input, otherwise formatting fails and nothing changes.

use crate::errors::CompileError;
use crate::lexer::{Lexer, Token, TokenKind};
use crate::{compile, CompileOptions};

const INDENT: &str = "    ";

/// Formats FratmScript source. Fails if the source does not compile.
pub fn format(source: &str) -> Result<String, CompileError> {
    let before = compile(source, CompileOptions::default())?;
    let formatted = Formatter::new(source).run();
    let after =
        compile(&formatted, CompileOptions::default()).map_err(|e| CompileError::CodeGenError {
            message: format!("fratm fmt produced code that does not compile: {}", e),
        })?;
    if before.code != after.code {
        return Err(CompileError::CodeGenError {
            message: "fratm fmt would change the meaning of this file, so it was left as it is"
                .to_string(),
        });
    }
    Ok(formatted)
}

enum Piece<'a> {
    Token(Token),
    Comment { text: &'a str, line: bool },
    Space,
    Newlines(usize),
}

struct Frame {
    /// Indent level of the line that opened the frame
    base: usize,
    switch: bool,
    /// For `(`: whether it follows `vedimm`, so the next `{` opens a switch
    after_vedimm: bool,
}

struct Formatter<'a> {
    pieces: Vec<Piece<'a>>,
    out: String,
    frames: Vec<Frame>,
    /// Indent level of the line being written
    level: usize,
    /// Previous token on the current line
    prev: Option<TokenKind>,
    /// Whether the `)` just closed followed `vedimm`
    closed_vedimm: bool,
    newlines: usize,
    space: bool,
}

impl<'a> Formatter<'a> {
    fn new(source: &'a str) -> Self {
        Formatter {
            pieces: split(source),
            out: String::new(),
            frames: Vec::new(),
            level: 0,
            prev: None,
            closed_vedimm: false,
            newlines: 0,
            space: false,
        }
    }

    fn run(mut self) -> String {
        let pieces = std::mem::take(&mut self.pieces);
        for (i, piece) in pieces.iter().enumerate() {
            match piece {
                Piece::Space => self.space = true,
                Piece::Newlines(n) => {
                    if !self.out.is_empty() {
                        self.newlines = self.newlines.max(*n);
                    }
                    self.space = false;
                }
                Piece::Comment { text, line } => {
                    if self.start_line_if_needed() {
                        let next = pieces[i + 1..].iter().find_map(|p| match p {
                            Piece::Token(t) => Some(&t.kind),
                            _ => None,
                        });
                        self.level = match next {
                            Some(kind) if !is_closer(kind) => self.line_level(kind),
                            _ => self.body_level(),
                        };
                        self.write_indent();
                    } else if *line || self.space {
                        self.out.push(' ');
                    }
                    self.out.push_str(text);
                    // A comment between two tokens keeps them apart
                    self.space = !*line;
                }
                Piece::Token(token) => self.token(token),
            }
        }
        let trimmed = self.out.trim_end().len();
        self.out.truncate(trimmed);
        if !self.out.is_empty() {
            self.out.push('\n');
        }
        self.out
    }

    /// Starts a new output line when a newline is pending (or nothing was
    /// written yet). Returns whether a line was started.
    fn start_line_if_needed(&mut self) -> bool {
        if self.newlines == 0 && !self.out.is_empty() {
            return false;
        }
        if !self.out.is_empty() {
            let trimmed = self.out.trim_end_matches([' ', '\t']).len();
            self.out.truncate(trimmed);
            self.out.push('\n');
            if self.newlines >= 2 {
                self.out.push('\n');
            }
        }
        self.newlines = 0;
        self.space = false;
        self.prev = None;
        true
    }

    fn write_indent(&mut self) {
        for _ in 0..self.level {
            self.out.push_str(INDENT);
        }
    }

    /// Level of a line inside the innermost frame
    fn body_level(&self) -> usize {
        match self.frames.last() {
            None => 0,
            Some(f) if f.switch => f.base + 2,
            Some(f) => f.base + 1,
        }
    }

    /// Level of a line that starts with `kind`
    fn line_level(&self, kind: &TokenKind) -> usize {
        match (self.frames.last(), kind) {
            (Some(f), _) if is_closer(kind) => f.base,
            (Some(f), TokenKind::Caso | TokenKind::Predefinit) if f.switch => f.base + 1,
            _ => self.body_level(),
        }
    }

    fn token(&mut self, token: &Token) {
        let kind = &token.kind;
        if self.start_line_if_needed() {
            self.level = self.line_level(kind);
            self.write_indent();
        } else if let Some(prev) = &self.prev {
            if wants_space(prev, kind, self.space) {
                self.out.push(' ');
            }
        } else if self.space {
            // After an inline block comment
            self.out.push(' ');
        }
        self.out.push_str(&token.literal);
        self.space = false;

        match kind {
            TokenKind::LeftBrace => {
                let switch = self.prev == Some(TokenKind::RightParen) && self.closed_vedimm;
                self.push_frame(switch, false);
            }
            TokenKind::LeftParen => {
                let after_vedimm =
                    matches!(&self.prev, Some(TokenKind::Identifier(name)) if name == "vedimm");
                self.push_frame(false, after_vedimm);
            }
            TokenKind::LeftBracket => self.push_frame(false, false),
            TokenKind::RightParen => {
                self.closed_vedimm = self.frames.pop().is_some_and(|f| f.after_vedimm);
            }
            TokenKind::RightBrace | TokenKind::RightBracket => {
                self.frames.pop();
            }
            _ => {}
        }
        self.prev = Some(kind.clone());
    }

    fn push_frame(&mut self, switch: bool, after_vedimm: bool) {
        self.frames.push(Frame {
            base: self.level,
            switch,
            after_vedimm,
        });
    }
}

fn is_closer(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::RightBrace | TokenKind::RightParen | TokenKind::RightBracket
    )
}

/// Whether to put a space between two tokens on the same line, given
/// whether the source had whitespace between them.
fn wants_space(prev: &TokenKind, next: &TokenKind, had_space: bool) -> bool {
    use TokenKind::*;
    match (prev, next) {
        // Array holes: `[a, , b]`
        (Comma, Comma) => true,
        (_, Comma | Semicolon | RightParen | RightBracket) => false,
        (LeftParen | LeftBracket | Ellipsis, _) => false,
        (Dot | QuestionDot, _) | (_, Dot | QuestionDot) => false,
        (Comma | Semicolon, _) => true,
        _ => had_space,
    }
}

/// Splits the source into tokens and the whitespace and comments between them.
fn split(source: &str) -> Vec<Piece<'_>> {
    let tokens = Lexer::new(source).tokenize();
    let mut pieces = Vec::new();
    let mut pos = 0;
    for token in tokens {
        if matches!(token.kind, TokenKind::Newline | TokenKind::Eof) {
            continue;
        }
        gap(&source[pos..token.span.start], &mut pieces);
        pos = token.span.end;
        pieces.push(Piece::Token(token));
    }
    gap(&source[pos..], &mut pieces);
    pieces
}

/// Splits a stretch of whitespace and comments.
fn gap<'a>(text: &'a str, pieces: &mut Vec<Piece<'a>>) {
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'\n' => {
                let mut n = 0;
                while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                    if bytes[i] == b'\n' {
                        n += 1;
                    }
                    i += 1;
                }
                pieces.push(Piece::Newlines(n));
            }
            b' ' | b'\t' | b'\r' => {
                i += 1;
                if !matches!(pieces.last(), Some(Piece::Space)) {
                    pieces.push(Piece::Space);
                }
            }
            b'/' if bytes.get(i + 1) == Some(&b'/') => {
                let end = text[i..].find('\n').map_or(text.len(), |n| i + n);
                pieces.push(Piece::Comment {
                    text: text[i..end].trim_end(),
                    line: true,
                });
                i = end;
            }
            b'/' if bytes.get(i + 1) == Some(&b'*') => {
                let end = text[i + 2..]
                    .find("*/")
                    .map_or(text.len(), |n| i + 2 + n + 2);
                pieces.push(Piece::Comment {
                    text: &text[i..end],
                    line: false,
                });
                i = end;
            }
            // Anything else between tokens would be a lexer bug; keep it
            _ => {
                let end = text[i..]
                    .find(|c: char| c.is_whitespace())
                    .map_or(text.len(), |n| i + n);
                pieces.push(Piece::Comment {
                    text: &text[i..end],
                    line: false,
                });
                i = end;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fmt(source: &str) -> String {
        format(source).unwrap()
    }

    #[test]
    fn test_reindents_blocks_and_keeps_comments() {
        let source = "// saluto\nfacc saluta(nome)   {\n  si (nome == nisciun) {   // nisciuno\n        piglie \"E chi si tu?\"\n  }\n\n\n\n  piglie   nome\n}   \n";
        assert_eq!(
            fmt(source),
            "// saluto\nfacc saluta(nome) {\n    si (nome == nisciun) { // nisciuno\n        piglie \"E chi si tu?\"\n    }\n\n    piglie nome\n}\n"
        );
    }

    #[test]
    fn test_normalizes_spacing_around_brackets_and_commas() {
        assert_eq!(
            fmt("tien a = f( 1 ,2 ,[ 3,4 ] )\nstamm a dì( a . b ?. c )"),
            "tien a = f(1, 2, [3, 4])\nstamm a dì(a.b?.c)\n"
        );
        // Spread and `for (;;)` headers
        assert_eq!(
            fmt("tien a = [ ... b ]\npe (tien i = 0 ;i < 3 ;i++) { }"),
            "tien a = [...b]\npe (tien i = 0; i < 3; i++) { }\n"
        );
        assert_eq!(fmt("tien [a,,b] = c"), "tien [a, , b] = c\n");
    }

    #[test]
    fn test_keeps_keyword_spelling_strings_and_templates() {
        let source = "chist e x = `a  ${ x }\n   b`\nstamm a dì('  doje   spazie  ')\n";
        assert_eq!(fmt(source), source);
    }

    #[test]
    fn test_indents_multiline_calls() {
        assert_eq!(
            fmt("f(1,\n2, () => {\nstamm a dì(1)\n})\n"),
            "f(1,\n    2, () => {\n        stamm a dì(1)\n    })\n"
        );
    }

    #[test]
    fn test_indents_switch_cases() {
        let source = "vedimm (x) {\ncaso 1:\nstamm a dì(1)\ncaso 2: {\nrompe\n}\npredefinit:\n// niente\n}\n";
        assert_eq!(
            fmt(source),
            "vedimm (x) {\n    caso 1:\n        stamm a dì(1)\n    caso 2: {\n        rompe\n    }\n    predefinit:\n        // niente\n}\n"
        );
    }

    #[test]
    fn test_keeps_block_comments() {
        assert_eq!(
            fmt("/* uno\n   doje */\ntien a = /* x */ 1\n"),
            "/* uno\n   doje */\ntien a = /* x */ 1\n"
        );
    }

    #[test]
    fn test_is_idempotent_and_rejects_invalid_source() {
        let once = fmt("facc f( a ,b ){\npiglie a+b }\n");
        assert_eq!(fmt(&once), once);
        assert!(format("tien = 1").is_err());
        assert_eq!(fmt(""), "");
    }
}
