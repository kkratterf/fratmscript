//! Abstract Syntax Tree definitions

use crate::lexer::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    /// `tien x = 1, y = 2` / `chist è { a } = o`
    VariableDecl {
        declarations: Vec<Declarator>,
        is_const: bool,
        span: Span,
    },
    FunctionDecl {
        name: String,
        params: Vec<Pattern>,
        body: Vec<Statement>,
        is_async: bool,
        /// `facc* nomme()`
        is_generator: bool,
        span: Span,
    },
    Return {
        value: Option<Expression>,
        span: Span,
    },
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
        span: Span,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
        span: Span,
    },
    /// `primm fa { } mentre che (cond)`
    DoWhile {
        body: Vec<Statement>,
        condition: Expression,
        span: Span,
    },
    For {
        init: Option<Box<Statement>>,
        condition: Option<Expression>,
        update: Option<Expression>,
        body: Vec<Statement>,
        span: Span,
    },
    /// `pe ogni (tien x dint xs)` / `pe ogni (tien k 'ncopp o)` /
    /// `pe aspett (… dint …)`
    ForEach {
        binding: ForBinding,
        /// `dint` (for...of) when true, `'ncopp` (for...in) when false
        of: bool,
        is_await: bool,
        iterable: Expression,
        body: Vec<Statement>,
        span: Span,
    },
    /// `vedimm (x) { caso 1: … predefinit: … }`
    Switch {
        discriminant: Expression,
        cases: Vec<SwitchCase>,
        span: Span,
    },
    Break {
        span: Span,
    },
    Continue {
        span: Span,
    },
    /// debugger statement - "fermete"
    Debugger {
        span: Span,
    },
    /// `pruvamm { } e si schiatta (e) { } aropp { }`; at least one of
    /// `catch_body` and `finally_body` is present
    TryCatch {
        try_body: Vec<Statement>,
        catch_param: Option<Pattern>,
        catch_body: Option<Vec<Statement>>,
        finally_body: Option<Vec<Statement>>,
        span: Span,
    },
    Throw {
        value: Expression,
        span: Span,
    },
    ClassDecl {
        name: String,
        super_class: Option<String>,
        members: Vec<ClassMember>,
        span: Span,
    },
    Import {
        specifiers: Vec<ImportSpecifier>,
        source: String,
        span: Span,
    },
    Export {
        declaration: Option<Box<Statement>>,
        default_value: Option<Expression>,
        span: Span,
    },
    Expression {
        expression: Expression,
        span: Span,
    },
    Block {
        statements: Vec<Statement>,
        span: Span,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSpecifier {
    pub imported: String,
    pub local: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClassMember {
    Method(ClassMethod),
    /// `conteggio = 0`, `fisso totale = 0`, `#segreto`, `[chiave] = 1`
    Field {
        /// Private names are `PropertyKey::Name` and keep their `#`
        key: PropertyKey,
        value: Option<Expression>,
        is_static: bool,
        span: Span,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MethodKind {
    Method,
    /// `damme nome() { }`
    Getter,
    /// `miette nome(v) { }`
    Setter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassMethod {
    /// Private names are `PropertyKey::Name` and keep their `#`;
    /// `*[Symbol.iterator]()` has a computed key
    pub key: PropertyKey,
    pub kind: MethodKind,
    pub params: Vec<Pattern>,
    pub body: Vec<Statement>,
    pub is_async: bool,
    pub is_generator: bool,
    pub is_static: bool,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    Identifier {
        name: String,
        span: Span,
    },
    Number {
        value: f64,
        span: Span,
    },
    String {
        value: String,
        span: Span,
    },
    Boolean {
        value: bool,
        span: Span,
    },
    Null {
        span: Span,
    },
    Undefined {
        span: Span,
    },
    This {
        span: Span,
    },
    Super {
        span: Span,
    },
    Array {
        elements: Vec<Expression>,
        span: Span,
    },
    /// `a, b` (only in `pe` loop headers)
    Sequence {
        expressions: Vec<Expression>,
        span: Span,
    },
    /// `/pattern/flags`, kept as written
    Regex {
        pattern: String,
        flags: String,
        span: Span,
    },
    /// Template literal; `quasis` are raw text, one more than `expressions`
    Template {
        quasis: Vec<String>,
        expressions: Vec<Expression>,
        span: Span,
    },
    Object {
        properties: Vec<Property>,
        span: Span,
    },
    /// `...argument` in arrays, calls and object literals
    Spread {
        argument: Box<Expression>,
        span: Span,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOp,
        right: Box<Expression>,
        span: Span,
    },
    Unary {
        operator: UnaryOp,
        operand: Box<Expression>,
        span: Span,
    },
    Assignment {
        target: Box<Expression>,
        operator: AssignOp,
        value: Box<Expression>,
        span: Span,
    },
    /// `++x`, `x--`
    Update {
        operator: UpdateOp,
        prefix: bool,
        argument: Box<Expression>,
        span: Span,
    },
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
        /// `f?.()`
        optional: bool,
        span: Span,
    },
    Member {
        object: Box<Expression>,
        property: Box<Expression>,
        computed: bool,
        /// `a?.b`, `a?.[k]`
        optional: bool,
        span: Span,
    },
    New {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
        span: Span,
    },
    ArrowFunction {
        params: Vec<Pattern>,
        body: ArrowBody,
        is_async: bool,
        span: Span,
    },
    Await {
        argument: Box<Expression>,
        span: Span,
    },
    /// `caccia x`, `caccia* altri` (delegation), or a bare `caccia`
    Yield {
        argument: Option<Box<Expression>>,
        delegate: bool,
        span: Span,
    },
    Ternary {
        condition: Box<Expression>,
        consequent: Box<Expression>,
        alternate: Box<Expression>,
        span: Span,
    },
    ConsoleLog {
        arguments: Vec<Expression>,
        span: Span,
    },
    /// console.warn() - "avvis"
    ConsoleWarn {
        arguments: Vec<Expression>,
        span: Span,
    },
    /// console.error() - "scrive"
    ConsoleError {
        arguments: Vec<Expression>,
        span: Span,
    },
    /// typeof operand - "chè è"
    TypeOf {
        operand: Box<Expression>,
        span: Span,
    },
    /// delete operand - "leva"
    Delete {
        operand: Box<Expression>,
        span: Span,
    },
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Expression::Identifier { span, .. } => *span,
            Expression::Number { span, .. } => *span,
            Expression::String { span, .. } => *span,
            Expression::Boolean { span, .. } => *span,
            Expression::Null { span } => *span,
            Expression::Undefined { span } => *span,
            Expression::This { span } => *span,
            Expression::Super { span } => *span,
            Expression::Array { span, .. } => *span,
            Expression::Regex { span, .. } => *span,
            Expression::Sequence { span, .. } => *span,
            Expression::Template { span, .. } => *span,
            Expression::Object { span, .. } => *span,
            Expression::Spread { span, .. } => *span,
            Expression::Binary { span, .. } => *span,
            Expression::Unary { span, .. } => *span,
            Expression::Assignment { span, .. } => *span,
            Expression::Update { span, .. } => *span,
            Expression::Call { span, .. } => *span,
            Expression::Member { span, .. } => *span,
            Expression::New { span, .. } => *span,
            Expression::ArrowFunction { span, .. } => *span,
            Expression::Await { span, .. } => *span,
            Expression::Yield { span, .. } => *span,
            Expression::Ternary { span, .. } => *span,
            Expression::ConsoleLog { span, .. } => *span,
            Expression::ConsoleWarn { span, .. } => *span,
            Expression::ConsoleError { span, .. } => *span,
            Expression::TypeOf { span, .. } => *span,
            Expression::Delete { span, .. } => *span,
        }
    }
}

/// A `caso value:` (or `predefinit:` when `test` is `None`) label and its statements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchCase {
    pub test: Option<Expression>,
    pub body: Vec<Statement>,
}

/// The loop variable of a for...of / for...in.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForBinding {
    /// `tien x`
    Let(Pattern),
    /// `chist è x`
    Const(Pattern),
    /// `x`, an existing variable
    Existing(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Declarator {
    pub target: Pattern,
    pub value: Option<Expression>,
}

/// A binding target: declarations, function parameters and catch parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Pattern {
    Identifier {
        name: String,
        span: Span,
    },
    /// `[primo, , terzo, ...ato]` (`None` is a hole)
    Array {
        elements: Vec<Option<Pattern>>,
        span: Span,
    },
    /// `{ a, b: x, c = 1, ...resto }`
    Object {
        properties: Vec<ObjectPatternProperty>,
        span: Span,
    },
    /// `target = default`
    Default {
        target: Box<Pattern>,
        default: Box<Expression>,
    },
    /// `...target`
    Rest {
        argument: Box<Pattern>,
    },
}

/// An entry of an object pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectPatternProperty {
    /// `key: value`, or `key` / `key = default` when `shorthand`
    Property {
        key: PropertyKey,
        value: Pattern,
        shorthand: bool,
    },
    /// `...resto`
    Rest(Pattern),
}

