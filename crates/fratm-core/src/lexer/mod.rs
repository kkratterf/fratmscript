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

pub use token::{
    lookup_keyword, JsxToken, JsxTokenAttribute, JsxTokenChild, JsxTokenValue, Span, Token,
    TokenKind,
};

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
    /// How many template interpolations enclose the current position
    template_depth: usize,
}

/// Why a JSX element could not be read.
enum JsxError {
    Message(String),
    /// An invalid token from inside a `{ … }`, reported as it is
    Nested(Token),
}

/// Templates nested deeper than this are an error rather than a stack
/// overflow (each `${` recurses into the lexer).
const MAX_TEMPLATE_DEPTH: usize = 50;

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
            template_depth: 0,
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
        join_continuation_lines(tokens)
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
                '.' if self.peek() == Some('.') && self.peek_next() == Some('.') => {
                    self.advance();
                    self.advance();
                    self.make_token(TokenKind::Ellipsis)
                }
                '.' => self.make_token(TokenKind::Dot),
                ':' => self.make_token(TokenKind::Colon),
                ';' => self.make_token(TokenKind::Semicolon),
                '?' => {
                    // `?.` unless it is `? .5` in a ternary
                    if self.peek() == Some('.')
                        && !self
                            .peek_next()
                            .map(|c| c.is_ascii_digit())
                            .unwrap_or(false)
                    {
                        self.advance();
                        self.make_token(TokenKind::QuestionDot)
                    } else if self.match_char('?') {
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
                '|' if self.match_char('=') => self.make_token(TokenKind::PipeEqual),
                '|' => self.make_token(TokenKind::Pipe),
                '&' if self.match_char('&') => {
                    if self.match_char('=') {
                        self.make_token(TokenKind::AndEqual)
                    } else {
                        self.make_token(TokenKind::And)
                    }
                }
                '&' if self.match_char('=') => self.make_token(TokenKind::AmpEqual),
                '&' => self.make_token(TokenKind::Amp),
                '^' if self.match_char('=') => self.make_token(TokenKind::CaretEqual),
                '^' => self.make_token(TokenKind::Caret),
                '~' => self.make_token(TokenKind::Tilde),

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
                '/' if self.regex_allowed() => self.scan_regex(),
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
                '<' if self.jsx_allowed() => self.scan_jsx(),
                '<' => {
                    if self.match_char('<') {
                        if self.match_char('=') {
                            self.make_token(TokenKind::LessLessEqual)
                        } else {
                            self.make_token(TokenKind::LessLess)
                        }
                    } else if self.match_char('=') {
                        self.make_token(TokenKind::LessEqual)
                    } else {
                        self.make_token(TokenKind::Less)
                    }
                }
                '>' => {
                    if self.match_char('>') {
                        if self.match_char('>') {
                            if self.match_char('=') {
                                self.make_token(TokenKind::GreaterGreaterGreaterEqual)
                            } else {
                                self.make_token(TokenKind::GreaterGreaterGreater)
                            }
                        } else if self.match_char('=') {
                            self.make_token(TokenKind::GreaterGreaterEqual)
                        } else {
                            self.make_token(TokenKind::GreaterGreater)
                        }
                    } else if self.match_char('=') {
                        self.make_token(TokenKind::GreaterEqual)
                    } else {
                        self.make_token(TokenKind::Greater)
                    }
                }

                '"' => self.scan_string(c),
                '`' => self.scan_template(),
                '\'' => self.scan_apostrophe_or_string(),
                c if c.is_ascii_digit() => self.scan_number(),
                c if is_ident_start(c) => self.scan_identifier(),
                '#' if self.peek().map(is_ident_start).unwrap_or(false) => {
                    while self.peek().map(is_ident_continue).unwrap_or(false) {
                        self.advance();
                    }
                    let name = self.source[self.token_start..self.position].to_string();
                    self.make_token(TokenKind::PrivateName(name))
                }

                '\n' => {
                    self.line += 1;
                    self.column = 1;
                    self.make_token(TokenKind::Newline)
                }

                _ => self.make_token(TokenKind::Invalid(format!("Unknown character '{c}'"))),
            },
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        // A `#!` line at the very start is a shebang: skip it like a comment.
        // `compile` re-emits it at the top of the generated JavaScript.
        if self.position == 0 && self.source.starts_with("#!") {
            while self.peek() != Some('\n') && self.peek().is_some() {
                self.advance();
            }
        }
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
    /// - `'ncopp` -> for...in (`TokenKind::Ncopp`)
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

        // `'ncopp` (for...in), only right after the loop variable: a string
        // never follows a name directly, so 'ncopp a tavola' still lexes as a string
        let after_name = matches!(self.prev_kind, Some(TokenKind::Identifier(_)));
        if let Some(after_ncopp) = rest.strip_prefix("ncopp").filter(|_| after_name) {
            if after_ncopp
                .chars()
                .next()
                .map(|c| c.is_whitespace())
                .unwrap_or(false)
            {
                for _ in 0.."ncopp".len() {
                    self.advance();
                }
                return self.make_token(TokenKind::Ncopp);
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

    /// Scans a template literal after the opening backtick. Text is kept raw
    /// (JS template escapes are the same), and each `${ … }` is lexed into its
    /// own token list, so nested templates and strings work.
    fn scan_template(&mut self) -> Token {
        let (start, line, column) = (self.token_start, self.token_line, self.token_column);
        let mut quasis = Vec::new();
        let mut expressions = Vec::new();
        let mut text = String::new();
        loop {
            match self.peek() {
                None => {
                    return self.make_token(TokenKind::Invalid("Unterminated template".to_string()))
                }
                Some('`') => {
                    self.advance();
                    quasis.push(text);
                    return self.make_token(TokenKind::Template {
                        quasis,
                        expressions,
                    });
                }
                Some('\\') => {
                    self.advance();
                    text.push('\\');
                    if let Some(c) = self.peek() {
                        self.advance_tracking_lines();
                        text.push(c);
                    }
                }
                Some('$') if self.peek_next() == Some('{') => {
                    if self.template_depth >= MAX_TEMPLATE_DEPTH {
                        return self.make_token(TokenKind::Invalid(
                            "Templates nested too deeply".to_string(),
                        ));
                    }
                    self.advance();
                    self.advance();
                    quasis.push(std::mem::take(&mut text));
                    self.template_depth += 1;
                    let tokens = self.scan_interpolation();
                    self.template_depth -= 1;
                    (self.token_start, self.token_line, self.token_column) = (start, line, column);
                    match tokens {
                        Ok(tokens) => expressions.push(tokens),
                        Err(Some(nested)) => return nested,
                        Err(None) => {
                            return self.make_token(TokenKind::Invalid(
                                "Unterminated template".to_string(),
                            ))
                        }
                    }
                }
                Some(c) => {
                    self.advance_tracking_lines();
                    text.push(c);
                }
            }
        }
    }

    /// Lexes the tokens of a `${ … }` up to its closing brace. Fails with
    /// `None` at the end of the input, or with a too deeply nested template.
    fn scan_interpolation(&mut self) -> Result<Vec<Token>, Option<Token>> {
        // An interpolation starts like an expression: `${/re/.source}`
        self.prev_kind = Some(TokenKind::LeftBrace);
        let mut tokens = Vec::new();
        let mut depth = 0usize;
        loop {
            let token = self.next_token();
            match token.kind {
                TokenKind::Eof => return Err(None),
                TokenKind::Invalid(ref message) if message.contains("nested too deeply") => {
                    return Err(Some(token))
                }
                TokenKind::LeftBrace => depth += 1,
                TokenKind::RightBrace if depth == 0 => return Ok(tokens),
                TokenKind::RightBrace => depth -= 1,
                _ => {}
            }
            self.prev_kind = Some(token.kind.clone());
            tokens.push(token);
        }
    }

    /// A `<` starts JSX where a value can start (where a regex could) and
    /// when a tag name or the `>` of a fragment follows right away:
    /// `piglie <b>…</b>`, `x = <></>`, but `a < b` and `i <n`.
    fn jsx_allowed(&mut self) -> bool {
        self.regex_allowed()
            && self
                .peek()
                .map(|c| is_ident_start(c) || c == '>')
                .unwrap_or(false)
    }

    /// Scans a JSX element or fragment after its `<` into one token.
    fn scan_jsx(&mut self) -> Token {
        let (start, line, column) = (self.token_start, self.token_line, self.token_column);
        let element = self.scan_jsx_element();
        (self.token_start, self.token_line, self.token_column) = (start, line, column);
        match element {
            Ok(element) => self.make_token(TokenKind::Jsx(Box::new(element))),
            Err(JsxError::Nested(token)) => token,
            Err(JsxError::Message(message)) => self.make_token(TokenKind::Invalid(message)),
        }
    }

    fn scan_jsx_element(&mut self) -> Result<JsxToken, JsxError> {
        if self.template_depth >= MAX_TEMPLATE_DEPTH {
            return Err(JsxError::Message("JSX nested too deeply".to_string()));
        }
        self.template_depth += 1;
        let element = self.scan_jsx_element_inner();
        self.template_depth -= 1;
        element
    }

    fn scan_jsx_element_inner(&mut self) -> Result<JsxToken, JsxError> {
        self.skip_jsx_space();
        if self.match_char('>') {
            let children = self.scan_jsx_children("<>")?;
            self.skip_jsx_space();
            if !self.match_char('>') {
                return Err(JsxError::Message(
                    "The fragment '<>' must be closed by '</>'".to_string(),
                ));
            }
            return Ok(JsxToken {
                name: None,
                attributes: Vec::new(),
                children: Some(children),
            });
        }
        let name = self.scan_jsx_name(true)?;
        let mut attributes = Vec::new();
        loop {
            self.skip_jsx_space();
            match self.peek() {
                Some('/') => {
                    self.advance();
                    if !self.match_char('>') {
                        return Err(JsxError::Message(format!(
                            "Expected '>' after '/' in '<{}'",
                            name
                        )));
                    }
                    return Ok(JsxToken {
                        name: Some(name),
                        attributes,
                        children: None,
                    });
                }
                Some('>') => {
                    self.advance();
                    let children = self.scan_jsx_children(&name)?;
                    self.skip_jsx_space();
                    let closing = if self.peek() == Some('>') {
                        String::new()
                    } else {
                        self.scan_jsx_name(true)?
                    };
                    if closing != name {
                        return Err(JsxError::Message(format!(
                            "'<{}>' is closed by '</{}>'",
                            name, closing
                        )));
                    }
                    self.skip_jsx_space();
                    if !self.match_char('>') {
                        return Err(JsxError::Message(format!(
                            "Expected '>' after '</{}'",
                            name
                        )));
                    }
                    return Ok(JsxToken {
                        name: Some(name),
                        attributes,
                        children: Some(children),
                    });
                }
                Some('{') => {
                    self.advance();
                    self.skip_jsx_space();
                    if !self.source[self.position..].starts_with("...") {
                        return Err(JsxError::Message(format!(
                            "Only '{{...props}}' can go in braces among the attributes of '<{}>'",
                            name
                        )));
                    }
                    for _ in 0..3 {
                        self.advance();
                    }
                    attributes.push(JsxTokenAttribute::Spread(self.scan_jsx_expression()?));
                }
                Some(c) if is_ident_start(c) => {
                    let (line, column, start) = (self.line, self.column, self.position);
                    let attr = self.scan_jsx_name(false)?;
                    let span = Span::new(start, self.position, line, column);
                    self.skip_jsx_space();
                    let value = if self.match_char('=') {
                        self.skip_jsx_space();
                        Some(self.scan_jsx_attribute_value(&attr)?)
                    } else {
                        None
                    };
                    attributes.push(JsxTokenAttribute::Named {
                        name: attr,
                        value,
                        span,
                    });
                }
                None => {
                    return Err(JsxError::Message(format!(
                        "The tag '<{}' is never closed",
                        name
                    )))
                }
                Some(c) => {
                    return Err(JsxError::Message(format!(
                        "Unexpected '{}' in the tag '<{}>'",
                        c, name
                    )))
                }
            }
        }
    }

    fn scan_jsx_attribute_value(&mut self, attr: &str) -> Result<JsxTokenValue, JsxError> {
        match self.peek() {
            Some(quote @ ('"' | '\'')) => {
                self.advance();
                let mut text = String::new();
                loop {
                    match self.peek() {
                        None => {
                            return Err(JsxError::Message(format!(
                                "Unterminated string in the attribute '{}'",
                                attr
                            )))
                        }
                        Some(c) if c == quote => {
                            self.advance();
                            return Ok(JsxTokenValue::String(text));
                        }
                        Some(c) => {
                            self.advance_tracking_lines();
                            text.push(c);
                        }
                    }
                }
            }
            Some('{') => {
                self.advance();
                let tokens = self.scan_jsx_expression()?;
                if tokens.is_empty() {
                    return Err(JsxError::Message(format!(
                        "The attribute '{}' has empty braces: write a value inside '{{ }}'",
                        attr
                    )));
                }
                Ok(JsxTokenValue::Expression(tokens))
            }
            Some('<') => {
                self.advance();
                Ok(JsxTokenValue::Element(Box::new(self.scan_jsx_element()?)))
            }
            _ => Err(JsxError::Message(format!(
                "The attribute '{}' needs a value in quotes or in braces: {}=\"…\" or {}={{…}}",
                attr, attr, attr
            ))),
        }
    }

    /// Text, `{ … }` and nested elements up to the `</` that closes `parent`.
    fn scan_jsx_children(&mut self, parent: &str) -> Result<Vec<JsxTokenChild>, JsxError> {
        let mut children = Vec::new();
        let mut text = String::new();
        loop {
            match self.peek() {
                None => {
                    return Err(JsxError::Message(if parent == "<>" {
                        "The fragment '<>' is never closed: add '</>'".to_string()
                    } else {
                        format!("'<{}>' is never closed: add '</{}>'", parent, parent)
                    }))
                }
                Some('<') => {
                    if !text.is_empty() {
                        children.push(JsxTokenChild::Text(std::mem::take(&mut text)));
                    }
                    self.advance();
                    if self.match_char('/') {
                        return Ok(children);
                    }
                    children.push(JsxTokenChild::Element(Box::new(self.scan_jsx_element()?)));
                }
                Some('{') => {
                    if !text.is_empty() {
                        children.push(JsxTokenChild::Text(std::mem::take(&mut text)));
                    }
                    let span = Span::new(self.position, self.position + 1, self.line, self.column);
                    self.advance();
                    let tokens = self.scan_jsx_expression()?;
                    if tokens.first().map(|t| &t.kind) == Some(&TokenKind::Ellipsis) {
                        return Err(JsxError::Message(
                            "'{...lista}' is not allowed among the children: write '{lista}'"
                                .to_string(),
                        ));
                    }
                    children.push(JsxTokenChild::Expression(tokens, span));
                }
                Some('}') => {
                    return Err(JsxError::Message(
                        "Unexpected '}' in the text of a JSX element: write {'}'}".to_string(),
                    ))
                }
                Some(c) => {
                    self.advance_tracking_lines();
                    text.push(c);
                }
            }
        }
    }

    /// The tokens of a `{ … }` inside JSX, after its `{`.
    fn scan_jsx_expression(&mut self) -> Result<Vec<Token>, JsxError> {
        self.template_depth += 1;
        let tokens = self.scan_interpolation();
        self.template_depth -= 1;
        match tokens {
            Ok(tokens) => Ok(tokens
                .into_iter()
                .filter(|t| t.kind != TokenKind::Newline)
                .collect()),
            Err(Some(nested)) => Err(JsxError::Nested(nested)),
            Err(None) => Err(JsxError::Message(
                "A '{' in JSX is never closed".to_string(),
            )),
        }
    }

    /// A tag or attribute name: letters, digits, `_`, `$` and `-`, with
    /// `.` (tags only) or one `:` between parts.
    fn scan_jsx_name(&mut self, tag: bool) -> Result<String, JsxError> {
        let start = self.position;
        if !self.peek().map(is_ident_start).unwrap_or(false) {
            return Err(JsxError::Message(match self.peek() {
                Some(c) => format!("Expected a tag name, not '{}'", c),
                None => "Expected a tag name".to_string(),
            }));
        }
        loop {
            while self
                .peek()
                .map(|c| is_ident_continue(c) || c == '-')
                .unwrap_or(false)
            {
                self.advance();
            }
            let joins = match self.peek() {
                Some('.') => tag,
                Some(':') => true,
                _ => false,
            };
            if joins && self.peek_next().map(is_ident_start).unwrap_or(false) {
                self.advance();
            } else {
                break;
            }
        }
        Ok(self.source[start..self.position].to_string())
    }

    fn skip_jsx_space(&mut self) {
        while self.peek().map(char::is_whitespace).unwrap_or(false) {
            self.advance_tracking_lines();
        }
    }

    /// A `/` starts a regex unless it follows something that ends an operand
    /// (then it is a division): `a / b`, `f() / 2`, but `x = /ab+/g`.
    fn regex_allowed(&self) -> bool {
        !matches!(
            self.prev_kind,
            Some(
                TokenKind::Identifier(_)
                    | TokenKind::PrivateName(_)
                    | TokenKind::Number(_)
                    | TokenKind::BigInt(_)
                    | TokenKind::String(_)
                    | TokenKind::Template { .. }
                    | TokenKind::Regex { .. }
                    | TokenKind::RightParen
                    | TokenKind::RightBracket
                    | TokenKind::RightBrace
                    | TokenKind::Overo
                    | TokenKind::Sfols
                    | TokenKind::Nisciun
                    | TokenKind::Boh
                    | TokenKind::Cos
                    | TokenKind::PlusPlus
                    | TokenKind::MinusMinus
            )
        )
    }

    /// Scans a regex literal after the opening `/`. The pattern is kept raw;
    /// a `/` inside a `[...]` class or escaped does not end it.
    fn scan_regex(&mut self) -> Token {
        let mut pattern = String::new();
        let mut in_class = false;
        loop {
            match self.peek() {
                None | Some('\n') => {
                    return self.make_token(TokenKind::Invalid("Unterminated regex".to_string()))
                }
                Some('/') if !in_class => {
                    self.advance();
                    break;
                }
                Some('\\') => {
                    self.advance();
                    pattern.push('\\');
                    match self.peek() {
                        None | Some('\n') => continue,
                        Some(c) => {
                            self.advance();
                            pattern.push(c);
                        }
                    }
                }
                Some(c) => {
                    if c == '[' {
                        in_class = true;
                    } else if c == ']' {
                        in_class = false;
                    }
                    self.advance();
                    pattern.push(c);
                }
            }
        }
        let mut flags = String::new();
        while let Some(c) = self.peek().filter(|c| is_ident_continue(*c)) {
            self.advance();
            flags.push(c);
        }
        let valid_flags = flags.chars().all(|c| "dgimsuvy".contains(c))
            && flags
                .chars()
                .enumerate()
                .all(|(i, c)| !flags[..i].contains(c));
        if !valid_flags {
            return self.make_token(TokenKind::Invalid(format!(
                "Invalid regex flags: {}",
                flags
            )));
        }
        self.make_token(TokenKind::Regex { pattern, flags })
    }

    fn advance_tracking_lines(&mut self) {
        if self.peek() == Some('\n') {
            self.line += 1;
            self.column = 0;
        }
        self.advance();
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

        // `10n`: a BigInt, only for integers (no `.5n`, `1e3n` or legacy octal)
        let integer = !self.source[self.token_start..self.position].contains(['.', 'e', 'E'])
            || radix.is_some();
        if valid
            && integer
            && self.peek() == Some('n')
            && !self
                .source
                .get(self.position + 1..)
                .and_then(|rest| rest.chars().next())
                .map(is_ident_continue)
                .unwrap_or(false)
        {
            let literal = self.source[self.token_start..self.position].to_string();
            self.advance(); // n
            let octal_like = radix.is_none() && literal.len() > 1 && literal.starts_with('0');
            if separators_ok(&literal, radix.unwrap_or(10)) && !octal_like {
                return self.make_token(TokenKind::BigInt(literal));
            }
            return self.make_token(TokenKind::Invalid(format!("Invalid number: {}n", literal)));
        }

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

        // `dint'a` (the `in` operator): one word with an apostrophe
        if literal == "dint" {
            let rest = &self.source[self.position..];
            let glued = rest
                .strip_prefix("'a")
                .map(|after| !after.chars().next().map(is_ident_continue).unwrap_or(false))
                .unwrap_or(false);
            if glued {
                self.advance();
                self.advance();
                return self.make_token(TokenKind::DintA);
            }
        }

        if let Some(keyword) = lookup_keyword(literal) {
            self.make_token(keyword)
        } else {
            self.make_token(TokenKind::Identifier(literal.to_string()))
        }
    }
}

/// Drops the line breaks that cannot end a statement, so that an expression
/// can go on over several lines: inside `( )` and `[ ]`, after an operator
/// that needs a right operand (`+`, `&&`, `=`, `?`, `=>`, …) and before a line
/// that starts with `.method()`, `?`, `:` or a binary operator.
fn join_continuation_lines(tokens: Vec<Token>) -> Vec<Token> {
    let mut out: Vec<Token> = Vec::with_capacity(tokens.len());
    // Open brackets; a `{` inside `( )` starts a block where lines matter again
    let mut open: Vec<TokenKind> = Vec::new();
    for (i, token) in tokens.iter().enumerate() {
        match token.kind {
            TokenKind::LeftParen | TokenKind::LeftBracket | TokenKind::LeftBrace => {
                open.push(token.kind.clone())
            }
            TokenKind::RightParen | TokenKind::RightBracket | TokenKind::RightBrace => {
                open.pop();
            }
            TokenKind::Newline => {
                let in_parens = matches!(
                    open.last(),
                    Some(TokenKind::LeftParen | TokenKind::LeftBracket)
                );
                let after = out
                    .iter()
                    .rev()
                    .find(|t| t.kind != TokenKind::Newline)
                    .is_some_and(continues_after);
                let before = tokens[i + 1..]
                    .iter()
                    .find(|t| t.kind != TokenKind::Newline)
                    .is_some_and(continues_before);
                if in_parens || after || before {
                    continue;
                }
            }
            _ => {}
        }
        out.push(token.clone());
    }
    out
}

/// Whether a line ending with `token` goes on to the next line.
pub fn continues_after(token: &Token) -> bool {
    use TokenKind::*;
    match &token.kind {
        // `e` / `o` / `pure` are also names: only the symbols continue
        And | Or => token.literal == "&&" || token.literal == "||",
        Plus
        | Minus
        | Star
        | Slash
        | Percent
        | StarStar
        | Amp
        | Pipe
        | Caret
        | LessLess
        | GreaterGreater
        | GreaterGreaterGreater
        | EqualEqual
        | EqualEqualEqual
        | BangEqual
        | BangEqualEqual
        | Less
        | Greater
        | LessEqual
        | GreaterEqual
        | QuestionQuestion
        | DintA
        | Equal
        | PlusEqual
        | MinusEqual
        | StarEqual
        | SlashEqual
        | PercentEqual
        | StarStarEqual
        | QuestionQuestionEqual
        | OrEqual
        | AndEqual
        | AmpEqual
        | PipeEqual
        | CaretEqual
        | LessLessEqual
        | GreaterGreaterEqual
        | GreaterGreaterGreaterEqual
        | Arrow
        | Question
        | Colon
        | QuestionDot
        | Dot => true,
        _ => false,
    }
}

/// Whether a line starting with `token` continues the previous one: no
/// statement can start with these.
pub fn continues_before(token: &Token) -> bool {
    use TokenKind::*;
    match &token.kind {
        And | Or => token.literal == "&&" || token.literal == "||",
        Dot
        | QuestionDot
        | Question
        | Colon
        | QuestionQuestion
        | Percent
        | StarStar
        | Amp
        | Pipe
        | Caret
        | LessLess
        | GreaterGreater
        | GreaterGreaterGreater
        | EqualEqual
        | EqualEqualEqual
        | BangEqual
        | BangEqualEqual
        | Less
        | Greater
        | LessEqual
        | GreaterEqual
        | DintA
        | Arrow => true,
        _ => false,
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

    #[test]
    fn test_template_interpolation_tokens_have_source_positions() {
        let tokens = Lexer::new("tien x = `a\n${ nome }`").tokenize();
        let TokenKind::Template {
            quasis,
            expressions,
        } = &tokens[3].kind
        else {
            panic!("{:?}", tokens[3].kind)
        };
        assert_eq!(quasis, &vec!["a\n".to_string(), String::new()]);
        let nome = &expressions[0][0];
        assert!(matches!(&nome.kind, TokenKind::Identifier(n) if n == "nome"));
        assert_eq!((nome.span.line, nome.span.column), (2, 4));
        // The template token itself keeps its start position
        assert_eq!((tokens[3].span.line, tokens[3].span.column), (1, 10));
    }

    fn kinds(source: &str) -> Vec<TokenKind> {
        Lexer::new(source)
            .tokenize()
            .into_iter()
            .map(|t| t.kind)
            .collect()
    }

    #[test]
    fn test_jsx_starts_only_where_a_value_can() {
        assert!(matches!(kinds("x = <b>ciao</b>")[2], TokenKind::Jsx(_)));
        assert!(matches!(kinds("piglie <></>")[1], TokenKind::Jsx(_)));
        assert!(matches!(kinds("f(<br />)")[2], TokenKind::Jsx(_)));
        // Comparisons stay comparisons
        assert_eq!(kinds("a <b")[1], TokenKind::Less);
        assert_eq!(kinds("1 < 2")[1], TokenKind::Less);
        assert_eq!(kinds("f() <x")[3], TokenKind::Less);
        assert_eq!(kinds("x = a < b")[3], TokenKind::Less);
        assert_eq!(kinds("x = 1 <<2")[3], TokenKind::LessLess);
    }

    #[test]
    fn test_jsx_element_keeps_attributes_children_and_inner_tokens() {
        let tokens = Lexer::new(
            "x = <Menu.Voce aria-label='a' attivo {...resto} n={1 + 2}>\n  Uè {nomme}<i/></Menu.Voce>",
        )
        .tokenize();
        let TokenKind::Jsx(element) = &tokens[2].kind else {
            panic!("{:?}", tokens[2].kind)
        };
        assert_eq!(element.name.as_deref(), Some("Menu.Voce"));
        let names: Vec<_> = element
            .attributes
            .iter()
            .map(|a| match a {
                JsxTokenAttribute::Named { name, .. } => name.as_str(),
                JsxTokenAttribute::Spread(_) => "...",
            })
            .collect();
        assert_eq!(names, ["aria-label", "attivo", "...", "n"]);
        let children = element.children.as_ref().unwrap();
        assert_eq!(children.len(), 3);
        assert!(matches!(&children[0], JsxTokenChild::Text(t) if t == "\n  Uè "));
        let JsxTokenChild::Expression(inner, _) = &children[1] else {
            panic!("{:?}", children[1])
        };
        assert!(matches!(&inner[0].kind, TokenKind::Identifier(n) if n == "nomme"));
        assert_eq!((inner[0].span.line, inner[0].span.column), (2, 7));
        assert!(matches!(&children[2], JsxTokenChild::Element(e) if e.children.is_none()));
        // The token covers the whole element and the next token follows it
        assert!(tokens[2].literal.ends_with("</Menu.Voce>"));
        assert_eq!(tokens[3].kind, TokenKind::Eof);
    }

    #[test]
    fn test_jsx_errors_are_invalid_tokens() {
        for source in [
            "x = <div>",
            "x = <a></b>",
            "x = <a b=>",
            "x = <a>{b</a>",
            "x = <a>}</a>",
            "x = <a {b} />",
        ] {
            let tokens = Lexer::new(source).tokenize();
            assert!(
                matches!(tokens[2].kind, TokenKind::Invalid(_)),
                "{source}: {:?}",
                tokens[2].kind
            );
        }
    }

    #[test]
    fn test_deeply_nested_jsx_is_an_error_not_a_crash() {
        let source = format!("x = {}{}", "<a>".repeat(200), "</a>".repeat(200));
        let tokens = Lexer::new(&source).tokenize();
        assert!(
            matches!(&tokens[2].kind, TokenKind::Invalid(m) if m.contains("nested too deeply"))
        );
    }
}
