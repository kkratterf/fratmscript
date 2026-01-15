//! # Token Definitions for FratmScript
//!
//! This module defines all token types recognized by the lexer
//! and structures for tracking positions in source code.
//!
//! ## Token Categories
//!
//! ### Neapolitan Keywords
//! - **Variables**: `chist` (const), `è`, `tien` (let)
//! - **Functions**: `facc` (function), `piglie` (return)
//! - **Control**: `si` (if), `sinnò` (else), `pe` (for), `mentre` (while)
//! - **Classes**: `na` (class), `famiglie`, `nu` (new), `bell`
//! - **Values**: `overo` (true), `sfòls` (false), `nisciun` (null), `boh` (undefined)
//! - **Async**: `mo` (async), `vir`, `aspett` (await)
//!
//! ### Operators
//! - **Arithmetic**: `+`, `-`, `*`, `/`, `%`, `**`
//! - **Comparison**: `==`, `===`, `!=`, `!==`, `<`, `>`, `<=`, `>=`
//! - **Logical**: `e` (and), `o` (or), `no` (not)
//! - **Assignment**: `=`, `+=`, `-=`, `*=`, `/=`

use std::fmt;
use serde::{Serialize, Deserialize};

/// Position of a token in the source code.
///
/// Tracks both byte offset and line/column position for
/// human-readable error messages and source map generation.
///
/// # Fields
///
/// * `start` - Byte offset from file start
/// * `end` - Byte offset of token end
/// * `line` - Line number (1-indexed)
/// * `column` - Column number (1-indexed)
///
/// # Example
///
/// ```rust
/// use fratm_core::lexer::Span;
///
/// let span = Span::new(0, 5, 1, 1);
/// assert_eq!(span.line, 1);
/// assert_eq!(span.column, 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    /// Byte offset from file start (0-indexed)
    pub start: usize,
    /// Byte offset of token end (exclusive)
    pub end: usize,
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
}

impl Span {
    /// Creates a new span with the specified positions.
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self { start, end, line, column }
    }

    /// Merges two spans creating one that covers both.
    ///
    /// Useful for creating spans that cover entire compound expressions.
    pub fn merge(&self, other: &Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            line: self.line.min(other.line),
            column: if self.line <= other.line { self.column } else { other.column },
        }
    }
}

impl Default for Span {
    fn default() -> Self {
        Self { start: 0, end: 0, line: 1, column: 1 }
    }
}

/// A token with its position in source.
///
/// Represents a single lexical element recognized by the lexer,
/// complete with position information and original text.
///
/// # Example
///
/// ```rust
/// use fratm_core::lexer::{Token, TokenKind, Span};
///
/// let token = Token::new(
///     TokenKind::Chist,
///     Span::new(0, 5, 1, 1),
///     "chist".to_string()
/// );
/// assert_eq!(token.literal, "chist");
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    /// The token type (keyword, operator, literal, etc.)
    pub kind: TokenKind,
    /// Position of the token in source
    pub span: Span,
    /// Original text of the token as it appears in source
    pub literal: String,
}

impl Token {
    /// Creates a new token with type, position, and literal text.
    pub fn new(kind: TokenKind, span: Span, literal: String) -> Self {
        Self { kind, span, literal }
    }
}