/// An entry of an object literal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Property {
    /// `key: value`
    KeyValue { key: PropertyKey, value: Expression },
    /// `{ nome }`
    Shorthand { name: String, span: Span },
    /// `saluta() { }`, `mo vir carica() { }`
    Method {
        key: PropertyKey,
        params: Vec<Pattern>,
        body: Vec<Statement>,
        is_async: bool,
        is_generator: bool,
    },
    /// `...object`
    Spread(Expression),
}

/// The key of an object literal entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyKey {
    /// A word, keywords included: `{ si: 1 }`
    Name(String),
    String(String),
    Number(f64),
    /// `{ [chiave]: valore }`
    Computed(Box<Expression>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArrowBody {
    Expression(Box<Expression>),
    Block(Vec<Statement>),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    StrictEqual,
    NotEqual,
    StrictNotEqual,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    InstanceOf,
    Nullish,
}

impl BinaryOp {
    pub fn to_js(&self) -> &'static str {
        match self {
            BinaryOp::Add => "+",
            BinaryOp::Subtract => "-",
            BinaryOp::Multiply => "*",
            BinaryOp::Divide => "/",
            BinaryOp::Modulo => "%",
            BinaryOp::Power => "**",
            BinaryOp::Equal => "==",
            BinaryOp::StrictEqual => "===",
            BinaryOp::NotEqual => "!=",
            BinaryOp::StrictNotEqual => "!==",
            BinaryOp::LessThan => "<",
            BinaryOp::GreaterThan => ">",
            BinaryOp::LessEqual => "<=",
            BinaryOp::GreaterEqual => ">=",
            BinaryOp::And => "&&",
            BinaryOp::Or => "||",
            BinaryOp::InstanceOf => "instanceof",
            BinaryOp::Nullish => "??",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AssignOp {
    Assign,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Nullish,
    Or,
    And,
}

impl AssignOp {
    pub fn to_js(&self) -> &'static str {
        match self {
            AssignOp::Assign => "=",
            AssignOp::Add => "+=",
            AssignOp::Subtract => "-=",
            AssignOp::Multiply => "*=",
            AssignOp::Divide => "/=",
            AssignOp::Modulo => "%=",
            AssignOp::Power => "**=",
            AssignOp::Nullish => "??=",
            AssignOp::Or => "||=",
            AssignOp::And => "&&=",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum UpdateOp {
    Increment,
    Decrement,
}

impl UpdateOp {
    pub fn to_js(&self) -> &'static str {
        match self {
            UpdateOp::Increment => "++",
            UpdateOp::Decrement => "--",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum UnaryOp {
    Negate,
    Not,
}

impl UnaryOp {
    pub fn to_js(&self) -> &'static str {
        match self {
            UnaryOp::Negate => "-",
            UnaryOp::Not => "!",
        }
    }
}
