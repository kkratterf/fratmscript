//! # FratmScript Lexer
//!
//! This module implements the lexical analyzer (lexer) that converts
//! FratmScript source code into a sequence of tokens.
//!
//! ## How It Works
//!
//! The lexer reads the source code character by character and recognizes:
//!
//! - **Keywords**: `chist`, `è`, `tien`, `facc`, `piglie`, `si`, `sinnò`, etc.
//! - **Operators**: `+`, `-`, `*`, `/`, `==`, `===`, `e`, `o`, `no`, etc.
//! - **Literals**: numbers (`42`, `3.14`), strings (`"ciao"`), booleans
//! - **Punctuation**: `(`, `)`, `{`, `}`, `[`, `]`, `,`, `.`, etc.
//!
//! ## Example
//!
//! ```rust
//! use fratm_core::lexer::{Lexer, TokenKind};
//!
//! let mut lexer = Lexer::new("chist è x = 42");
//! let tokens = lexer.tokenize();
//!
//! assert!(matches!(tokens[0].kind, TokenKind::Chist));
//! assert!(matches!(tokens[1].kind, TokenKind::E));
//! assert!(matches!(tokens[2].kind, TokenKind::Identifier(_)));
//! assert!(matches!(tokens[3].kind, TokenKind::Equal));
//! assert!(matches!(tokens[4].kind, TokenKind::Number(42.0)));
//! ```
//!
//! ## Position Tracking
//!
//! Each token includes information about its position in the source code
//! (line, column, offset). This enables precise error messages.

mod token;

pub use token::{lookup_keyword, Span, Token, TokenKind};