/// All token types recognized by the FratmScript lexer.
///
/// Tokens are organized in categories:
/// - **Keywords**: reserved words in Neapolitan
/// - **Operators**: arithmetic, logical, comparison
/// - **Punctuation**: parentheses, separators
/// - **Literals**: identifiers, strings, numbers
/// - **Special**: newline, EOF, invalid tokens
///
/// # Main Keywords
///
/// | Token | Neapolitan | JavaScript |
/// |-------|------------|------------|
/// | `Chist` + `E` | `chist è` | `const` |
/// | `Tien` | `tien` | `let` |
/// | `Facc` | `facc` | `function` |
/// | `Piglie` | `piglie` | `return` |
/// | `Si` | `si` | `if` |
/// | `Sinno` | `sinnò` | `else` |
/// | `Pe` | `pe` | `for` |
/// | `Mentre` + `Che` | `mentre che` | `while` |
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenKind {
    // === Keywords ===
    Chist,          // const (part 1)
    E,              // const (part 2: "è")
    Tien,           // let
    Facc,           // function
    Piglie,         // return
    Si,             // if
    Sinno,          // else
    Pe,             // for (part 1)
    Ogni,           // for (part 2)
    Mentre,         // while (part 1)
    Che,            // while (part 2)
    Overo,          // true
    Sfols,          // false
    Nisciun,        // null
    Boh,            // undefined
    Stamm,          // console.log (part 1)
    A,              // console.log (part 2)
    Di,             // console.log (part 3: "dì")
    Mo,             // async (part 1)
    Vir,            // async (part 2)
    Aspett,         // await
    Pruvamm,        // try
    Schiatta,       // catch (in "e si schiatta")
    Iett,           // throw
    Nu,             // new (part 1)
    Bell,           // new (part 2)
    Na,             // class (part 1)
    Famiglie,       // class (part 2)
    Stu,            // this (part 1)
    Cos,            // this (part 2)
    Chiamm,         // import
    Da,             // from
    Mann,           // export (part 1)
    For,            // export (part 2)
    Predefinit,     // default
    Rompe,          // break
    Salta,          // continue

    // === New Keywords (Goliardic) ===
    Vir2,           // switch (part 1) - "vir" (see)
    Caso,           // case - "e che"
    SinnoFa,        // default in switch - "sinnò fa"
    Fisso,          // static
    Figlio,         // extends (part 1) - "figlio"
    De,             // extends (part 2) - "'e" (of)
    OPate,          // super - "'o pate" (the father)
    CheE,           // typeof - "chè è" (what is)
    EUno,           // instanceof (part 1) - "è uno"
    Leva,           // delete
    DintA,          // in - "dint'a" (inside)
    Caccia,         // yield
    Fermete,        // debugger
    Scrive,         // console.error (part 1)
    Avvis,          // console.warn (part 1)

    // === Logical Operators ===
    And,            // && - "e"
    Or,             // || - "o"
    Not,            // ! - "no"
    Manco,          // ! (alias) - "manco" (not even)
    Pure,           // && (alias) - "pure" (also)

    // === Operators ===
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    StarStar,

    // === Comparison ===
    EqualEqual,
    EqualEqualEqual,
    BangEqual,
    BangEqualEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    // === Assignment ===
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,

    // === Punctuation ===
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Colon,
    Semicolon,
    Question,
    Arrow,

    // === Literals ===
    Identifier(String),
    String(String),
    Number(f64),

    // === Special ===
    Newline,
    Eof,
    Invalid(String),
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Chist => write!(f, "chist"),
            TokenKind::E => write!(f, "è"),
            TokenKind::Tien => write!(f, "tien"),
            TokenKind::Facc => write!(f, "facc"),
            TokenKind::Piglie => write!(f, "piglie"),
            TokenKind::Si => write!(f, "si"),
            TokenKind::Sinno => write!(f, "sinnò"),
            TokenKind::Pe => write!(f, "pe"),
            TokenKind::Ogni => write!(f, "ogni"),
            TokenKind::Mentre => write!(f, "mentre"),
            TokenKind::Che => write!(f, "che"),
            TokenKind::Overo => write!(f, "overo"),
            TokenKind::Sfols => write!(f, "sfòls"),
            TokenKind::Nisciun => write!(f, "nisciun"),
            TokenKind::Boh => write!(f, "boh"),
            TokenKind::Stamm => write!(f, "stamm"),
            TokenKind::A => write!(f, "a"),
            TokenKind::Di => write!(f, "dì"),
            TokenKind::Mo => write!(f, "mo"),
            TokenKind::Vir => write!(f, "vir"),
            TokenKind::Aspett => write!(f, "aspett"),
            TokenKind::Pruvamm => write!(f, "pruvamm"),
            TokenKind::Schiatta => write!(f, "schiatta"),
            TokenKind::Iett => write!(f, "iett"),
            TokenKind::Nu => write!(f, "nu"),
            TokenKind::Bell => write!(f, "bell"),
            TokenKind::Na => write!(f, "na"),
            TokenKind::Famiglie => write!(f, "famiglie"),
            TokenKind::Stu => write!(f, "stu"),
            TokenKind::Cos => write!(f, "cos"),
            TokenKind::Chiamm => write!(f, "chiamm"),
            TokenKind::Da => write!(f, "da"),
            TokenKind::Mann => write!(f, "mann"),
            TokenKind::For => write!(f, "for"),
            TokenKind::Predefinit => write!(f, "predefinit"),
            TokenKind::Rompe => write!(f, "rompe"),
            TokenKind::Salta => write!(f, "salta"),
            // New Keywords
            TokenKind::Vir2 => write!(f, "vir"),
            TokenKind::Caso => write!(f, "caso"),
            TokenKind::SinnoFa => write!(f, "sinnò fa"),
            TokenKind::Fisso => write!(f, "fisso"),
            TokenKind::Figlio => write!(f, "figlio"),
            TokenKind::De => write!(f, "'e"),
            TokenKind::OPate => write!(f, "'o pate"),
            TokenKind::CheE => write!(f, "chè è"),
            TokenKind::EUno => write!(f, "è uno"),
            TokenKind::Leva => write!(f, "leva"),
            TokenKind::DintA => write!(f, "dint'a"),
            TokenKind::Caccia => write!(f, "caccia"),
            TokenKind::Fermete => write!(f, "fermete"),
            TokenKind::Scrive => write!(f, "scrive"),
            TokenKind::Avvis => write!(f, "avvis"),
            // Logical Operators
            TokenKind::And => write!(f, "e"),
            TokenKind::Or => write!(f, "o"),
            TokenKind::Not => write!(f, "no"),
            TokenKind::Manco => write!(f, "manco"),
            TokenKind::Pure => write!(f, "pure"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::StarStar => write!(f, "**"),
            TokenKind::EqualEqual => write!(f, "=="),
            TokenKind::EqualEqualEqual => write!(f, "==="),
            TokenKind::BangEqual => write!(f, "!="),
            TokenKind::BangEqualEqual => write!(f, "!=="),
            TokenKind::Less => write!(f, "<"),
            TokenKind::Greater => write!(f, ">"),
            TokenKind::LessEqual => write!(f, "<="),
            TokenKind::GreaterEqual => write!(f, ">="),
            TokenKind::Equal => write!(f, "="),
            TokenKind::PlusEqual => write!(f, "+="),
            TokenKind::MinusEqual => write!(f, "-="),
            TokenKind::StarEqual => write!(f, "*="),
            TokenKind::SlashEqual => write!(f, "/="),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::LeftBracket => write!(f, "["),
            TokenKind::RightBracket => write!(f, "]"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Question => write!(f, "?"),
            TokenKind::Arrow => write!(f, "=>"),
            TokenKind::Identifier(s) => write!(f, "{}", s),
            TokenKind::String(s) => write!(f, "\"{}\"", s),
            TokenKind::Number(n) => write!(f, "{}", n),
            TokenKind::Newline => write!(f, "\\n"),
            TokenKind::Eof => write!(f, "EOF"),
            TokenKind::Invalid(s) => write!(f, "INVALID({})", s),
        }
    }
}

