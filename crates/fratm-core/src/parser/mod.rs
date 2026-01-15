//! Parser for FratmScript

mod ast;

pub use ast::*;
use crate::lexer::{Span, Token, TokenKind};

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

impl ParseError {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self { message: message.into(), span }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, Vec<ParseError>> {
        let mut statements = Vec::new();
        let mut errors = Vec::new();

        while !self.is_at_end() {
            while self.check(&TokenKind::Newline) {
                self.advance();
            }
            if self.is_at_end() { break; }

            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    errors.push(e);
                    self.synchronize();
                }
            }
        }

        if errors.is_empty() {
            Ok(Program { statements })
        } else {
            Err(errors)
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        while self.check(&TokenKind::Newline) { self.advance(); }

        if self.check(&TokenKind::Chist) { return self.parse_const_declaration(); }
        if self.check(&TokenKind::Tien) { return self.parse_let_declaration(); }
        if self.check(&TokenKind::Mo) && self.check_next(&TokenKind::Vir) { return self.parse_async_function(); }
        if self.check(&TokenKind::Facc) { return self.parse_function(); }
        if self.check(&TokenKind::Piglie) { return self.parse_return(); }
        if self.check(&TokenKind::Si) { return self.parse_if(); }
        if self.check(&TokenKind::Mentre) { return self.parse_while(); }
        if self.check(&TokenKind::Pe) { return self.parse_for(); }
        if self.check(&TokenKind::Rompe) { return self.parse_break(); }
        if self.check(&TokenKind::Salta) { return self.parse_continue(); }
        if self.check(&TokenKind::Fermete) { return self.parse_debugger(); }
        if self.check(&TokenKind::Pruvamm) { return self.parse_try_catch(); }
        if self.check(&TokenKind::Iett) { return self.parse_throw(); }
        if self.check(&TokenKind::Na) { return self.parse_class(); }
        if self.check(&TokenKind::Chiamm) { return self.parse_import(); }
        if self.check(&TokenKind::Mann) { return self.parse_export(); }
        if self.check(&TokenKind::LeftBrace) { return self.parse_block(); }

        self.parse_expression_statement()
    }

    fn parse_const_declaration(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Chist)?;
        self.expect(&TokenKind::E)?;
        let name = self.expect_identifier()?;
        self.expect(&TokenKind::Equal)?;
        let value = self.parse_expression()?;
        Ok(Statement::VariableDecl { name, value: Some(value), is_const: true, span: self.span_from(start.start) })
    }

    fn parse_let_declaration(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Tien)?;
        let name = self.expect_identifier()?;
        let value = if self.match_token(&TokenKind::Equal) { Some(self.parse_expression()?) } else { None };
        Ok(Statement::VariableDecl { name, value, is_const: false, span: self.span_from(start.start) })
    }

    fn parse_function(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Facc)?;
        let name = self.expect_identifier()?;
        let params = self.parse_parameters()?;
        let body = self.parse_block_body()?;
        Ok(Statement::FunctionDecl { name, params, body, is_async: false, span: self.span_from(start.start) })
    }

    fn parse_async_function(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Mo)?;
        self.expect(&TokenKind::Vir)?;
        self.expect(&TokenKind::Facc)?;
        let name = self.expect_identifier()?;
        let params = self.parse_parameters()?;
        let body = self.parse_block_body()?;
        Ok(Statement::FunctionDecl { name, params, body, is_async: true, span: self.span_from(start.start) })
    }

    fn parse_parameters(&mut self) -> Result<Vec<String>, ParseError> {
        self.expect(&TokenKind::LeftParen)?;
        let mut params = Vec::new();
        if !self.check(&TokenKind::RightParen) {
            params.push(self.expect_identifier()?);
            while self.match_token(&TokenKind::Comma) {
                params.push(self.expect_identifier()?);
            }
        }
        self.expect(&TokenKind::RightParen)?;
        Ok(params)
    }

    fn parse_return(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Piglie)?;
        let value = if !self.check(&TokenKind::Newline) && !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            Some(self.parse_expression()?)
        } else { None };
        Ok(Statement::Return { value, span: self.span_from(start.start) })
    }

    fn parse_if(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Si)?;
        self.expect(&TokenKind::LeftParen)?;
        let condition = self.parse_expression()?;
        self.expect(&TokenKind::RightParen)?;
        let then_branch = self.parse_block_body()?;
        let else_branch = if self.match_token(&TokenKind::Sinno) {
            if self.check(&TokenKind::Si) { Some(vec![self.parse_if()?]) }
            else { Some(self.parse_block_body()?) }
        } else { None };
        Ok(Statement::If { condition, then_branch, else_branch, span: self.span_from(start.start) })
    }

    fn parse_while(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Mentre)?;
        self.expect(&TokenKind::Che)?;
        self.expect(&TokenKind::LeftParen)?;
        let condition = self.parse_expression()?;
        self.expect(&TokenKind::RightParen)?;
        let body = self.parse_block_body()?;
        Ok(Statement::While { condition, body, span: self.span_from(start.start) })
    }

    fn parse_for(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Pe)?;
        // "ogni" è ora opzionale per retrocompatibilità
        self.match_token(&TokenKind::Ogni);
        self.expect(&TokenKind::LeftParen)?;
        let init = if self.check(&TokenKind::Tien) { Some(Box::new(self.parse_let_declaration()?)) }
        else if !self.check(&TokenKind::Semicolon) {
            let expr = self.parse_expression()?;
            Some(Box::new(Statement::Expression { span: self.current_span(), expression: expr }))
        } else { None };
        self.expect(&TokenKind::Semicolon)?;
        let condition = if !self.check(&TokenKind::Semicolon) { Some(self.parse_expression()?) } else { None };
        self.expect(&TokenKind::Semicolon)?;
        let update = if !self.check(&TokenKind::RightParen) { Some(self.parse_expression()?) } else { None };
        self.expect(&TokenKind::RightParen)?;
        let body = self.parse_block_body()?;
        Ok(Statement::For { init, condition, update, body, span: self.span_from(start.start) })
    }

    fn parse_break(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Rompe)?;
        Ok(Statement::Break { span: self.span_from(start.start) })
    }

    fn parse_continue(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Salta)?;
        Ok(Statement::Continue { span: self.span_from(start.start) })
    }

    fn parse_debugger(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Fermete)?;
        Ok(Statement::Debugger { span: self.span_from(start.start) })
    }

    fn parse_try_catch(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Pruvamm)?;
        let try_body = self.parse_block_body()?;
        self.expect(&TokenKind::And)?;
        self.expect(&TokenKind::Si)?;
        self.expect(&TokenKind::Schiatta)?;
        let catch_param = if self.match_token(&TokenKind::LeftParen) {
            let param = self.expect_identifier()?;
            self.expect(&TokenKind::RightParen)?;
            Some(param)
        } else { None };
        let catch_body = self.parse_block_body()?;
        Ok(Statement::TryCatch { try_body, catch_param, catch_body, span: self.span_from(start.start) })
    }

    fn parse_throw(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Iett)?;
        let value = self.parse_expression()?;
        Ok(Statement::Throw { value, span: self.span_from(start.start) })
    }

    fn parse_class(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Na)?;
        self.expect(&TokenKind::Famiglie)?;
        let name = self.expect_identifier()?;
        self.expect(&TokenKind::LeftBrace)?;
        let mut methods = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            while self.check(&TokenKind::Newline) { self.advance(); }
            if self.check(&TokenKind::RightBrace) { break; }
            methods.push(self.parse_function()?);
        }
        self.expect(&TokenKind::RightBrace)?;
        Ok(Statement::ClassDecl { name, methods, span: self.span_from(start.start) })
    }

    fn parse_import(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Chiamm)?;
        self.expect(&TokenKind::LeftBrace)?;
        let mut specifiers = Vec::new();
        if !self.check(&TokenKind::RightBrace) {
            loop {
                let imported = self.expect_identifier()?;
                specifiers.push(ImportSpecifier { imported: imported.clone(), local: imported });
                if !self.match_token(&TokenKind::Comma) { break; }
            }
        }
        self.expect(&TokenKind::RightBrace)?;
        self.expect(&TokenKind::Da)?;
        let source = self.expect_string()?;
        Ok(Statement::Import { specifiers, source, span: self.span_from(start.start) })
    }

    fn parse_export(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Mann)?;
        self.expect(&TokenKind::For)?;
        if self.match_token(&TokenKind::Predefinit) {
            let value = self.parse_expression()?;
            Ok(Statement::Export { declaration: None, default_value: Some(value), span: self.span_from(start.start) })
        } else {
            let decl = self.parse_statement()?;
            Ok(Statement::Export { declaration: Some(Box::new(decl)), default_value: None, span: self.span_from(start.start) })
        }
    }

    fn parse_block(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        let statements = self.parse_block_body()?;
        Ok(Statement::Block { statements, span: self.span_from(start.start) })
    }

    fn parse_block_body(&mut self) -> Result<Vec<Statement>, ParseError> {
        self.expect(&TokenKind::LeftBrace)?;
        let mut statements = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            while self.check(&TokenKind::Newline) { self.advance(); }
            if self.check(&TokenKind::RightBrace) { break; }
            statements.push(self.parse_statement()?);
        }
        self.expect(&TokenKind::RightBrace)?;
        Ok(statements)
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        let expr = self.parse_expression()?;
        Ok(Statement::Expression { expression: expr, span: self.span_from(start.start) })
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expression, ParseError> {
        let expr = self.parse_ternary()?;
        if self.match_token(&TokenKind::Equal) {
            let value = self.parse_assignment()?;
            let span = self.span_from(expr.span().start);
            return Ok(Expression::Assignment { target: Box::new(expr), value: Box::new(value), span });
        }
        Ok(expr)
    }

    fn parse_ternary(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_or()?;
        if self.match_token(&TokenKind::Question) {
            let consequent = self.parse_expression()?;
            self.expect(&TokenKind::Colon)?;
            let alternate = self.parse_ternary()?;
            let span = self.span_from(expr.span().start);
            expr = Expression::Ternary { condition: Box::new(expr), consequent: Box::new(consequent), alternate: Box::new(alternate), span };
        }
        Ok(expr)
    }

    fn parse_or(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_and()?;
        // Supporta sia "o" che "||" style
        while self.match_token(&TokenKind::Or) {
            let right = self.parse_and()?;
            let span = self.span_from(expr.span().start);
            expr = Expression::Binary { left: Box::new(expr), operator: BinaryOp::Or, right: Box::new(right), span };
        }
        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_equality()?;
        // Supporta sia "e" che "pure" (entrambi significano AND)
        while self.match_token(&TokenKind::And) || self.match_token(&TokenKind::Pure) {
            let right = self.parse_equality()?;
            let span = self.span_from(expr.span().start);
            expr = Expression::Binary { left: Box::new(expr), operator: BinaryOp::And, right: Box::new(right), span };
        }
        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_comparison()?;
        loop {
            let op = if self.match_token(&TokenKind::EqualEqualEqual) { BinaryOp::StrictEqual }
            else if self.match_token(&TokenKind::EqualEqual) { BinaryOp::Equal }
            else if self.match_token(&TokenKind::BangEqualEqual) { BinaryOp::StrictNotEqual }
            else if self.match_token(&TokenKind::BangEqual) { BinaryOp::NotEqual }
            else { break };
            let right = self.parse_comparison()?;
            let span = self.span_from(expr.span().start);
            expr = Expression::Binary { left: Box::new(expr), operator: op, right: Box::new(right), span };
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_term()?;
        loop {
            let op = if self.match_token(&TokenKind::Less) { BinaryOp::LessThan }
            else if self.match_token(&TokenKind::LessEqual) { BinaryOp::LessEqual }
            else if self.match_token(&TokenKind::Greater) { BinaryOp::GreaterThan }
            else if self.match_token(&TokenKind::GreaterEqual) { BinaryOp::GreaterEqual }
            else { break };
            let right = self.parse_term()?;
            let span = self.span_from(expr.span().start);
            expr = Expression::Binary { left: Box::new(expr), operator: op, right: Box::new(right), span };
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_factor()?;
        loop {
            let op = if self.match_token(&TokenKind::Plus) { BinaryOp::Add }
            else if self.match_token(&TokenKind::Minus) { BinaryOp::Subtract }
            else { break };
            let right = self.parse_factor()?;
            let span = self.span_from(expr.span().start);
            expr = Expression::Binary { left: Box::new(expr), operator: op, right: Box::new(right), span };
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_power()?;
        loop {
            let op = if self.match_token(&TokenKind::Star) { BinaryOp::Multiply }
            else if self.match_token(&TokenKind::Slash) { BinaryOp::Divide }
            else if self.match_token(&TokenKind::Percent) { BinaryOp::Modulo }
            else { break };
            let right = self.parse_power()?;
            let span = self.span_from(expr.span().start);
            expr = Expression::Binary { left: Box::new(expr), operator: op, right: Box::new(right), span };
        }
        Ok(expr)
    }

    fn parse_power(&mut self) -> Result<Expression, ParseError> {
        let expr = self.parse_unary()?;
        if self.match_token(&TokenKind::StarStar) {
            let right = self.parse_power()?;
            let span = self.span_from(expr.span().start);
            return Ok(Expression::Binary { left: Box::new(expr), operator: BinaryOp::Power, right: Box::new(right), span });
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expression, ParseError> {
        // Negazione numerica: -x
        if self.match_token(&TokenKind::Minus) {
            let start = self.previous().span;
            let operand = self.parse_unary()?;
            return Ok(Expression::Unary { operator: UnaryOp::Negate, operand: Box::new(operand), span: self.span_from(start.start) });
        }
        // Negazione logica: no, !, manco
        if self.match_token(&TokenKind::Not) || self.match_token(&TokenKind::Manco) {
            let start = self.previous().span;
            let operand = self.parse_unary()?;
            return Ok(Expression::Unary { operator: UnaryOp::Not, operand: Box::new(operand), span: self.span_from(start.start) });
        }
        // Await: aspett
        if self.match_token(&TokenKind::Aspett) {
            let start = self.previous().span;
            let argument = self.parse_unary()?;
            return Ok(Expression::Await { argument: Box::new(argument), span: self.span_from(start.start) });
        }
        // Delete: leva
        if self.match_token(&TokenKind::Leva) {
            let start = self.previous().span;
            let operand = self.parse_unary()?;
            return Ok(Expression::Delete { operand: Box::new(operand), span: self.span_from(start.start) });
        }
        // Typeof: chè è - per ora non implementato come keyword composta, useremo CheE se presente
        self.parse_call()
    }

    fn parse_call(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.match_token(&TokenKind::LeftParen) {
                let arguments = self.parse_arguments()?;
                let span = self.span_from(expr.span().start);
                expr = Expression::Call { callee: Box::new(expr), arguments, span };
            } else if self.match_token(&TokenKind::Dot) {
                let name = self.expect_identifier()?;
                let span = self.span_from(expr.span().start);
                expr = Expression::Member { object: Box::new(expr), property: Box::new(Expression::Identifier { name, span: self.previous().span }), computed: false, span };
            } else if self.match_token(&TokenKind::LeftBracket) {
                let property = self.parse_expression()?;
                self.expect(&TokenKind::RightBracket)?;
                let span = self.span_from(expr.span().start);
                expr = Expression::Member { object: Box::new(expr), property: Box::new(property), computed: true, span };
            } else { break; }
        }
        Ok(expr)
    }

    fn parse_arguments(&mut self) -> Result<Vec<Expression>, ParseError> {
        let mut args = Vec::new();
        if !self.check(&TokenKind::RightParen) {
            args.push(self.parse_expression()?);
            while self.match_token(&TokenKind::Comma) { args.push(self.parse_expression()?); }
        }
        self.expect(&TokenKind::RightParen)?;
        Ok(args)
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        let token = self.advance();
        let span = token.span;

        match &token.kind {
            TokenKind::Number(n) => Ok(Expression::Number { value: *n, span }),
            TokenKind::String(s) => Ok(Expression::String { value: s.clone(), span }),
            TokenKind::Overo => Ok(Expression::Boolean { value: true, span }),
            TokenKind::Sfols => Ok(Expression::Boolean { value: false, span }),
            TokenKind::Nisciun => Ok(Expression::Null { span }),
            TokenKind::Boh => Ok(Expression::Undefined { span }),
            TokenKind::Stu => {
                self.expect(&TokenKind::Cos)?;
                Ok(Expression::This { span: self.span_from(span.start) })
            }
            TokenKind::Nu => {
                self.expect(&TokenKind::Bell)?;
                let callee = self.parse_call()?;
                let span = self.span_from(span.start);
                if let Expression::Call { callee: inner, arguments, .. } = callee {
                    Ok(Expression::New { callee: inner, arguments, span })
                } else {
                    Ok(Expression::New { callee: Box::new(callee), arguments: vec![], span })
                }
            }
            TokenKind::Stamm => {
                self.expect(&TokenKind::A)?;
                self.expect(&TokenKind::Di)?;
                self.expect(&TokenKind::LeftParen)?;
                let arguments = self.parse_arguments()?;
                Ok(Expression::ConsoleLog { arguments, span: self.span_from(span.start) })
            }
            // console.warn() - "avvis a dì(...)"
            TokenKind::Avvis => {
                self.expect(&TokenKind::A)?;
                self.expect(&TokenKind::Di)?;
                self.expect(&TokenKind::LeftParen)?;
                let arguments = self.parse_arguments()?;
                Ok(Expression::ConsoleWarn { arguments, span: self.span_from(span.start) })
            }
            // console.error() - "scrive a dì(...)"
            TokenKind::Scrive => {
                self.expect(&TokenKind::A)?;
                self.expect(&TokenKind::Di)?;
                self.expect(&TokenKind::LeftParen)?;
                let arguments = self.parse_arguments()?;
                Ok(Expression::ConsoleError { arguments, span: self.span_from(span.start) })
            }
            TokenKind::Identifier(name) => Ok(Expression::Identifier { name: name.clone(), span }),
            TokenKind::LeftParen => {
                let expr = self.parse_expression()?;
                self.expect(&TokenKind::RightParen)?;
                if self.match_token(&TokenKind::Arrow) {
                    let params = if let Expression::Identifier { name, .. } = expr { vec![name] } else { vec![] };
                    let body = if self.check(&TokenKind::LeftBrace) {
                        ArrowBody::Block(self.parse_block_body()?)
                    } else {
                        ArrowBody::Expression(Box::new(self.parse_expression()?))
                    };
                    return Ok(Expression::ArrowFunction { params, body, span: self.span_from(span.start) });
                }
                Ok(expr)
            }
            TokenKind::LeftBracket => {
                let mut elements = Vec::new();
                if !self.check(&TokenKind::RightBracket) {
                    elements.push(self.parse_expression()?);
                    while self.match_token(&TokenKind::Comma) {
                        if self.check(&TokenKind::RightBracket) { break; }
                        elements.push(self.parse_expression()?);
                    }
                }
                self.expect(&TokenKind::RightBracket)?;
                Ok(Expression::Array { elements, span: self.span_from(span.start) })
            }
            TokenKind::LeftBrace => {
                let mut properties = Vec::new();
                // Salta newline iniziali
                while self.check(&TokenKind::Newline) { self.advance(); }
                if !self.check(&TokenKind::RightBrace) {
                    loop {
                        // Salta newline prima della key
                        while self.check(&TokenKind::Newline) { self.advance(); }
                        if self.check(&TokenKind::RightBrace) { break; }
                        let key = self.expect_identifier()?;
                        self.expect(&TokenKind::Colon)?;
                        let value = self.parse_expression()?;
                        properties.push((key, value));
                        // Salta newline dopo il valore
                        while self.check(&TokenKind::Newline) { self.advance(); }
                        if !self.match_token(&TokenKind::Comma) { break; }
                        // Salta newline dopo la virgola
                        while self.check(&TokenKind::Newline) { self.advance(); }
                        if self.check(&TokenKind::RightBrace) { break; }
                    }
                }
                // Salta newline finali
                while self.check(&TokenKind::Newline) { self.advance(); }
                self.expect(&TokenKind::RightBrace)?;
                Ok(Expression::Object { properties, span: self.span_from(span.start) })
            }
            _ => Err(ParseError::new(format!("Ma che è '{}' qua? Aspettavo un'espressione!", token.kind), span)),
        }
    }

    // === Helpers ===

    fn is_at_end(&self) -> bool { self.peek().kind == TokenKind::Eof }
    fn peek(&self) -> &Token { &self.tokens[self.current] }
    fn previous(&self) -> &Token { &self.tokens[self.current - 1] }
    fn advance(&mut self) -> &Token { if !self.is_at_end() { self.current += 1; } self.previous() }
    fn check(&self, kind: &TokenKind) -> bool { std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind) }
    fn check_next(&self, kind: &TokenKind) -> bool {
        if self.current + 1 >= self.tokens.len() { return false; }
        std::mem::discriminant(&self.tokens[self.current + 1].kind) == std::mem::discriminant(kind)
    }
    fn match_token(&mut self, kind: &TokenKind) -> bool { if self.check(kind) { self.advance(); true } else { false } }
    fn expect(&mut self, kind: &TokenKind) -> Result<&Token, ParseError> {
        if self.check(kind) { Ok(self.advance()) }
        else { Err(ParseError::new(format!("Aspettavo '{}', ma ho trovato '{}'", kind, self.peek().kind), self.peek().span)) }
    }
    fn expect_identifier(&mut self) -> Result<String, ParseError> {
        let token = self.advance();
        if let TokenKind::Identifier(name) = &token.kind { Ok(name.clone()) }
        else { Err(ParseError::new(format!("Aspettavo un nome, no '{}'", token.kind), token.span)) }
    }
    fn expect_string(&mut self) -> Result<String, ParseError> {
        let token = self.advance();
        if let TokenKind::String(s) = &token.kind { Ok(s.clone()) }
        else { Err(ParseError::new(format!("Aspettavo una stringa, no '{}'", token.kind), token.span)) }
    }
    fn current_span(&self) -> Span { self.peek().span }
    fn span_from(&self, start: usize) -> Span { Span::new(start, self.previous().span.end, self.previous().span.line, 0) }
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if matches!(self.peek().kind, TokenKind::Chist | TokenKind::Tien | TokenKind::Facc | TokenKind::Si | TokenKind::Mentre | TokenKind::Pe | TokenKind::Piglie | TokenKind::Na | TokenKind::Chiamm | TokenKind::Mann) { return; }
            self.advance();
        }
    }
}