/// Lexical analyzer for FratmScript.
///
/// The lexer converts source code into a sequence of [`Token`]s.
/// It tracks the current position in the source to generate
/// accurate span information for each token.
///
/// # Example
///
/// ```rust
/// use fratm_core::lexer::Lexer;
///
/// let mut lexer = Lexer::new("stamm a dì(42)");
/// let tokens = lexer.tokenize();
///
/// // Tokens include: Stamm, A, Di, LeftParen, Number, RightParen, Eof
/// assert_eq!(tokens.len(), 7);
/// ```
///
/// # Error Handling
///
/// Unrecognized tokens are emitted as `TokenKind::Invalid`:
///
/// ```rust
/// use fratm_core::lexer::{Lexer, TokenKind};
///
/// let mut lexer = Lexer::new("@"); // Invalid character
/// let tokens = lexer.tokenize();
/// assert!(matches!(tokens[0].kind, TokenKind::Invalid(_)));
/// ```
pub struct Lexer<'a> {
    /// Source code to analyze
    source: &'a str,
    /// Character iterator with indices
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    /// Current position in source (byte offset)
    position: usize,
    /// Current line (1-indexed)
    line: usize,
    /// Current column (1-indexed)
    column: usize,
    /// Start of current token (byte offset)
    token_start: usize,
    /// Line where current token starts
    token_line: usize,
    /// Column where current token starts
    token_column: usize,
    /// Kind of the last token emitted, used to disambiguate `'e`
    prev_kind: Option<TokenKind>,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer for the specified source code.
    ///
    /// # Arguments
    ///
    /// * `source` - The FratmScript source code to tokenize
    ///
    /// # Example
    ///
    /// ```rust
    /// use fratm_core::lexer::Lexer;
    ///
    /// let lexer = Lexer::new("chist è saluto = \"Uè!\"");
    /// ```
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
            position: 0,
            line: 1,
            column: 1,
            token_start: 0,
            token_line: 1,
            token_column: 1,
            prev_kind: None,
        }
    }

    /// Tokenizes the entire source code and returns all tokens.
    ///
    /// Continues reading tokens until reaching end of file (EOF).
    /// The last token will always be `TokenKind::Eof`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fratm_core::lexer::{Lexer, TokenKind};
    ///
    /// let mut lexer = Lexer::new("42 + 8");
    /// let tokens = lexer.tokenize();
    ///
    /// // Number(42), Plus, Number(8), Eof
    /// assert_eq!(tokens.len(), 4);
    /// assert!(matches!(tokens.last().unwrap().kind, TokenKind::Eof));
    /// ```
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            let is_eof = token.kind == TokenKind::Eof;
            self.prev_kind = Some(token.kind.clone());
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        tokens
    }

    /// Reads and returns the next token from the source.
    ///
    /// This method advances the lexer in the source and returns the
    /// next token. It automatically skips whitespace and comments.
    ///
    /// # Token Types
    ///
    /// - Neapolitan keywords (`chist`, `tien`, `facc`, etc.)
    /// - Operators (`+`, `-`, `*`, `/`, `==`, `===`, etc.)
    /// - Logical operators (`e`, `o`, `no`)
    /// - Literals (numbers, strings)
    /// - Punctuation (`(`, `)`, `{`, `}`, etc.)
    /// - Identifiers (variable/function names)
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();
        self.mark_token_start();

        match self.advance() {
            None => self.make_token(TokenKind::Eof),
            Some(c) => match c {
                '(' => self.make_token(TokenKind::LeftParen),
                ')' => self.make_token(TokenKind::RightParen),
                '{' => self.make_token(TokenKind::LeftBrace),
                '}' => self.make_token(TokenKind::RightBrace),
                '[' => self.make_token(TokenKind::LeftBracket),
                ']' => self.make_token(TokenKind::RightBracket),
                ',' => self.make_token(TokenKind::Comma),
                '.' => self.make_token(TokenKind::Dot),
                ':' => self.make_token(TokenKind::Colon),
                ';' => self.make_token(TokenKind::Semicolon),
                '?' => {
                    if self.match_char('?') {
                        if self.match_char('=') {
                            self.make_token(TokenKind::QuestionQuestionEqual)
                        } else {
                            self.make_token(TokenKind::QuestionQuestion)
                        }
                    } else {
                        self.make_token(TokenKind::Question)
                    }
                }
                '|' if self.match_char('|') => {
                    if self.match_char('=') {
                        self.make_token(TokenKind::OrEqual)
                    } else {
                        self.make_token(TokenKind::Or)
                    }
                }
                '&' if self.match_char('&') => {
                    if self.match_char('=') {
                        self.make_token(TokenKind::AndEqual)
                    } else {
                        self.make_token(TokenKind::And)
                    }
                }

                '+' => {
                    if self.match_char('+') {
                        self.make_token(TokenKind::PlusPlus)
                    } else if self.match_char('=') {
                        self.make_token(TokenKind::PlusEqual)
                    } else {
                        self.make_token(TokenKind::Plus)
                    }
                }
                '-' => {
                    if self.match_char('-') {
                        self.make_token(TokenKind::MinusMinus)
                    } else if self.match_char('=') {
                        self.make_token(TokenKind::MinusEqual)
                    } else {
                        self.make_token(TokenKind::Minus)
                    }
                }
                '*' => {
                    if self.match_char('*') {
                        if self.match_char('=') {
                            self.make_token(TokenKind::StarStarEqual)
                        } else {
                            self.make_token(TokenKind::StarStar)
                        }
                    } else if self.match_char('=') {
                        self.make_token(TokenKind::StarEqual)
                    } else {
                        self.make_token(TokenKind::Star)
                    }
                }
                '/' => {
                    if self.match_char('=') {
                        self.make_token(TokenKind::SlashEqual)
                    } else {
                        self.make_token(TokenKind::Slash)
                    }
                }
                '%' => {
                    if self.match_char('=') {
                        self.make_token(TokenKind::PercentEqual)
                    } else {
                        self.make_token(TokenKind::Percent)
                    }
                }

                '=' => {
                    if self.match_char('=') {
                        if self.match_char('=') {
                            self.make_token(TokenKind::EqualEqualEqual)
                        } else {
                            self.make_token(TokenKind::EqualEqual)
                        }
                    } else if self.match_char('>') {
                        self.make_token(TokenKind::Arrow)
                    } else {
                        self.make_token(TokenKind::Equal)
                    }
                }
                '!' => {
                    if self.match_char('=') {
                        if self.match_char('=') {
                            self.make_token(TokenKind::BangEqualEqual)
                        } else {
                            self.make_token(TokenKind::BangEqual)
                        }
                    } else {
                        // ! alone is now supported as NOT operator
                        self.make_token(TokenKind::Not)
                    }
                }
                '<' => {
                    if self.match_char('=') {
                        self.make_token(TokenKind::LessEqual)
                    } else {
                        self.make_token(TokenKind::Less)
                    }
                }
                '>' => {
                    if self.match_char('=') {
                        self.make_token(TokenKind::GreaterEqual)
                    } else {
                        self.make_token(TokenKind::Greater)
                    }
                }

                '"' => self.scan_string(c),
                '\'' => self.scan_apostrophe_or_string(),
                c if c.is_ascii_digit() => self.scan_number(),
                c if is_ident_start(c) => self.scan_identifier(),

                '\n' => {
                    self.line += 1;
                    self.column = 1;
                    self.make_token(TokenKind::Newline)
                }

                _ => self.make_token(TokenKind::Invalid(c.to_string())),
            },
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek() {
                Some(' ') | Some('\t') | Some('\r') => {
                    self.advance();
                }
                Some('/') => {
                    if self.peek_next() == Some('/') {
                        while self.peek() != Some('\n') && self.peek().is_some() {
                            self.advance();
                        }
                    } else if self.peek_next() == Some('*') {
                        self.advance();
                        self.advance();
                        while !(self.peek() == Some('*') && self.peek_next() == Some('/')) {
                            if self.peek().is_none() {
                                break;
                            }
                            if self.peek() == Some('\n') {
                                self.line += 1;
                                self.column = 0;
                            }
                            self.advance();
                        }
                        self.advance();
                        self.advance();
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    fn mark_token_start(&mut self) {
        self.token_start = self.position;
        self.token_line = self.line;
        self.token_column = self.column;
    }

    fn advance(&mut self) -> Option<char> {
        if let Some((pos, c)) = self.chars.next() {
            self.position = pos + c.len_utf8();
            self.column += 1;
            Some(c)
        } else {
            None
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|(_, c)| *c)
    }

    fn peek_next(&self) -> Option<char> {
        let mut iter = self.source[self.position..].chars();
        iter.next();
        iter.next()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn make_token(&self, kind: TokenKind) -> Token {
        let literal = self.source[self.token_start..self.position].to_string();
        Token::new(
            kind,
            Span::new(
                self.token_start,
                self.position,
                self.token_line,
                self.token_column,
            ),
            literal,
        )
    }

    /// Handles apostrophe-based OOP aliases before falling back to a normal string.
    ///
    /// Supported forms:
    /// - `'e`   -> extends connector (`TokenKind::De`)
    /// - `'o pate` -> super (`TokenKind::OPate`)
    fn scan_apostrophe_or_string(&mut self) -> Token {
        let rest = &self.source[self.position..];

        // `'e` (used in "figlio 'e Padre"), only right after `figlio` so that
        // strings like 'e poi' still lex as strings
        let after_figlio = matches!(self.prev_kind, Some(TokenKind::Figlio));
        if let Some(after_e) = rest.strip_prefix('e').filter(|_| after_figlio) {
            if after_e
                .chars()
                .next()
                .map(|c| c.is_whitespace())
                .unwrap_or(false)
            {
                self.advance(); // consume 'e'
                return self.make_token(TokenKind::De);
            }
        }

        // `'o pate` used for super calls
        if let Some(after_o) = rest.strip_prefix('o') {
            let spaces_len = after_o.len() - after_o.trim_start_matches([' ', '\t']).len();
            let trimmed = &after_o[spaces_len..];
            if spaces_len > 0 {
                if let Some(after_pate) = trimmed.strip_prefix("pate") {
                    // super is always called or accessed: `'o pate(...)` / `'o pate.x`
                    if matches!(after_pate.chars().next(), Some('(') | Some('.')) {
                        let consumed = 1 + spaces_len + "pate".len();
                        for _ in 0..consumed {
                            self.advance();
                        }
                        return self.make_token(TokenKind::OPate);
                    }
                }
            }
        }

        // Not an apostrophe keyword: parse as a single-quoted string.
        self.scan_string('\'')
    }

    fn scan_string(&mut self, quote: char) -> Token {
        let mut value = String::new();

        while let Some(c) = self.peek() {
            if c == quote {
                self.advance();
                return self.make_token(TokenKind::String(value));
            }
            if c == '\n' {
                self.line += 1;
                self.column = 0;
            }
            if c == '\\' {
                self.advance();
                match self.peek() {
                    Some('n') => {
                        self.advance();
                        value.push('\n');
                    }
                    Some('t') => {
                        self.advance();
                        value.push('\t');
                    }
                    Some('r') => {
                        self.advance();
                        value.push('\r');
                    }
                    Some('\\') => {
                        self.advance();
                        value.push('\\');
                    }
                    Some('"') => {
                        self.advance();
                        value.push('"');
                    }
                    Some('\'') => {
                        self.advance();
                        value.push('\'');
                    }
                    Some(c) => {
                        self.advance();
                        value.push(c);
                    }
                    None => break,
                }
            } else {
                self.advance();
                value.push(c);
            }
        }

        self.make_token(TokenKind::Invalid("Unterminated string".to_string()))
    }

    /// Scans a numeric literal with the same shapes JavaScript accepts:
    /// decimals (`42`, `3.14`), exponents (`1e3`, `2.5E-4`), hex/octal/binary
    /// (`0x1F`, `0o17`, `0b101`) and `_` separators (`1_000`).
    fn scan_number(&mut self) -> Token {
        let first = &self.source[self.token_start..self.position];
        let radix = match (first, self.peek()) {
            ("0", Some('x' | 'X')) => Some(16),
            ("0", Some('o' | 'O')) => Some(8),
            ("0", Some('b' | 'B')) => Some(2),
            _ => None,
        };

        let valid = if let Some(radix) = radix {
            self.advance(); // consume x/o/b
            let digits_start = self.position;
            self.scan_digits(radix);
            self.position > digits_start
        } else {
            self.scan_digits(10);
            if self.peek() == Some('.')
                && self
                    .peek_next()
                    .map(|c| c.is_ascii_digit())
                    .unwrap_or(false)
            {
                self.advance();
                self.scan_digits(10);
            }
            let mut ok = true;
            if matches!(self.peek(), Some('e' | 'E')) {
                let mut rest = self.source[self.position..].chars().skip(1);
                let next = rest.next();
                let after_sign = if matches!(next, Some('+' | '-')) {
                    rest.next()
                } else {
                    next
                };
                if after_sign.map(|c| c.is_ascii_digit()).unwrap_or(false) {
                    self.advance(); // e/E
                    if matches!(self.peek(), Some('+' | '-')) {
                        self.advance();
                    }
                    self.scan_digits(10);
                } else {
                    ok = false;
                }
            }
            ok
        };

        // An identifier character glued to a number (`3abc`, `0x1G`, `1e`) is an error, as in JS.
        let glued = self.peek().map(is_ident_continue).unwrap_or(false);
        while self.peek().map(is_ident_continue).unwrap_or(false) {
            self.advance();
        }

        let literal = &self.source[self.token_start..self.position];
        let invalid = || TokenKind::Invalid(format!("Invalid number: {}", literal));
        if !valid || glued || !separators_ok(literal, radix.unwrap_or(10)) {
            return self.make_token(invalid());
        }
        // JS rejects legacy octal like `0123` in strict code: refuse it rather than guess.
        if radix.is_none()
            && literal.len() > 1
            && literal.starts_with('0')
            && literal.as_bytes()[1].is_ascii_digit()
        {
            return self.make_token(invalid());
        }

        let clean: String = literal.chars().filter(|&c| c != '_').collect();
        let value = match radix {
            Some(radix) => parse_radix(&clean[2..], radix),
            None => clean.parse::<f64>().ok(),
        };
        match value {
            Some(n) => self.make_token(TokenKind::Number(n)),
            None => self.make_token(invalid()),
        }
    }

    /// Consumes digits of the given radix and `_` separators.
    fn scan_digits(&mut self, radix: u32) {
        while self
            .peek()
            .map(|c| c.is_digit(radix) || c == '_')
            .unwrap_or(false)
        {
            self.advance();
        }
    }

    fn scan_identifier(&mut self) -> Token {
        while self.peek().map(is_ident_continue).unwrap_or(false) {
            self.advance();
        }

        let literal = &self.source[self.token_start..self.position];

        if let Some(keyword) = lookup_keyword(literal) {
            self.make_token(keyword)
        } else {
            self.make_token(TokenKind::Identifier(literal.to_string()))
        }
    }
}

/// A `_` separator must sit between two digits (`1_000` ok, `1__0`, `1_`, `0x_1`, `1_.5`, `1_e3` not).
fn separators_ok(literal: &str, radix: u32) -> bool {
    let chars: Vec<char> = literal.chars().collect();
    let is_digit = |j: usize| chars.get(j).map(|c| c.is_digit(radix)).unwrap_or(false);
    chars
        .iter()
        .enumerate()
        .all(|(i, &c)| c != '_' || (i > 0 && is_digit(i - 1) && is_digit(i + 1)))
}

/// Parses hex/octal/binary digits, rounding like JS does for values above 2^53.
fn parse_radix(digits: &str, radix: u32) -> Option<f64> {
    if digits.is_empty() {
        return None;
    }
    if let Ok(n) = u128::from_str_radix(digits, radix) {
        return Some(n as f64);
    }
    digits.chars().try_fold(0f64, |acc, c| {
        c.to_digit(radix).map(|d| acc * radix as f64 + d as f64)
    })
}

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("chist è tien facc piglie");
        let tokens = lexer.tokenize();

        assert!(matches!(tokens[0].kind, TokenKind::Chist));
        assert!(matches!(tokens[1].kind, TokenKind::E));
        assert!(matches!(tokens[2].kind, TokenKind::Tien));
        assert!(matches!(tokens[3].kind, TokenKind::Facc));
        assert!(matches!(tokens[4].kind, TokenKind::Piglie));
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = Lexer::new("\"Uè!\"");
        let tokens = lexer.tokenize();
        assert!(matches!(&tokens[0].kind, TokenKind::String(s) if s == "Uè!"));
    }

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("42 2.75");
        let tokens = lexer.tokenize();
        assert!(matches!(tokens[0].kind, TokenKind::Number(n) if n == 42.0));
        assert!(matches!(tokens[1].kind, TokenKind::Number(n) if n == 2.75));
    }

    #[test]
    fn test_js_number_shapes_are_single_tokens() {
        let cases = [
            ("0x1F", 31.0),
            ("0XfF", 255.0),
            ("0o17", 15.0),
            ("0b101", 5.0),
            ("1e3", 1000.0),
            ("1E+2", 100.0),
            ("2.5e-1", 0.25),
            ("1_000", 1000.0),
            ("0xFF_FF", 65535.0),
            ("0", 0.0),
        ];
        for (src, expected) in cases {
            let tokens = Lexer::new(src).tokenize();
            assert!(
                matches!(tokens[0].kind, TokenKind::Number(n) if n == expected),
                "{} -> {:?}",
                src,
                tokens[0].kind
            );
            assert!(
                matches!(tokens[1].kind, TokenKind::Eof),
                "{} split into several tokens",
                src
            );
        }
    }

    #[test]
    fn test_apostrophe_oop_keywords() {
        let mut lexer = Lexer::new("figlio 'e Padre { 'o pate(nome) }");
        let tokens = lexer.tokenize();

        assert!(matches!(tokens[0].kind, TokenKind::Figlio));
        assert!(matches!(tokens[1].kind, TokenKind::De));
        assert!(matches!(tokens[2].kind, TokenKind::Identifier(_)));
        assert!(matches!(tokens[4].kind, TokenKind::OPate));
    }

    #[test]
    fn test_apostrophe_strings_are_not_oop_keywords() {
        let mut lexer = Lexer::new("'e poi' 'o pate mio' 'e'");
        let tokens = lexer.tokenize();

        assert!(matches!(&tokens[0].kind, TokenKind::String(s) if s == "e poi"));
        assert!(matches!(&tokens[1].kind, TokenKind::String(s) if s == "o pate mio"));
        assert!(matches!(&tokens[2].kind, TokenKind::String(s) if s == "e"));
    }
}