/// Maps a string to the corresponding keyword (if it exists).
///
/// Used by the lexer to determine if an identifier is a reserved
/// word of the language.
///
/// # Arguments
///
/// * `ident` - The string to verify
///
/// # Returns
///
/// * `Some(TokenKind)` - If the string is a keyword
/// * `None` - If the string is a normal identifier
///
/// # Example
///
/// ```rust
/// use fratm_core::lexer::{lookup_keyword, TokenKind};
///
/// assert!(matches!(lookup_keyword("chist"), Some(TokenKind::Chist)));
/// assert!(matches!(lookup_keyword("facc"), Some(TokenKind::Facc)));
/// assert!(lookup_keyword("pizza").is_none()); // Not a keyword
/// ```
pub fn lookup_keyword(ident: &str) -> Option<TokenKind> {
    match ident {
        "chist" => Some(TokenKind::Chist),
        "è" => Some(TokenKind::E),
        "tien" => Some(TokenKind::Tien),
        "facc" => Some(TokenKind::Facc),
        "piglie" => Some(TokenKind::Piglie),
        "si" => Some(TokenKind::Si),
        "sinnò" => Some(TokenKind::Sinno),
        "pe" => Some(TokenKind::Pe),
        "ogni" => Some(TokenKind::Ogni),
        "mentre" => Some(TokenKind::Mentre),
        "che" => Some(TokenKind::Che),
        "overo" => Some(TokenKind::Overo),
        "sfòls" => Some(TokenKind::Sfols),
        "nisciun" => Some(TokenKind::Nisciun),
        "boh" => Some(TokenKind::Boh),
        "stamm" => Some(TokenKind::Stamm),
        "a" => Some(TokenKind::A),
        "dì" => Some(TokenKind::Di),
        "mo" => Some(TokenKind::Mo),
        "vir" => Some(TokenKind::Vir),
        "aspett" => Some(TokenKind::Aspett),
        "pruvamm" => Some(TokenKind::Pruvamm),
        "schiatta" => Some(TokenKind::Schiatta),
        "iett" => Some(TokenKind::Iett),
        "nu" => Some(TokenKind::Nu),
        "bell" => Some(TokenKind::Bell),
        "na" => Some(TokenKind::Na),
        "famiglie" => Some(TokenKind::Famiglie),
        "stu" => Some(TokenKind::Stu),
        "cos" => Some(TokenKind::Cos),
        "chiamm" => Some(TokenKind::Chiamm),
        "da" => Some(TokenKind::Da),
        "mann" => Some(TokenKind::Mann),
        "for" => Some(TokenKind::For),
        "predefinit" => Some(TokenKind::Predefinit),
        "rompe" => Some(TokenKind::Rompe),
        "salta" => Some(TokenKind::Salta),
        // New goliardic keywords
        "caso" => Some(TokenKind::Caso),
        "fisso" => Some(TokenKind::Fisso),
        "figlio" => Some(TokenKind::Figlio),
        "leva" => Some(TokenKind::Leva),
        "caccia" => Some(TokenKind::Caccia),
        "fermete" => Some(TokenKind::Fermete),
        "scrive" => Some(TokenKind::Scrive),
        "avvis" => Some(TokenKind::Avvis),
        // Logical operators
        "e" => Some(TokenKind::And),
        "o" => Some(TokenKind::Or),
        "no" => Some(TokenKind::Not),
        "manco" => Some(TokenKind::Manco),
        "pure" => Some(TokenKind::Pure),
        _ => None,
    }
}
