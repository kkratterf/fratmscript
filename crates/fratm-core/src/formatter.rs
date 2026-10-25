//! Source formatter behind `fratm fmt`.
//!
//! The formatter works on the token stream, not on the AST, so it keeps
//! everything the AST throws away: comments, the keyword spelling the author
//! chose (`chist è` / `chist e`, `stamm a dì`), parentheses and blank lines.
//! It only rewrites whitespace:
//!
//! - every line is re-indented with four spaces per open block, bracket or
//!   parenthesis (switch `caso` bodies one level deeper than their label);
//!   a line that continues an expression (after `+`, before `.map()`, …)
//!   goes one level deeper than the line that started it;
//! - runs of spaces collapse to one, with none inside `( )` and `[ ]`,
//!   none before `,` `;` and none around `.`, and one after `,` and `;`;
//! - trailing whitespace goes, at most one blank line is kept and the file
//!   ends with a single newline.
//!
//! Line breaks are never added or removed, so statement boundaries do not
//! move. As a safety net the result must compile to exactly the same
//! JavaScript as the input, otherwise formatting fails and nothing changes.

use crate::errors::CompileError;
use crate::lexer::{continues_after, continues_before, Lexer, Token, TokenKind};
use crate::{compile, CompileOptions};

const INDENT: &str = "    ";

/// Formats FratmScript source. Fails if the source does not compile.
pub fn format(source: &str) -> Result<String, CompileError> {
    let before = compile(source, CompileOptions::default())?;
    // The shebang line is kept as written; the formatter sees only the rest.
    let formatted = match crate::shebang(source) {
        Some(line) => {
            let rest = &source[line.len()..];
            let rest = rest
                .strip_prefix("\r\n")
                .or_else(|| rest.strip_prefix('\n'))
                .unwrap_or(rest);
            format!("{}\n{}", line, Formatter::new(rest).run())
        }
        None => Formatter::new(source).run(),
    };
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
    /// `(` or `[`: lines inside are continuations already
    paren: bool,
    /// `stmt_level` outside the frame, restored when it closes
    outer_stmt: usize,
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
    /// Base level of the `(` just closed, for a `{` that follows it
    closed_base: usize,
    /// Last token written, on this line or an earlier one
    last: Option<Token>,
    /// Level of the last line that did not continue an expression
    stmt_level: usize,
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
            closed_base: 0,
            last: None,
            stmt_level: 0,
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
            let in_paren = self.frames.last().is_some_and(|f| f.paren);
            let in_switch = self.frames.last().is_some_and(|f| f.switch);
            // In a switch, `caso x:` ends a label, not an expression
            let after = self
                .last
                .as_ref()
                .is_some_and(|t| continues_after(t) && !(in_switch && t.kind == TokenKind::Colon));
            let continues = !in_paren && !is_closer(kind) && (after || continues_before(token));
            if continues {
                self.level = self.stmt_level + 1;
            } else {
                self.level = self.line_level(kind);
                if !is_closer(kind) {
                    self.stmt_level = self.level;
                }
            }
            self.write_indent();
        } else if let Some(prev) = &self.prev {
            if wants_space(prev, kind, self.space) {
                self.out.push(' ');
            }
        } else if self.space {
            // After an inline block comment
            self.out.push(' ');
        }
        if matches!(kind, TokenKind::Jsx(_)) && token.literal.contains('\n') {
            self.push_jsx(token);
        } else {
            self.out.push_str(&token.literal);
        }
        self.space = false;

        match kind {
            TokenKind::LeftBrace => {
                let after_paren = self.prev == Some(TokenKind::RightParen);
                let switch = after_paren && self.closed_vedimm;
                // `si (a e\n    b) {`: the block belongs to the line of `si`
                let base = if after_paren {
                    self.closed_base.min(self.level)
                } else {
                    self.level
                };
                self.push_frame(base, switch, false, false);
            }
            TokenKind::LeftParen => {
                let after_vedimm =
                    matches!(&self.prev, Some(TokenKind::Identifier(name)) if name == "vedimm");
                self.push_frame(self.level, false, after_vedimm, true);
            }
            TokenKind::LeftBracket => self.push_frame(self.level, false, false, true),
            TokenKind::RightParen => {
                let frame = self.pop_frame();
                self.closed_vedimm = frame.as_ref().is_some_and(|f| f.after_vedimm);
                self.closed_base = frame.map_or(self.level, |f| f.base);
            }
            TokenKind::RightBrace | TokenKind::RightBracket => {
                self.pop_frame();
            }
            _ => {}
        }
        self.prev = Some(kind.clone());
        self.last = Some(token.clone());
    }

    /// A JSX element over several lines moves as a block: its later lines
    /// shift by as much as its first line did, so the markup keeps its shape.
    fn push_jsx(&mut self, token: &Token) {
        let line_start = self.out.rfind('\n').map_or(0, |i| i + 1);
        let new_column = self.out[line_start..].chars().count() as isize;
        let shift = new_column - (token.span.column as isize - 1);
        for (i, line) in token.literal.split('\n').enumerate() {
            if i == 0 {
                self.out.push_str(line);
                continue;
            }
            self.out.push('\n');
            let line = line.trim_end_matches([' ', '\t']);
            if line.is_empty() {
                continue;
            }
            let indent = line.len() - line.trim_start_matches(' ').len();
            let indent = (indent as isize + shift).max(0) as usize;
            self.out.push_str(&" ".repeat(indent));
            self.out.push_str(line.trim_start_matches(' '));
        }
    }

    fn push_frame(&mut self, base: usize, switch: bool, after_vedimm: bool, paren: bool) {
        self.frames.push(Frame {
            base,
            switch,
            after_vedimm,
            paren,
            outer_stmt: self.stmt_level,
        });
    }

    fn pop_frame(&mut self) -> Option<Frame> {
        let frame = self.frames.pop();
        if let Some(f) = &frame {
            self.stmt_level = f.outer_stmt;
        }
        frame
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

    #[test]
    fn keeps_shebang_line() {
        let source = "#!/usr/bin/env fratm lancia\nsi (overo) {\nstamm a dì(1)\n}\n";
        assert_eq!(
            format(source).unwrap(),
            "#!/usr/bin/env fratm lancia\nsi (overo) {\n    stamm a dì(1)\n}\n"
        );
    }

    fn fmt(source: &str) -> String {
        format(source).unwrap()
    }

    #[test]
    fn test_moves_multiline_jsx_as_a_block() {
        let source = "facc A() {\n  piglie (\n<div>\n  <b>{x}</b>\n\n</div>\n  )\n}\n";
        let formatted = fmt(source);
        assert_eq!(
            formatted,
            "facc A() {\n    piglie (\n        <div>\n          <b>{x}</b>\n\n        </div>\n    )\n}\n"
        );
        assert_eq!(fmt(&formatted), formatted);
        // One-line JSX is kept as written
        assert_eq!(
            fmt("x = <i   class = \"a\">ciao</i>\n"),
            "x = <i   class = \"a\">ciao</i>\n"
        );
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

    #[test]
    fn test_indents_continuation_lines() {
        assert_eq!(
            fmt("chist è x = xs\n.map(v => {\npiglie v\n})\n.filter(f)\nstamm a dì(x +\n1)\n"),
            "chist è x = xs\n    .map(v => {\n        piglie v\n    })\n    .filter(f)\nstamm a dì(x +\n    1)\n"
        );
        // The block of a condition split over two lines belongs to `si`
        assert_eq!(
            fmt("si (a e\nb) {\nf()\n}\n"),
            "si (a e\n    b) {\n    f()\n}\n"
        );
    }
}
