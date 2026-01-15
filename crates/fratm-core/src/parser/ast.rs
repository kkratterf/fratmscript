//! Abstract Syntax Tree definitions

use crate::lexer::Span;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    VariableDecl {
        name: String,
        value: Option<Expression>,
        is_const: bool,
        span: Span,
    },
    FunctionDecl {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
        is_async: bool,
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
    For {
        init: Option<Box<Statement>>,
        condition: Option<Expression>,
        update: Option<Expression>,
        body: Vec<Statement>,
        span: Span,
    },
    Break { span: Span },
    Continue { span: Span },
    /// debugger statement - "fermete"
    Debugger { span: Span },
    TryCatch {
        try_body: Vec<Statement>,
        catch_param: Option<String>,
        catch_body: Vec<Statement>,
        span: Span,
    },
    Throw {
        value: Expression,
        span: Span,
    },
    ClassDecl {
        name: String,
        methods: Vec<Statement>,
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
pub enum Expression {
    Identifier { name: String, span: Span },
    Number { value: f64, span: Span },
    String { value: String, span: Span },
    Boolean { value: bool, span: Span },
    Null { span: Span },
    Undefined { span: Span },
    This { span: Span },
    Array { elements: Vec<Expression>, span: Span },
    Object { properties: Vec<(String, Expression)>, span: Span },
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
        value: Box<Expression>,
        span: Span,
    },
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
        span: Span,
    },
    Member {
        object: Box<Expression>,
        property: Box<Expression>,
        computed: bool,
        span: Span,
    },
    New {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
        span: Span,
    },
    ArrowFunction {
        params: Vec<String>,
        body: ArrowBody,
        span: Span,
    },
    Await {
        argument: Box<Expression>,
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
            Expression::Array { span, .. } => *span,
            Expression::Object { span, .. } => *span,
            Expression::Binary { span, .. } => *span,
            Expression::Unary { span, .. } => *span,
            Expression::Assignment { span, .. } => *span,
            Expression::Call { span, .. } => *span,
            Expression::Member { span, .. } => *span,
            Expression::New { span, .. } => *span,
            Expression::ArrowFunction { span, .. } => *span,
            Expression::Await { span, .. } => *span,
            Expression::Ternary { span, .. } => *span,
            Expression::ConsoleLog { span, .. } => *span,
            Expression::ConsoleWarn { span, .. } => *span,
            Expression::ConsoleError { span, .. } => *span,
            Expression::TypeOf { span, .. } => *span,
            Expression::Delete { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArrowBody {
    Expression(Box<Expression>),
    Block(Vec<Statement>),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BinaryOp {
    Add, Subtract, Multiply, Divide, Modulo, Power,
    Equal, StrictEqual, NotEqual, StrictNotEqual,
    LessThan, GreaterThan, LessEqual, GreaterEqual,
    And, Or,
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
