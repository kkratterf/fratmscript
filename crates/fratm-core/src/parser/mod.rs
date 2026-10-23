//! Parser for FratmScript

mod ast;

use crate::lexer::{Span, Token, TokenKind};
pub use ast::*;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

impl ParseError {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span,
        }
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
            if self.is_at_end() {
                break;
            }

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
        while self.check(&TokenKind::Newline) {
            self.advance();
        }

        if self.check(&TokenKind::Mo)
            && self.check_next(&TokenKind::Vir)
            && self.check_at(2, &TokenKind::Facc)
        {
            return self.parse_async_function();
        }
        if self.check(&TokenKind::Facc) {
            return self.parse_function();
        }
        if self.check(&TokenKind::Si) {
            return self.parse_if();
        }
        if self.check(&TokenKind::Mentre) {
            return self.parse_while();
        }
        if self.check(&TokenKind::Pe) {
            return self.parse_for();
        }
        if self.check(&TokenKind::Pruvamm) {
            return self.parse_try_catch();
        }
        // `na` and `mann` are keywords only in `na famiglie` / `mann for`; otherwise they are names
        if self.check(&TokenKind::Na) && self.check_next(&TokenKind::Famiglie) {
            return self.parse_class();
        }
        if self.check(&TokenKind::Mann) {
            return self.parse_export();
        }
        if self.check(&TokenKind::LeftBrace) {
            return self.parse_block();
        }

        // Simple statements must end the line: otherwise `typeof a` or `0x1F`
        // would silently split into several statements.
        let stmt = if self.check(&TokenKind::Chist) {
            self.parse_const_declaration()?
        } else if self.check(&TokenKind::Tien) {
            self.parse_let_declaration()?
        } else if self.check(&TokenKind::Piglie) {
            self.parse_return()?
        } else if self.check(&TokenKind::Rompe) {
            self.parse_break()?
        } else if self.check(&TokenKind::Salta) {
            self.parse_continue()?
        } else if self.check(&TokenKind::Fermete) {
            self.parse_debugger()?
        } else if self.check(&TokenKind::Iett) {
            self.parse_throw()?
        } else if self.check(&TokenKind::Chiamm) {
            self.parse_import()?
        } else {
            self.parse_expression_statement()?
        };
        self.expect_statement_end()?;
        Ok(stmt)
    }

    /// A simple statement ends at a newline, a `;`, a closing `}` or the end of file.
    fn expect_statement_end(&mut self) -> Result<(), ParseError> {
        if self.match_token(&TokenKind::Semicolon) {
            return Ok(());
        }
        if self.check(&TokenKind::Newline) || self.check(&TokenKind::RightBrace) || self.is_at_end()
        {
            return Ok(());
        }
        let token = self.peek();
        Err(ParseError::new(
            format!(
                "Unexpected '{}' after the end of the statement: go to a new line or use ';'",
                token.kind
            ),
            token.span,
        ))
    }

    fn parse_const_declaration(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Chist)?;
        self.expect(&TokenKind::E)?;
        let name = self.expect_identifier()?;
        self.expect(&TokenKind::Equal)?;
        let value = self.parse_expression()?;
        Ok(Statement::VariableDecl {
            name,
            value: Some(value),
            is_const: true,
            span: self.span_from(start.start),
        })
    }

    fn parse_let_declaration(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Tien)?;
        let name = self.expect_identifier()?;
        let value = if self.match_token(&TokenKind::Equal) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        Ok(Statement::VariableDecl {
            name,
            value,
            is_const: false,
            span: self.span_from(start.start),
        })
    }

    fn parse_function(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Facc)?;
        let name = self.expect_identifier()?;
        let params = self.parse_parameters()?;
        let body = self.parse_block_body()?;
        Ok(Statement::FunctionDecl {
            name,
            params,
            body,
            is_async: false,
            span: self.span_from(start.start),
        })
    }

    fn parse_async_function(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Mo)?;
        self.expect(&TokenKind::Vir)?;
        self.expect(&TokenKind::Facc)?;
        let name = self.expect_identifier()?;
        let params = self.parse_parameters()?;
        let body = self.parse_block_body()?;
        Ok(Statement::FunctionDecl {
            name,
            params,
            body,
            is_async: true,
            span: self.span_from(start.start),
        })
    }

    /// `(a, b = 1, ...resto)`
    fn parse_parameters(&mut self) -> Result<Vec<Pattern>, ParseError> {
        self.expect(&TokenKind::LeftParen)?;
        let mut params = Vec::new();
        loop {
            self.skip_newlines();
            if self.check(&TokenKind::RightParen) {
                break;
            }
            if let Some(Pattern::Rest { .. }) = params.last() {
                return Err(ParseError::new(
                    "The rest parameter must be the last one",
                    self.current_span(),
                ));
            }
            params.push(self.parse_parameter()?);
            self.skip_newlines();
            if !self.match_token(&TokenKind::Comma) {
                break;
            }
        }
        self.expect(&TokenKind::RightParen)?;
        Ok(params)
    }

    fn parse_parameter(&mut self) -> Result<Pattern, ParseError> {
        if self.match_token(&TokenKind::Ellipsis) {
            let argument = self.parse_binding_target()?;
            return Ok(Pattern::Rest {
                argument: Box::new(argument),
            });
        }
        let target = self.parse_binding_target()?;
        if self.match_token(&TokenKind::Equal) {
            let default = self.parse_assignment()?;
            return Ok(Pattern::Default {
                target: Box::new(target),
                default: Box::new(default),
            });
        }
        Ok(target)
    }

    fn parse_binding_target(&mut self) -> Result<Pattern, ParseError> {
        let span = self.current_span();
        let name = self.expect_identifier()?;
        Ok(Pattern::Identifier { name, span })
    }

    fn parse_return(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Piglie)?;
        let value = if !self.check(&TokenKind::Newline)
            && !self.check(&TokenKind::RightBrace)
            && !self.check(&TokenKind::Semicolon)
            && !self.is_at_end()
        {
            Some(self.parse_expression()?)
        } else {
            None
        };
        Ok(Statement::Return {
            value,
            span: self.span_from(start.start),
        })
    }

    fn parse_if(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Si)?;
        self.expect(&TokenKind::LeftParen)?;
        let condition = self.parse_expression()?;
        self.expect(&TokenKind::RightParen)?;
        let then_branch = self.parse_block_body()?;
        let else_branch = if self.match_token(&TokenKind::Sinno) {
            if self.check(&TokenKind::Si) {
                Some(vec![self.parse_if()?])
            } else {
                Some(self.parse_block_body()?)
            }
        } else {
            None
        };
        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
            span: self.span_from(start.start),
        })
    }

    fn parse_while(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Mentre)?;
        self.expect(&TokenKind::Che)?;
        self.expect(&TokenKind::LeftParen)?;
        let condition = self.parse_expression()?;
        self.expect(&TokenKind::RightParen)?;
        let body = self.parse_block_body()?;
        Ok(Statement::While {
            condition,
            body,
            span: self.span_from(start.start),
        })
    }

    fn parse_for(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Pe)?;
        // "ogni" is now optional for backwards compatibility
        self.match_token(&TokenKind::Ogni);
        self.expect(&TokenKind::LeftParen)?;
        let init = if self.check(&TokenKind::Tien) {
            Some(Box::new(self.parse_let_declaration()?))
        } else if !self.check(&TokenKind::Semicolon) {
            let expr = self.parse_expression()?;
            Some(Box::new(Statement::Expression {
                span: self.current_span(),
                expression: expr,
            }))
        } else {
            None
        };
        self.expect(&TokenKind::Semicolon)?;
        let condition = if !self.check(&TokenKind::Semicolon) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.expect(&TokenKind::Semicolon)?;
        let update = if !self.check(&TokenKind::RightParen) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.expect(&TokenKind::RightParen)?;
        let body = self.parse_block_body()?;
        Ok(Statement::For {
            init,
            condition,
            update,
            body,
            span: self.span_from(start.start),
        })
    }

    fn parse_break(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Rompe)?;
        Ok(Statement::Break {
            span: self.span_from(start.start),
        })
    }

    fn parse_continue(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Salta)?;
        Ok(Statement::Continue {
            span: self.span_from(start.start),
        })
    }

    fn parse_debugger(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Fermete)?;
        Ok(Statement::Debugger {
            span: self.span_from(start.start),
        })
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
        } else {
            None
        };
        let catch_body = self.parse_block_body()?;
        Ok(Statement::TryCatch {
            try_body,
            catch_param,
            catch_body,
            span: self.span_from(start.start),
        })
    }

    fn parse_throw(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Iett)?;
        let value = self.parse_expression()?;
        Ok(Statement::Throw {
            value,
            span: self.span_from(start.start),
        })
    }

    fn parse_class(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Na)?;
        self.expect(&TokenKind::Famiglie)?;
        let name = self.expect_identifier()?;

        // Optional inheritance:
        // - "figlio 'e Base"
        // - "figlio Base" (shortcut)
        let super_class = if self.match_token(&TokenKind::Figlio) {
            // Connector is optional to preserve backwards compatibility.
            self.match_token(&TokenKind::De);
            Some(self.expect_identifier()?)
        } else {
            None
        };

        self.expect(&TokenKind::LeftBrace)?;
        let mut methods = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            while self.check(&TokenKind::Newline) {
                self.advance();
            }
            if self.check(&TokenKind::RightBrace) {
                break;
            }
            methods.push(self.parse_class_method()?);
        }
        self.expect(&TokenKind::RightBrace)?;
        Ok(Statement::ClassDecl {
            name,
            super_class,
            methods,
            span: self.span_from(start.start),
        })
    }

    fn parse_class_method(&mut self) -> Result<ClassMethod, ParseError> {
        let start = self.current_span();
        let is_static = self.match_token(&TokenKind::Fisso);

        let is_async = if self.match_token(&TokenKind::Mo) {
            self.expect(&TokenKind::Vir)?;
            true
        } else {
            false
        };

        // "facc" inside classes is optional.
        self.match_token(&TokenKind::Facc);

        let name = self.expect_identifier()?;
        let params = self.parse_parameters()?;
        let body = self.parse_block_body()?;

        Ok(ClassMethod {
            name,
            params,
            body,
            is_async,
            is_static,
            span: self.span_from(start.start),
        })
    }

    fn parse_import(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Chiamm)?;
        self.expect(&TokenKind::LeftBrace)?;
        let mut specifiers = Vec::new();
        if !self.check(&TokenKind::RightBrace) {
            loop {
                let imported = self.expect_identifier()?;
                specifiers.push(ImportSpecifier {
                    imported: imported.clone(),
                    local: imported,
                });
                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
        }
        self.expect(&TokenKind::RightBrace)?;
        self.expect(&TokenKind::Da)?;
        let source = self.expect_string()?;
        Ok(Statement::Import {
            specifiers,
            source,
            span: self.span_from(start.start),
        })
    }

    fn parse_export(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Mann)?;
        self.expect(&TokenKind::For)?;
        if self.match_token(&TokenKind::Predefinit) {
            let value = self.parse_expression()?;
            self.expect_statement_end()?;
            Ok(Statement::Export {
                declaration: None,
                default_value: Some(value),
                span: self.span_from(start.start),
            })
        } else {
            let decl = self.parse_statement()?;
            Ok(Statement::Export {
                declaration: Some(Box::new(decl)),
                default_value: None,
                span: self.span_from(start.start),
            })
        }
    }

    fn parse_block(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        let statements = self.parse_block_body()?;
        Ok(Statement::Block {
            statements,
            span: self.span_from(start.start),
        })
    }

    fn parse_block_body(&mut self) -> Result<Vec<Statement>, ParseError> {
        self.expect(&TokenKind::LeftBrace)?;
        let mut statements = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            while self.check(&TokenKind::Newline) {
                self.advance();
            }
            if self.check(&TokenKind::RightBrace) {
                break;
            }
            statements.push(self.parse_statement()?);
        }
        self.expect(&TokenKind::RightBrace)?;
        Ok(statements)
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        let expr = self.parse_expression()?;
        Ok(Statement::Expression {
            expression: expr,
            span: self.span_from(start.start),
        })
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expression, ParseError> {
        let expr = self.parse_ternary()?;
        let operator = match self.peek().kind {
            TokenKind::Equal => AssignOp::Assign,
            TokenKind::PlusEqual => AssignOp::Add,
            TokenKind::MinusEqual => AssignOp::Subtract,
            TokenKind::StarEqual => AssignOp::Multiply,
            TokenKind::SlashEqual => AssignOp::Divide,
            TokenKind::PercentEqual => AssignOp::Modulo,
            TokenKind::StarStarEqual => AssignOp::Power,
            TokenKind::QuestionQuestionEqual => AssignOp::Nullish,
            TokenKind::OrEqual => AssignOp::Or,
            TokenKind::AndEqual => AssignOp::And,
            _ => return Ok(expr),
        };
        let op_span = self.advance().span;
        check_assignment_target(&expr, op_span)?;
        let value = self.parse_assignment()?;
        let span = self.span_from(expr.span().start);
        Ok(Expression::Assignment {
            target: Box::new(expr),
            operator,
            value: Box::new(value),
            span,
        })
    }

    fn parse_ternary(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_or()?;
        if self.match_token(&TokenKind::Question) {
            let consequent = self.parse_expression()?;
            self.expect(&TokenKind::Colon)?;
            let alternate = self.parse_ternary()?;
            let span = self.span_from(expr.span().start);
            expr = Expression::Ternary {
                condition: Box::new(expr),
                consequent: Box::new(consequent),
                alternate: Box::new(alternate),
                span,
            };
        }
        Ok(expr)
    }

    fn parse_or(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_and()?;
        // Supports both "o" and "||" style
        while self.match_token(&TokenKind::Or) {
            let right = self.parse_and()?;
            let span = self.span_from(expr.span().start);
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOp::Or,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_equality()?;
        // Supports both "e" and "pure" (both mean AND)
        while self.match_token(&TokenKind::And) || self.match_token(&TokenKind::Pure) {
            let right = self.parse_equality()?;
            let span = self.span_from(expr.span().start);
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOp::And,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_comparison()?;
        loop {
            let op = if self.match_token(&TokenKind::EqualEqualEqual) {
                BinaryOp::StrictEqual
            } else if self.match_token(&TokenKind::EqualEqual) {
                BinaryOp::Equal
            } else if self.match_token(&TokenKind::BangEqualEqual) {
                BinaryOp::StrictNotEqual
            } else if self.match_token(&TokenKind::BangEqual) {
                BinaryOp::NotEqual
            } else {
                break;
            };
            let right = self.parse_comparison()?;
            let span = self.span_from(expr.span().start);
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_term()?;
        loop {
            let op = if self.match_token(&TokenKind::Less) {
                BinaryOp::LessThan
            } else if self.match_token(&TokenKind::LessEqual) {
                BinaryOp::LessEqual
            } else if self.match_token(&TokenKind::Greater) {
                BinaryOp::GreaterThan
            } else if self.match_token(&TokenKind::GreaterEqual) {
                BinaryOp::GreaterEqual
            } else if self.match_token(&TokenKind::EUno) {
                BinaryOp::InstanceOf
            } else if self.check(&TokenKind::E) && self.check_next_identifier("uno") {
                self.advance();
                self.advance();
                BinaryOp::InstanceOf
            } else {
                break;
            };
            let right = self.parse_term()?;
            let span = self.span_from(expr.span().start);
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_factor()?;
        loop {
            let op = if self.match_token(&TokenKind::Plus) {
                BinaryOp::Add
            } else if self.match_token(&TokenKind::Minus) {
                BinaryOp::Subtract
            } else {
                break;
            };
            let right = self.parse_factor()?;
            let span = self.span_from(expr.span().start);
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_power()?;
        loop {
            let op = if self.match_token(&TokenKind::Star) {
                BinaryOp::Multiply
            } else if self.match_token(&TokenKind::Slash) {
                BinaryOp::Divide
            } else if self.match_token(&TokenKind::Percent) {
                BinaryOp::Modulo
            } else {
                break;
            };
            let right = self.parse_power()?;
            let span = self.span_from(expr.span().start);
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
                span,
            };
        }
        Ok(expr)
    }

    fn parse_power(&mut self) -> Result<Expression, ParseError> {
        let expr = self.parse_unary()?;
        if self.match_token(&TokenKind::StarStar) {
            let right = self.parse_power()?;
            let span = self.span_from(expr.span().start);
            return Ok(Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOp::Power,
                right: Box::new(right),
                span,
            });
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expression, ParseError> {
        // Numeric negation: -x
        if self.match_token(&TokenKind::Minus) {
            let start = self.previous().span;
            let operand = self.parse_unary()?;
            return Ok(Expression::Unary {
                operator: UnaryOp::Negate,
                operand: Box::new(operand),
                span: self.span_from(start.start),
            });
        }
        // Logical negation: no, !, manco (`no` alone is a name, as in `tien no = 1`)
        if (self.check(&TokenKind::Not)
            && (self.peek().literal == "!" || self.next_starts_operand()))
            || self.check(&TokenKind::Manco)
        {
            self.advance();
            let start = self.previous().span;
            let operand = self.parse_unary()?;
            return Ok(Expression::Unary {
                operator: UnaryOp::Not,
                operand: Box::new(operand),
                span: self.span_from(start.start),
            });
        }
        // Await keyword: aspett
        if self.match_token(&TokenKind::Aspett) {
            let start = self.previous().span;
            let argument = self.parse_unary()?;
            return Ok(Expression::Await {
                argument: Box::new(argument),
                span: self.span_from(start.start),
            });
        }
        // Delete keyword: leva (a name when no operand follows)
        if self.check(&TokenKind::Leva) && self.next_starts_operand() {
            self.advance();
            let start = self.previous().span;
            let operand = self.parse_unary()?;
            return Ok(Expression::Delete {
                operand: Box::new(operand),
                span: self.span_from(start.start),
            });
        }
        // Prefix update: ++x, --x
        if self.check(&TokenKind::PlusPlus) || self.check(&TokenKind::MinusMinus) {
            let token = self.advance();
            let (start, operator) = (token.span, update_op(&token.kind));
            let argument = self.parse_unary()?;
            check_assignment_target(&argument, start)?;
            return Ok(Expression::Update {
                operator,
                prefix: true,
                argument: Box::new(argument),
                span: self.span_from(start.start),
            });
        }
        // Typeof: chè è (or plain `typeof`)
        if self.match_token(&TokenKind::CheE) {
            let start = self.previous().span;
            if self.previous().literal == "chè" {
                self.expect(&TokenKind::E)?;
            }
            let operand = self.parse_unary()?;
            return Ok(Expression::TypeOf {
                operand: Box::new(operand),
                span: self.span_from(start.start),
            });
        }
        self.parse_postfix()
    }

    /// Postfix update: `x++`, `a.b--` (on the same line, since a newline ends the statement)
    fn parse_postfix(&mut self) -> Result<Expression, ParseError> {
        let expr = self.parse_call()?;
        if self.check(&TokenKind::PlusPlus) || self.check(&TokenKind::MinusMinus) {
            let token = self.advance();
            let (op_span, operator) = (token.span, update_op(&token.kind));
            check_assignment_target(&expr, op_span)?;
            let span = self.span_from(expr.span().start);
            return Ok(Expression::Update {
                operator,
                prefix: false,
                argument: Box::new(expr),
                span,
            });
        }
        Ok(expr)
    }

    fn parse_call(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.match_token(&TokenKind::LeftParen) {
                let arguments = self.parse_arguments()?;
                let span = self.span_from(expr.span().start);
                expr = Expression::Call {
                    callee: Box::new(expr),
                    arguments,
                    span,
                };
            } else if self.match_token(&TokenKind::Dot) {
                let name = self.expect_property_name()?;
                let span = self.span_from(expr.span().start);
                expr = Expression::Member {
                    object: Box::new(expr),
                    property: Box::new(Expression::Identifier {
                        name,
                        span: self.previous().span,
                    }),
                    computed: false,
                    span,
                };
            } else if self.match_token(&TokenKind::LeftBracket) {
                let property = self.parse_expression()?;
                self.expect(&TokenKind::RightBracket)?;
                let span = self.span_from(expr.span().start);
                expr = Expression::Member {
                    object: Box::new(expr),
                    property: Box::new(property),
                    computed: true,
                    span,
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    /// Whether the tokens at the cursor are arrow parameters: a name or a
    /// balanced `( … )` group followed by `=>`.
    fn at_arrow_params(&self) -> bool {
        let token = self.peek();
        if matches!(token.kind, TokenKind::Identifier(_)) || contextual_name(token).is_some() {
            return self.check_next(&TokenKind::Arrow);
        }
        if !self.check(&TokenKind::LeftParen) {
            return false;
        }
        let mut depth = 0usize;
        for (i, t) in self.tokens[self.current..].iter().enumerate() {
            match t.kind {
                TokenKind::LeftParen | TokenKind::LeftBracket | TokenKind::LeftBrace => depth += 1,
                TokenKind::RightParen | TokenKind::RightBracket | TokenKind::RightBrace => {
                    depth -= 1;
                    if depth == 0 {
                        return self.check_at(i + 1, &TokenKind::Arrow);
                    }
                }
                TokenKind::Eof => return false,
                _ => {}
            }
        }
        false
    }

    fn parse_arrow(&mut self, start: Span, is_async: bool) -> Result<Expression, ParseError> {
        let params = if self.check(&TokenKind::LeftParen) {
            self.parse_parameters()?
        } else {
            vec![self.parse_binding_target()?]
        };
        self.expect(&TokenKind::Arrow)?;
        let body = if self.check(&TokenKind::LeftBrace) {
            ArrowBody::Block(self.parse_block_body()?)
        } else {
            ArrowBody::Expression(Box::new(self.parse_assignment()?))
        };
        Ok(Expression::ArrowFunction {
            params,
            body,
            is_async,
            span: self.span_from(start.start),
        })
    }

    /// Call arguments after `(`; they may span lines and use `...spread`.
    fn parse_arguments(&mut self) -> Result<Vec<Expression>, ParseError> {
        let mut args = Vec::new();
        loop {
            self.skip_newlines();
            if self.check(&TokenKind::RightParen) {
                break;
            }
            args.push(self.parse_spread_or_expression()?);
            self.skip_newlines();
            if !self.match_token(&TokenKind::Comma) {
                break;
            }
        }
        self.expect(&TokenKind::RightParen)?;
        Ok(args)
    }

    fn parse_spread_or_expression(&mut self) -> Result<Expression, ParseError> {
        if self.match_token(&TokenKind::Ellipsis) {
            let start = self.previous().span;
            let argument = self.parse_assignment()?;
            return Ok(Expression::Spread {
                argument: Box::new(argument),
                span: self.span_from(start.start),
            });
        }
        self.parse_expression()
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        // Arrow functions: `x => …`, `(a, b) => …`, `mo vir (x) => …`
        if self.check(&TokenKind::Mo) && self.check_next(&TokenKind::Vir) {
            let start = self.current_span();
            self.advance();
            self.advance();
            return self.parse_arrow(start, true);
        }
        if self.at_arrow_params() {
            let start = self.current_span();
            return self.parse_arrow(start, false);
        }

        // Short keywords are names unless they start their own construct here
        let starts_construct = match self.peek().kind {
            TokenKind::Nu => self.check_next(&TokenKind::Bell),
            TokenKind::Scrive => self.check_next_identifier("a"),
            TokenKind::Mo => self.check_next(&TokenKind::Vir),
            _ => false,
        };
        if let Some(name) = contextual_name(self.peek()).filter(|_| !starts_construct) {
            let name = name.to_string();
            let span = self.advance().span;
            return Ok(Expression::Identifier { name, span });
        }

        let token = self.advance();
        let span = token.span;

        match &token.kind {
            TokenKind::Number(n) => Ok(Expression::Number { value: *n, span }),
            TokenKind::String(s) => Ok(Expression::String {
                value: s.clone(),
                span,
            }),
            TokenKind::Template {
                quasis,
                expressions,
            } => {
                let quasis = quasis.clone();
                let expressions = expressions
                    .clone()
                    .into_iter()
                    .map(|tokens| parse_interpolation(tokens, span))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Expression::Template {
                    quasis,
                    expressions,
                    span,
                })
            }
            TokenKind::Overo => Ok(Expression::Boolean { value: true, span }),
            TokenKind::Sfols => Ok(Expression::Boolean { value: false, span }),
            TokenKind::Nisciun => Ok(Expression::Null { span }),
            TokenKind::Boh => Ok(Expression::Undefined { span }),
            TokenKind::Stu => {
                self.expect(&TokenKind::Cos)?;
                Ok(Expression::This {
                    span: self.span_from(span.start),
                })
            }
            TokenKind::OPate => Ok(Expression::Super {
                span: self.span_from(span.start),
            }),
            TokenKind::Nu => {
                self.expect(&TokenKind::Bell)?;
                let callee = self.parse_call()?;
                let span = self.span_from(span.start);
                if let Expression::Call {
                    callee: inner,
                    arguments,
                    ..
                } = callee
                {
                    Ok(Expression::New {
                        callee: inner,
                        arguments,
                        span,
                    })
                } else {
                    Ok(Expression::New {
                        callee: Box::new(callee),
                        arguments: vec![],
                        span,
                    })
                }
            }
            TokenKind::Stamm => {
                self.expect_identifier_a()?;
                self.expect(&TokenKind::Di)?;
                self.expect(&TokenKind::LeftParen)?;
                let arguments = self.parse_arguments()?;
                Ok(Expression::ConsoleLog {
                    arguments,
                    span: self.span_from(span.start),
                })
            }
            // console.warn() - "avvis a dì(...)"
            TokenKind::Avvis => {
                self.expect_identifier_a()?;
                self.expect(&TokenKind::Di)?;
                self.expect(&TokenKind::LeftParen)?;
                let arguments = self.parse_arguments()?;
                Ok(Expression::ConsoleWarn {
                    arguments,
                    span: self.span_from(span.start),
                })
            }
            // console.error() - "scrive a dì(...)"
            TokenKind::Scrive => {
                self.expect_identifier_a()?;
                self.expect(&TokenKind::Di)?;
                self.expect(&TokenKind::LeftParen)?;
                let arguments = self.parse_arguments()?;
                Ok(Expression::ConsoleError {
                    arguments,
                    span: self.span_from(span.start),
                })
            }
            TokenKind::Identifier(name) => Ok(Expression::Identifier {
                name: name.clone(),
                span,
            }),
            TokenKind::LeftParen => {
                let expr = self.parse_expression()?;
                self.expect(&TokenKind::RightParen)?;
                Ok(expr)
            }
            TokenKind::LeftBracket => {
                let mut elements = Vec::new();
                while self.check(&TokenKind::Newline) {
                    self.advance();
                }
                if !self.check(&TokenKind::RightBracket) {
                    elements.push(self.parse_spread_or_expression()?);
                    while self.match_token(&TokenKind::Comma) {
                        while self.check(&TokenKind::Newline) {
                            self.advance();
                        }
                        if self.check(&TokenKind::RightBracket) {
                            break;
                        }
                        elements.push(self.parse_spread_or_expression()?);
                    }
                }
                while self.check(&TokenKind::Newline) {
                    self.advance();
                }
                self.expect(&TokenKind::RightBracket)?;
                Ok(Expression::Array {
                    elements,
                    span: self.span_from(span.start),
                })
            }
            TokenKind::LeftBrace => {
                let mut properties = Vec::new();
                // Skip initial newlines
                while self.check(&TokenKind::Newline) {
                    self.advance();
                }
                if !self.check(&TokenKind::RightBrace) {
                    loop {
                        // Skip newlines before the key
                        while self.check(&TokenKind::Newline) {
                            self.advance();
                        }
                        if self.check(&TokenKind::RightBrace) {
                            break;
                        }
                        if self.match_token(&TokenKind::Ellipsis) {
                            properties.push(Property::Spread(self.parse_assignment()?));
                        } else {
                            let key = self.expect_property_name()?;
                            self.expect(&TokenKind::Colon)?;
                            let value = self.parse_expression()?;
                            properties.push(Property::KeyValue { key, value });
                        }
                        // Skip newlines after the value
                        while self.check(&TokenKind::Newline) {
                            self.advance();
                        }
                        if !self.match_token(&TokenKind::Comma) {
                            break;
                        }
                        // Skip newlines after the comma
                        while self.check(&TokenKind::Newline) {
                            self.advance();
                        }
                        if self.check(&TokenKind::RightBrace) {
                            break;
                        }
                    }
                }
                // Skip trailing newlines
                while self.check(&TokenKind::Newline) {
                    self.advance();
                }
                self.expect(&TokenKind::RightBrace)?;
                Ok(Expression::Object {
                    properties,
                    span: self.span_from(span.start),
                })
            }
            _ => Err(ParseError::new(
                format!("What is '{}' here? Expected an expression!", token.kind),
                span,
            )),
        }
    }

    // === Helpers ===

    fn skip_newlines(&mut self) {
        while self.check(&TokenKind::Newline) {
            self.advance();
        }
    }
    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    fn check(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind)
    }
    fn check_next(&self, kind: &TokenKind) -> bool {
        if self.current + 1 >= self.tokens.len() {
            return false;
        }
        std::mem::discriminant(&self.tokens[self.current + 1].kind) == std::mem::discriminant(kind)
    }
    fn check_at(&self, offset: usize, kind: &TokenKind) -> bool {
        self.tokens
            .get(self.current + offset)
            .map(|t| std::mem::discriminant(&t.kind) == std::mem::discriminant(kind))
            .unwrap_or(false)
    }
    /// Whether the token after the current one can begin an operand, so that
    /// `no x` is a negation while `no` alone (`f(no)`, `no = 1`) is a name.
    fn next_starts_operand(&self) -> bool {
        self.tokens
            .get(self.current + 1)
            .map(starts_operand)
            .unwrap_or(false)
    }
    fn check_next_identifier(&self, name: &str) -> bool {
        matches!(self.tokens.get(self.current + 1).map(|t| &t.kind), Some(TokenKind::Identifier(n)) if n == name)
    }
    fn match_token(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }
    fn expect(&mut self, kind: &TokenKind) -> Result<&Token, ParseError> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(ParseError::new(
                format!("Expected '{}', but found '{}'", kind, self.peek().kind),
                self.peek().span,
            ))
        }
    }
    fn expect_identifier(&mut self) -> Result<String, ParseError> {
        let token = self.advance();
        if let TokenKind::Identifier(name) = &token.kind {
            Ok(name.clone())
        } else if let Some(name) = contextual_name(token) {
            Ok(name.to_string())
        } else {
            Err(ParseError::new(
                format!("Expected an identifier, not '{}'", token.kind),
                token.span,
            ))
        }
    }
    /// Property names (`x.si`, `{ for: 1 }`) can be any word, keywords included, as in JS.
    fn expect_property_name(&mut self) -> Result<String, ParseError> {
        let token = self.advance();
        if let TokenKind::Identifier(name) = &token.kind {
            return Ok(name.clone());
        }
        if is_word(&token.literal) {
            return Ok(token.literal.clone());
        }
        Err(ParseError::new(
            format!("Expected an identifier, not '{}'", token.kind),
            token.span,
        ))
    }
    fn expect_string(&mut self) -> Result<String, ParseError> {
        let token = self.advance();
        if let TokenKind::String(s) = &token.kind {
            Ok(s.clone())
        } else {
            Err(ParseError::new(
                format!("Expected a string, not '{}'", token.kind),
                token.span,
            ))
        }
    }
    // Expect the identifier "a" specifically (used in "stamm a dì", "avvis a dì", etc.)
    fn expect_identifier_a(&mut self) -> Result<(), ParseError> {
        let token = self.advance();
        if let TokenKind::Identifier(name) = &token.kind {
            if name == "a" {
                return Ok(());
            }
        }
        Err(ParseError::new(
            format!("Expected 'a', but found '{}'", token.kind),
            token.span,
        ))
    }
    fn current_span(&self) -> Span {
        self.peek().span
    }
    fn span_from(&self, start: usize) -> Span {
        Span::new(
            start,
            self.previous().span.end,
            self.previous().span.line,
            0,
        )
    }
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if matches!(
                self.peek().kind,
                TokenKind::Chist
                    | TokenKind::Tien
                    | TokenKind::Facc
                    | TokenKind::Si
                    | TokenKind::Mentre
                    | TokenKind::Pe
                    | TokenKind::Piglie
                    | TokenKind::Na
                    | TokenKind::Chiamm
                    | TokenKind::Mann
            ) {
                return;
            }
            self.advance();
        }
    }
}

/// Short keywords that are also valid names. The lexer always emits them as
/// keywords; the parser reads them as names wherever their construct cannot start.
fn contextual_name(token: &Token) -> Option<&str> {
    let contextual = matches!(
        token.kind,
        TokenKind::And
            | TokenKind::Or
            | TokenKind::Not
            | TokenKind::Da
            | TokenKind::De
            | TokenKind::Na
            | TokenKind::Nu
            | TokenKind::Mo
            | TokenKind::Che
            | TokenKind::Cos
            | TokenKind::Caso
            | TokenKind::Pure
            | TokenKind::For
            | TokenKind::Leva
            | TokenKind::Scrive
    );
    // `!` and `&&` share token kinds with `no` and `e` but are never names
    (contextual && is_word(&token.literal)).then_some(token.literal.as_str())
}

fn is_word(s: &str) -> bool {
    let mut chars = s.chars();
    chars
        .next()
        .map(|c| c.is_alphabetic() || c == '_')
        .unwrap_or(false)
        && chars.all(|c| c.is_alphanumeric() || c == '_')
}

/// Tokens that can begin an operand of a prefix operator. The infix words
/// `e`, `o` and `pure` do not: `no e x` reads as `no && x`.
fn starts_operand(token: &Token) -> bool {
    let infix = matches!(token.kind, TokenKind::And | TokenKind::Or | TokenKind::Pure);
    (contextual_name(token).is_some() && !infix)
        || matches!(
            token.kind,
            TokenKind::Identifier(_)
                | TokenKind::Number(_)
                | TokenKind::String(_)
                | TokenKind::Template { .. }
                | TokenKind::LeftParen
                | TokenKind::LeftBracket
                | TokenKind::LeftBrace
                | TokenKind::Overo
                | TokenKind::Sfols
                | TokenKind::Nisciun
                | TokenKind::Boh
                | TokenKind::Stu
                | TokenKind::OPate
                | TokenKind::Stamm
                | TokenKind::Avvis
                | TokenKind::Not
                | TokenKind::Manco
                | TokenKind::Minus
                | TokenKind::Aspett
                | TokenKind::CheE
        )
}

/// Only names and properties can be assigned or updated (`x = 1`, `a.b += 1`, `i++`).
fn check_assignment_target(target: &Expression, op_span: Span) -> Result<(), ParseError> {
    match target {
        Expression::Identifier { .. } | Expression::Member { .. } => Ok(()),
        _ => Err(ParseError::new("Invalid assignment target", op_span)),
    }
}

fn update_op(kind: &TokenKind) -> UpdateOp {
    if matches!(kind, TokenKind::PlusPlus) {
        UpdateOp::Increment
    } else {
        UpdateOp::Decrement
    }
}

/// Parses the tokens of one `${ … }` as a single expression.
fn parse_interpolation(
    mut tokens: Vec<Token>,
    template_span: Span,
) -> Result<Expression, ParseError> {
    let end = tokens.last().map(|t| t.span).unwrap_or(template_span);
    tokens.retain(|t| t.kind != TokenKind::Newline);
    if tokens.is_empty() {
        return Err(ParseError::new("Expected an expression inside '${}'", end));
    }
    tokens.push(Token::new(TokenKind::Eof, end, String::new()));
    let mut parser = Parser::new(tokens);
    let expr = parser.parse_expression()?;
    if !parser.is_at_end() {
        let token = parser.peek();
        return Err(ParseError::new(
            format!("Expected '}}', but found '{}'", token.kind),
            token.span,
        ));
    }
    Ok(expr)
}
