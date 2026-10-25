//! Parser for FratmScript

mod ast;
mod jsx;

use crate::lexer::{Span, Token, TokenKind};
pub use ast::*;
pub use jsx::{decode_entities, jsx_text};

/// Name, parent and members of a class declaration or expression.
type ClassParts = (Option<String>, Option<String>, Vec<ClassMember>);

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

/// How deep the parser may recurse (blocks, expressions, operators) before
/// it reports an error instead of overflowing the stack. Each nested
/// bracket costs about three levels, so this allows ~100 nested `( )`,
/// which still fits the 1 MB stack of the WASM build.
const MAX_DEPTH: usize = 300;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    /// Whether the innermost function is a generator, so `caccia` is allowed
    in_generator: bool,
    /// Current recursion depth, bounded by `MAX_DEPTH`
    depth: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            in_generator: false,
            depth: 0,
        }
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
        // `fore:` before a statement is a label
        let named = matches!(self.peek().kind, TokenKind::Identifier(_))
            || contextual_name(self.peek()).is_some();
        if named && self.check_next(&TokenKind::Colon) {
            return self.parse_labeled();
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
        if self.at_switch() {
            return self.parse_switch();
        }
        // `primm fa {` (do...while); both words stay ordinary names elsewhere
        if self.check_identifier_at(0, "primm")
            && self.check_identifier_at(1, "fa")
            && self.check_at(2, &TokenKind::LeftBrace)
        {
            let stmt = self.parse_do_while()?;
            self.expect_statement_end()?;
            return Ok(stmt);
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
        } else if self.check(&TokenKind::Chiamm) && !self.check_next(&TokenKind::LeftParen) {
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
        self.expect_e()?;
        let mut declarations = Vec::new();
        loop {
            let target = self.parse_binding_target()?;
            self.expect(&TokenKind::Equal)?;
            let value = self.parse_expression()?;
            declarations.push(Declarator {
                target,
                value: Some(value),
            });
            if !self.match_token(&TokenKind::Comma) {
                break;
            }
            self.skip_newlines();
        }
        Ok(Statement::VariableDecl {
            declarations,
            is_const: true,
            span: self.span_from(start.start),
        })
    }

    fn parse_let_declaration(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Tien)?;
        let mut declarations = Vec::new();
        loop {
            let target = self.parse_binding_target()?;
            let value = if self.match_token(&TokenKind::Equal) {
                Some(self.parse_expression()?)
            } else {
                None
            };
            if value.is_none() && !matches!(target, Pattern::Identifier { .. }) {
                return Err(ParseError::new(
                    "Destructuring needs a value: add '= …'",
                    self.current_span(),
                ));
            }
            declarations.push(Declarator { target, value });
            if !self.match_token(&TokenKind::Comma) {
                break;
            }
            self.skip_newlines();
        }
        Ok(Statement::VariableDecl {
            declarations,
            is_const: false,
            span: self.span_from(start.start),
        })
    }

    fn parse_function(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Facc)?;
        let is_generator = self.match_token(&TokenKind::Star);
        let name = self.expect_identifier()?;
        let params = self.parse_parameters()?;
        let body = self.parse_function_body(is_generator)?;
        Ok(Statement::FunctionDecl {
            name,
            params,
            body,
            is_async: false,
            is_generator,
            span: self.span_from(start.start),
        })
    }

    fn parse_async_function(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Mo)?;
        self.expect(&TokenKind::Vir)?;
        self.expect(&TokenKind::Facc)?;
        let is_generator = self.match_token(&TokenKind::Star);
        let name = self.expect_identifier()?;
        let params = self.parse_parameters()?;
        let body = self.parse_function_body(is_generator)?;
        Ok(Statement::FunctionDecl {
            name,
            params,
            body,
            is_async: true,
            is_generator,
            span: self.span_from(start.start),
        })
    }

    /// A function body; `caccia` is allowed in it only for a generator.
    fn parse_function_body(&mut self, is_generator: bool) -> Result<Vec<Statement>, ParseError> {
        let outer = std::mem::replace(&mut self.in_generator, is_generator);
        let body = self.parse_block_body();
        self.in_generator = outer;
        body
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

    /// A name or a destructuring pattern: `x`, `[a, ...b]`, `{ a, b: { c } }`.
    fn parse_binding_target(&mut self) -> Result<Pattern, ParseError> {
        let start = self.current_span();
        if self.match_token(&TokenKind::LeftBracket) {
            let mut elements = Vec::new();
            loop {
                self.skip_newlines();
                if self.check(&TokenKind::RightBracket) {
                    break;
                }
                if let Some(Some(Pattern::Rest { .. })) = elements.last() {
                    return Err(ParseError::new(
                        "The rest element must be the last one",
                        self.current_span(),
                    ));
                }
                if self.check(&TokenKind::Comma) {
                    self.advance();
                    elements.push(None);
                    continue;
                }
                elements.push(Some(self.parse_parameter()?));
                self.skip_newlines();
                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
            self.expect(&TokenKind::RightBracket)?;
            return Ok(Pattern::Array {
                elements,
                span: self.span_from(start.start),
            });
        }
        if self.match_token(&TokenKind::LeftBrace) {
            let mut properties = Vec::new();
            loop {
                self.skip_newlines();
                if self.check(&TokenKind::RightBrace) {
                    break;
                }
                if let Some(ObjectPatternProperty::Rest(_)) = properties.last() {
                    return Err(ParseError::new(
                        "The rest element must be the last one",
                        self.current_span(),
                    ));
                }
                properties.push(self.parse_object_pattern_property()?);
                self.skip_newlines();
                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
            self.skip_newlines();
            self.expect(&TokenKind::RightBrace)?;
            return Ok(Pattern::Object {
                properties,
                span: self.span_from(start.start),
            });
        }
        let name = self.expect_identifier()?;
        Ok(Pattern::Identifier { name, span: start })
    }

    fn parse_object_pattern_property(&mut self) -> Result<ObjectPatternProperty, ParseError> {
        if self.match_token(&TokenKind::Ellipsis) {
            let span = self.current_span();
            let name = self.expect_identifier()?;
            return Ok(ObjectPatternProperty::Rest(Pattern::Identifier {
                name,
                span,
            }));
        }
        let key_token = self.peek().clone();
        let key = self.parse_property_key()?;
        if self.match_token(&TokenKind::Colon) {
            let value = self.parse_parameter()?;
            return Ok(ObjectPatternProperty::Property {
                key,
                value,
                shorthand: false,
            });
        }
        // Shorthand `{ nome }` / `{ nome = 1 }` binds a variable with the key's name
        let is_name = matches!(key_token.kind, TokenKind::Identifier(_))
            || contextual_name(&key_token).is_some();
        let PropertyKey::Name(name) = &key else {
            return Err(ParseError::new(
                format!("Expected ':', but found '{}'", self.peek().kind),
                self.peek().span,
            ));
        };
        if !is_name {
            return Err(ParseError::new(
                format!("Expected an identifier, not '{}'", key_token.kind),
                key_token.span,
            ));
        }
        let mut value = Pattern::Identifier {
            name: name.clone(),
            span: key_token.span,
        };
        if self.match_token(&TokenKind::Equal) {
            let default = self.parse_assignment()?;
            value = Pattern::Default {
                target: Box::new(value),
                default: Box::new(default),
            };
        }
        Ok(ObjectPatternProperty::Property {
            key,
            value,
            shorthand: true,
        })
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

    fn parse_do_while(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.advance(); // `primm`
        self.advance(); // `fa`
        let body = self.parse_block_body()?;
        // `mentre che` stays on the line of the `}`: on the next line it would
        // read as a new while loop
        if !self.check(&TokenKind::Mentre) {
            let token = self.peek();
            return Err(ParseError::new(
                format!(
                    "Missing 'mentre che (…)' on the line that closes 'primm fa', found '{}'",
                    token.kind
                ),
                token.span,
            ));
        }
        self.advance();
        self.expect(&TokenKind::Che)?;
        self.expect(&TokenKind::LeftParen)?;
        let condition = self.parse_expression()?;
        self.expect(&TokenKind::RightParen)?;
        Ok(Statement::DoWhile {
            body,
            condition,
            span: self.span_from(start.start),
        })
    }

    fn parse_for(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Pe)?;
        let is_await = self.match_token(&TokenKind::Aspett);
        // "ogni" is now optional for backwards compatibility
        self.match_token(&TokenKind::Ogni);
        self.expect(&TokenKind::LeftParen)?;
        if let Some(binding) = self.try_parse_for_each_binding() {
            let of = !self.match_token(&TokenKind::Ncopp);
            if of {
                self.advance(); // `dint`
            }
            let iterable = self.parse_expression()?;
            self.expect(&TokenKind::RightParen)?;
            let body = self.parse_block_body()?;
            if is_await && !of {
                return Err(ParseError::new(
                    "'pe aspett' works only with 'dint', not with ''ncopp'",
                    self.span_from(start.start),
                ));
            }
            return Ok(Statement::ForEach {
                binding,
                of,
                is_await,
                iterable,
                body,
                span: self.span_from(start.start),
            });
        }
        if is_await {
            let token = self.peek();
            return Err(ParseError::new(
                format!("Missing 'dint' in 'pe aspett', found '{}'", token.kind),
                token.span,
            ));
        }
        let init = if self.check(&TokenKind::Tien) {
            Some(Box::new(self.parse_let_declaration()?))
        } else if !self.check(&TokenKind::Semicolon) {
            let expr = self.parse_sequence()?;
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
            Some(self.parse_sequence()?)
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

    /// The loop variable of a for...of / for...in, if the header is one: the
    /// binding must be followed by `dint` or `'ncopp`. Otherwise nothing is
    /// consumed and the header is parsed as a classic `pe`.
    fn try_parse_for_each_binding(&mut self) -> Option<ForBinding> {
        let saved = self.current;
        let binding = if self.match_token(&TokenKind::Tien) {
            self.parse_binding_target().ok().map(ForBinding::Let)
        } else if self.check(&TokenKind::Chist) && self.at_e(1) {
            self.current += 2;
            self.parse_binding_target().ok().map(ForBinding::Const)
        } else {
            self.expect_identifier().ok().map(ForBinding::Existing)
        };
        if binding.is_some()
            && (self.check(&TokenKind::Ncopp) || self.check_identifier_at(0, "dint"))
        {
            return binding;
        }
        self.current = saved;
        None
    }

    /// `vedimm (…) {`: `vedimm` is not reserved, so `vedimm(x)` stays a call.
    fn at_switch(&self) -> bool {
        if !self.check_identifier_at(0, "vedimm") || !self.check_at(1, &TokenKind::LeftParen) {
            return false;
        }
        let mut depth = 0;
        for (offset, token) in self.tokens[self.current + 1..].iter().enumerate() {
            match token.kind {
                TokenKind::LeftParen => depth += 1,
                TokenKind::RightParen => {
                    depth -= 1;
                    if depth == 0 {
                        return self.check_at(offset + 2, &TokenKind::LeftBrace);
                    }
                }
                TokenKind::Eof => return false,
                _ => {}
            }
        }
        false
    }

    fn parse_switch(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.advance(); // `vedimm`
        self.expect(&TokenKind::LeftParen)?;
        let discriminant = self.parse_expression()?;
        self.expect(&TokenKind::RightParen)?;
        self.expect(&TokenKind::LeftBrace)?;
        let mut cases = Vec::new();
        let mut has_default = false;
        loop {
            self.skip_newlines();
            if self.match_token(&TokenKind::RightBrace) {
                break;
            }
            let label = self.current_span();
            let test = if self.match_token(&TokenKind::Caso) {
                Some(self.parse_expression()?)
            } else if self.match_token(&TokenKind::Predefinit) {
                if has_default {
                    return Err(ParseError::new(
                        "A 'vedimm' can have only one 'predefinit'",
                        label,
                    ));
                }
                has_default = true;
                None
            } else {
                let token = self.peek();
                return Err(ParseError::new(
                    format!(
                        "Missing 'caso' or 'predefinit' in 'vedimm', found '{}'",
                        token.kind
                    ),
                    token.span,
                ));
            };
            self.expect(&TokenKind::Colon)?;
            let mut body = Vec::new();
            loop {
                self.skip_newlines();
                if self.check(&TokenKind::Caso)
                    || self.check(&TokenKind::Predefinit)
                    || self.check(&TokenKind::RightBrace)
                    || self.is_at_end()
                {
                    break;
                }
                body.push(self.parse_statement()?);
            }
            cases.push(SwitchCase { test, body });
        }
        Ok(Statement::Switch {
            discriminant,
            cases,
            span: self.span_from(start.start),
        })
    }

    fn parse_break(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Rompe)?;
        let label = self.parse_jump_label()?;
        Ok(Statement::Break {
            label,
            span: self.span_from(start.start),
        })
    }

    fn parse_continue(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Salta)?;
        let label = self.parse_jump_label()?;
        Ok(Statement::Continue {
            label,
            span: self.span_from(start.start),
        })
    }

    /// The optional label after `rompe` / `salta`, on the same line.
    fn parse_jump_label(&mut self) -> Result<Option<String>, ParseError> {
        let named = matches!(self.peek().kind, TokenKind::Identifier(_))
            || contextual_name(self.peek()).is_some();
        if named {
            Ok(Some(self.expect_identifier()?))
        } else {
            Ok(None)
        }
    }

    /// `fore: pe (…) { … }`
    fn parse_labeled(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        let label = self.expect_identifier()?;
        self.expect(&TokenKind::Colon)?;
        self.skip_newlines();
        let body = self.parse_statement()?;
        Ok(Statement::Labeled {
            label,
            body: Box::new(body),
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
        let mut catch_param = None;
        let mut catch_body = None;
        if !self.at_finally() {
            self.expect(&TokenKind::And)?;
            self.expect(&TokenKind::Si)?;
            self.expect(&TokenKind::Schiatta)?;
            if self.match_token(&TokenKind::LeftParen) {
                catch_param = Some(self.parse_binding_target()?);
                self.expect(&TokenKind::RightParen)?;
            }
            catch_body = Some(self.parse_block_body()?);
        }
        let finally_body = if self.at_finally() {
            self.skip_newlines();
            self.advance();
            Some(self.parse_block_body()?)
        } else {
            None
        };
        Ok(Statement::TryCatch {
            try_body,
            catch_param,
            catch_body,
            finally_body,
            span: self.span_from(start.start),
        })
    }

    /// `aropp {` (finally), on the same line or the next ones. `aropp` is
    /// not reserved: anywhere else it is an ordinary name.
    fn at_finally(&self) -> bool {
        let mut offset = 0;
        while self.check_at(offset, &TokenKind::Newline) {
            offset += 1;
        }
        self.check_identifier_at(offset, "aropp")
            && self.check_at(offset + 1, &TokenKind::LeftBrace)
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
        let (name, super_class, members) = self.parse_class_parts(true)?;
        Ok(Statement::ClassDecl {
            name: name.unwrap_or_default(),
            super_class,
            members,
            span: self.span_from(start.start),
        })
    }

    /// `na famiglie [Nomme] [figlio 'e Base] { … }`; the name is optional
    /// in a class expression.
    fn parse_class_parts(&mut self, name_required: bool) -> Result<ClassParts, ParseError> {
        self.expect(&TokenKind::Na)?;
        self.expect(&TokenKind::Famiglie)?;
        let name = if name_required
            || !(self.check(&TokenKind::LeftBrace) || self.check(&TokenKind::Figlio))
        {
            Some(self.expect_identifier()?)
        } else {
            None
        };

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
        let mut members = Vec::new();
        loop {
            self.skip_newlines();
            if self.check(&TokenKind::RightBrace) || self.is_at_end() {
                break;
            }
            members.push(self.parse_class_member()?);
        }
        self.expect(&TokenKind::RightBrace)?;
        Ok((name, super_class, members))
    }

    /// `damme`/`miette` before a member name (`damme x()`, `damme [k]()`);
    /// `damme()` alone is a method called damme.
    fn accessor_kind(&self) -> MethodKind {
        if !(self.check_at(2, &TokenKind::LeftParen) || self.check_at(1, &TokenKind::LeftBracket)) {
            MethodKind::Method
        } else if self.check_identifier_at(0, "damme") {
            MethodKind::Getter
        } else if self.check_identifier_at(0, "miette") {
            MethodKind::Setter
        } else {
            MethodKind::Method
        }
    }

    fn check_accessor_params(
        &self,
        kind: MethodKind,
        params: &[Pattern],
        start: Span,
    ) -> Result<(), ParseError> {
        let expected = match kind {
            MethodKind::Getter => 0,
            MethodKind::Setter => 1,
            MethodKind::Method => return Ok(()),
        };
        if params.len() == expected {
            return Ok(());
        }
        Err(ParseError::new(
            if kind == MethodKind::Getter {
                "A 'damme' getter takes no parameters"
            } else {
                "A 'miette' setter takes exactly one parameter"
            },
            self.span_from(start.start),
        ))
    }

    fn parse_class_member(&mut self) -> Result<ClassMember, ParseError> {
        let start = self.current_span();
        // `fisso { … }`: static initialization block
        if self.check(&TokenKind::Fisso) && self.check_next(&TokenKind::LeftBrace) {
            self.advance();
            let body = self.parse_function_body(false)?;
            return Ok(ClassMember::StaticBlock {
                body,
                span: self.span_from(start.start),
            });
        }
        // `fisso` and `mo vir` are modifiers only when a member follows them
        let is_static = !self.check_next(&TokenKind::LeftParen)
            && !self.check_next(&TokenKind::Equal)
            && self.match_token(&TokenKind::Fisso);

        let is_async = if self.check(&TokenKind::Mo) && self.check_next(&TokenKind::Vir) {
            self.current += 2;
            true
        } else {
            false
        };

        // "facc" inside classes is optional.
        let has_facc = self.match_token(&TokenKind::Facc);
        // `facc* nomme()` / `*nomme()`
        let is_generator = self.match_token(&TokenKind::Star);

        // `damme x()` / `miette x(v)`
        let mut kind = MethodKind::Method;
        if !is_async && !has_facc && !is_generator {
            kind = self.accessor_kind();
            if kind != MethodKind::Method {
                self.advance();
            }
        }

        let key = self.parse_member_key()?;

        if kind == MethodKind::Method
            && !is_async
            && !has_facc
            && !is_generator
            && !self.check(&TokenKind::LeftParen)
        {
            let value = if self.match_token(&TokenKind::Equal) {
                Some(self.parse_expression()?)
            } else {
                None
            };
            self.expect_statement_end()?;
            return Ok(ClassMember::Field {
                key,
                value,
                is_static,
                span: self.span_from(start.start),
            });
        }

        let params = self.parse_parameters()?;
        self.check_accessor_params(kind, &params, start)?;
        let body = self.parse_function_body(is_generator)?;

        Ok(ClassMember::Method(ClassMethod {
            key,
            kind,
            params,
            body,
            is_async,
            is_generator,
            is_static,
            span: self.span_from(start.start),
        }))
    }

    /// A member name: any word (keywords included, as in JS) or `#private`.
    fn expect_member_name(&mut self) -> Result<String, ParseError> {
        if let TokenKind::PrivateName(name) = &self.peek().kind {
            let name = name.clone();
            self.advance();
            return Ok(name);
        }
        self.expect_property_name()
    }

    /// A class member key: `#private` or anything an object key accepts
    /// (a word, a string, a number or `[expr]`).
    fn parse_member_key(&mut self) -> Result<PropertyKey, ParseError> {
        if matches!(self.peek().kind, TokenKind::PrivateName(_)) {
            return Ok(PropertyKey::Name(self.expect_member_name()?));
        }
        self.parse_property_key()
    }

    fn parse_import(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_span();
        self.expect(&TokenKind::Chiamm)?;
        let mut default = None;
        let mut namespace = None;
        let mut specifiers = Vec::new();
        // `chiamm "./stile.css"`: no bindings, no `da`
        if let TokenKind::String(source) = &self.peek().kind {
            let source = source.clone();
            self.advance();
            return Ok(Statement::Import {
                default,
                namespace,
                specifiers,
                source,
                span: self.span_from(start.start),
            });
        }
        if !self.check(&TokenKind::LeftBrace) && !self.check(&TokenKind::Star) {
            default = Some(self.expect_identifier()?);
            if !self.match_token(&TokenKind::Comma) {
                return self.finish_import(start, default, namespace, specifiers);
            }
        }
        if self.match_token(&TokenKind::Star) {
            self.expect_comme()?;
            namespace = Some(self.expect_identifier()?);
        } else {
            self.expect(&TokenKind::LeftBrace)?;
            self.skip_newlines();
            while !self.check(&TokenKind::RightBrace) {
                let imported = self.expect_module_name()?;
                let local = if self.check_comme() {
                    self.advance();
                    self.expect_identifier()?
                } else {
                    imported.clone()
                };
                specifiers.push(ImportSpecifier { imported, local });
                self.skip_newlines();
                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
                self.skip_newlines();
            }
            self.expect(&TokenKind::RightBrace)?;
        }
        self.finish_import(start, default, namespace, specifiers)
    }

    fn finish_import(
        &mut self,
        start: Span,
        default: Option<String>,
        namespace: Option<String>,
        specifiers: Vec<ImportSpecifier>,
    ) -> Result<Statement, ParseError> {
        self.expect(&TokenKind::Da)?;
        let source = self.expect_string()?;
        Ok(Statement::Import {
            default,
            namespace,
            specifiers,
            source,
            span: self.span_from(start.start),
        })
    }

    /// `comme` (or `as`) between a module name and its alias.
    fn check_comme(&self) -> bool {
        matches!(&self.peek().kind, TokenKind::Identifier(n) if n == "comme" || n == "as")
    }
    fn expect_comme(&mut self) -> Result<(), ParseError> {
        if self.check_comme() {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::new(
                format!("Expected 'comme', but found '{}'", self.peek().kind),
                self.peek().span,
            ))
        }
    }

    /// A name on the module side of an import or export: any word, with
    /// `predefinit` meaning the default export.
    fn expect_module_name(&mut self) -> Result<String, ParseError> {
        if self.match_token(&TokenKind::Predefinit) {
            return Ok("default".to_string());
        }
        self.expect_property_name()
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
        } else if self.match_token(&TokenKind::Star) {
            let exported = if self.check_comme() {
                self.advance();
                Some(self.expect_module_name()?)
            } else {
                None
            };
            self.expect(&TokenKind::Da)?;
            let source = self.expect_string()?;
            self.expect_statement_end()?;
            Ok(Statement::ExportAll {
                exported,
                source,
                span: self.span_from(start.start),
            })
        } else if self.match_token(&TokenKind::LeftBrace) {
            let mut specifiers = Vec::new();
            self.skip_newlines();
            while !self.check(&TokenKind::RightBrace) {
                let local = self.expect_module_name()?;
                let exported = if self.check_comme() {
                    self.advance();
                    self.expect_module_name()?
                } else {
                    local.clone()
                };
                specifiers.push(ExportSpecifier { local, exported });
                self.skip_newlines();
                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
                self.skip_newlines();
            }
            self.expect(&TokenKind::RightBrace)?;
            let source = if self.match_token(&TokenKind::Da) {
                Some(self.expect_string()?)
            } else {
                None
            };
            self.expect_statement_end()?;
            Ok(Statement::ExportNamed {
                specifiers,
                source,
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
        self.nested(Self::parse_block_body_inner)
    }

    fn parse_block_body_inner(&mut self) -> Result<Vec<Statement>, ParseError> {
        self.expect(&TokenKind::LeftBrace)?;
        let mut statements = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            while self.check(&TokenKind::Newline) {
                self.advance();
            }
            // At the end of the file the brace is missing, not an expression
            if self.check(&TokenKind::RightBrace) || self.is_at_end() {
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

    /// `i++, j--`: comma-separated expressions, only in `pe ( … )` headers.
    fn parse_sequence(&mut self) -> Result<Expression, ParseError> {
        let first = self.parse_expression()?;
        if !self.check(&TokenKind::Comma) {
            return Ok(first);
        }
        let start = first.span().start;
        let mut expressions = vec![first];
        while self.match_token(&TokenKind::Comma) {
            expressions.push(self.parse_expression()?);
        }
        Ok(Expression::Sequence {
            expressions,
            span: self.span_from(start),
        })
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expression, ParseError> {
        self.nested(Self::parse_assignment_inner)
    }

    fn parse_assignment_inner(&mut self) -> Result<Expression, ParseError> {
        if self.check(&TokenKind::Caccia) {
            return self.parse_yield();
        }
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
            TokenKind::AmpEqual => AssignOp::BitAnd,
            TokenKind::PipeEqual => AssignOp::BitOr,
            TokenKind::CaretEqual => AssignOp::BitXor,
            TokenKind::LessLessEqual => AssignOp::ShiftLeft,
            TokenKind::GreaterGreaterEqual => AssignOp::ShiftRight,
            TokenKind::GreaterGreaterGreaterEqual => AssignOp::UnsignedShiftRight,
            _ => return Ok(expr),
        };
        let op_span = self.advance().span;
        if operator == AssignOp::Assign {
            check_destructuring_target(&expr, op_span)?;
        } else {
            check_assignment_target(&expr, op_span)?;
        }
        let value = self.parse_assignment()?;
        let span = self.span_from(expr.span().start);
        Ok(Expression::Assignment {
            target: Box::new(expr),
            operator,
            value: Box::new(value),
            span,
        })
    }

    /// `caccia x` / `caccia* altri` / `caccia`, at assignment level as in JS.
    fn parse_yield(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_span();
        if !self.in_generator {
            return Err(ParseError::new(
                "'caccia' works only inside a 'facc*' generator",
                start,
            ));
        }
        self.advance();
        let delegate = self.match_token(&TokenKind::Star);
        let ends = matches!(
            self.peek().kind,
            TokenKind::Newline
                | TokenKind::Semicolon
                | TokenKind::RightParen
                | TokenKind::RightBracket
                | TokenKind::RightBrace
                | TokenKind::Comma
                | TokenKind::Colon
                | TokenKind::Eof
        );
        let argument = if ends && !delegate {
            None
        } else {
            Some(Box::new(self.parse_assignment()?))
        };
        Ok(Expression::Yield {
            argument,
            delegate,
            span: self.span_from(start.start),
        })
    }

    fn parse_ternary(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_nullish()?;
        if self.match_token(&TokenKind::Question) {
            let consequent = self.parse_expression()?;
            self.expect(&TokenKind::Colon)?;
            let alternate = self.nested(Self::parse_ternary)?;
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

    /// `a ?? b`: binds looser than `o` / `e`. JS forbids mixing them without
    /// parentheses, and codegen always parenthesizes, so mixing is allowed here.
    fn parse_nullish(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_or()?;
        while self.match_token(&TokenKind::QuestionQuestion) {
            let right = self.parse_or()?;
            let span = self.span_from(expr.span().start);
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOp::Nullish,
                right: Box::new(right),
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
        let mut expr = self.parse_bit_or()?;
        // Supports both "e" and "pure" (both mean AND)
        while self.match_token(&TokenKind::And) || self.match_token(&TokenKind::Pure) {
            let right = self.parse_bit_or()?;
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

    /// `|`, `^`, `&`: between the logical operators and equality, as in JS.
    fn parse_bit_or(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_bit_xor()?;
        while self.match_token(&TokenKind::Pipe) {
            let right = self.parse_bit_xor()?;
            expr = self.binary(expr, BinaryOp::BitOr, right);
        }
        Ok(expr)
    }

    fn parse_bit_xor(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_bit_and()?;
        while self.match_token(&TokenKind::Caret) {
            let right = self.parse_bit_and()?;
            expr = self.binary(expr, BinaryOp::BitXor, right);
        }
        Ok(expr)
    }

    fn parse_bit_and(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_equality()?;
        while self.match_token(&TokenKind::Amp) {
            let right = self.parse_equality()?;
            expr = self.binary(expr, BinaryOp::BitAnd, right);
        }
        Ok(expr)
    }

    fn binary(&self, left: Expression, operator: BinaryOp, right: Expression) -> Expression {
        let span = self.span_from(left.span().start);
        Expression::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
            span,
        }
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
        let mut expr = self.parse_shift()?;
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
            } else if self.match_token(&TokenKind::DintA) {
                BinaryOp::In
            } else {
                break;
            };
            let right = self.parse_shift()?;
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

    fn parse_shift(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_term()?;
        loop {
            let op = if self.match_token(&TokenKind::LessLess) {
                BinaryOp::ShiftLeft
            } else if self.match_token(&TokenKind::GreaterGreater) {
                BinaryOp::ShiftRight
            } else if self.match_token(&TokenKind::GreaterGreaterGreater) {
                BinaryOp::UnsignedShiftRight
            } else {
                break;
            };
            let right = self.parse_term()?;
            expr = self.binary(expr, op, right);
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
            let right = self.nested(Self::parse_power)?;
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
        self.nested(Self::parse_unary_inner)
    }

    fn parse_unary_inner(&mut self) -> Result<Expression, ParseError> {
        // Numeric negation and conversion: -x, +x; bitwise not: ~x;
        // `void x` (`void` alone is a name)
        let operator = match &self.peek().kind {
            TokenKind::Minus => Some(UnaryOp::Negate),
            TokenKind::Plus => Some(UnaryOp::Plus),
            TokenKind::Tilde => Some(UnaryOp::BitNot),
            TokenKind::Identifier(n) if n == "void" && self.next_starts_operand() => {
                Some(UnaryOp::Void)
            }
            _ => None,
        };
        if let Some(operator) = operator {
            let start = self.advance().span;
            let operand = self.parse_unary()?;
            return Ok(Expression::Unary {
                operator,
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
                self.expect_e()?;
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
            // `?.` is followed by a name, `[` or `(`
            let optional = self.match_token(&TokenKind::QuestionDot);
            if self.match_token(&TokenKind::LeftParen) {
                let arguments = self.parse_arguments()?;
                let span = self.span_from(expr.span().start);
                expr = Expression::Call {
                    callee: Box::new(expr),
                    arguments,
                    optional,
                    span,
                };
            } else if self.match_token(&TokenKind::LeftBracket) {
                let property = self.nested(Self::parse_expression)?;
                self.expect(&TokenKind::RightBracket)?;
                let span = self.span_from(expr.span().start);
                expr = Expression::Member {
                    object: Box::new(expr),
                    property: Box::new(property),
                    computed: true,
                    optional,
                    span,
                };
            } else if !optional && matches!(self.peek().kind, TokenKind::Template { .. }) {
                // A template right after an expression is a tag call: html`…`
                let quasi = self.parse_primary()?;
                let span = self.span_from(expr.span().start);
                expr = Expression::TaggedTemplate {
                    tag: Box::new(expr),
                    quasi: Box::new(quasi),
                    span,
                };
            } else if optional || self.match_token(&TokenKind::Dot) {
                let name = self.expect_member_name()?;
                let span = self.span_from(expr.span().start);
                expr = Expression::Member {
                    object: Box::new(expr),
                    property: Box::new(Expression::Identifier {
                        name,
                        span: self.previous().span,
                    }),
                    computed: false,
                    optional,
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

    fn parse_function_expression(&mut self, is_async: bool) -> Result<Expression, ParseError> {
        let start = self.current_span();
        if is_async {
            self.advance();
            self.advance();
        }
        self.expect(&TokenKind::Facc)?;
        let is_generator = self.match_token(&TokenKind::Star);
        let name = if self.check(&TokenKind::LeftParen) {
            None
        } else {
            Some(self.expect_identifier()?)
        };
        let params = self.parse_parameters()?;
        let body = self.parse_function_body(is_generator)?;
        Ok(Expression::Function {
            name,
            params,
            body,
            is_async,
            is_generator,
            span: self.span_from(start.start),
        })
    }

    fn parse_arrow(&mut self, start: Span, is_async: bool) -> Result<Expression, ParseError> {
        let params = if self.check(&TokenKind::LeftParen) {
            self.parse_parameters()?
        } else {
            vec![self.parse_binding_target()?]
        };
        self.expect(&TokenKind::Arrow)?;
        // Arrows are never generators, even inside one
        let outer = std::mem::replace(&mut self.in_generator, false);
        let body = if self.check(&TokenKind::LeftBrace) {
            self.parse_block_body().map(ArrowBody::Block)
        } else {
            self.parse_assignment()
                .map(|e| ArrowBody::Expression(Box::new(e)))
        };
        self.in_generator = outer;
        let body = body?;
        Ok(Expression::ArrowFunction {
            params,
            body,
            is_async,
            span: self.span_from(start.start),
        })
    }

    /// Call arguments after `(`; they may span lines and use `...spread`.
    /// The class after `nu bell`: a primary expression followed by `.name`
    /// and `[key]` accesses, but no call.
    fn parse_new_callee(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.match_token(&TokenKind::Dot) {
                let name = self.expect_member_name()?;
                let span = self.span_from(expr.span().start);
                expr = Expression::Member {
                    object: Box::new(expr),
                    property: Box::new(Expression::Identifier {
                        name,
                        span: self.previous().span,
                    }),
                    computed: false,
                    optional: false,
                    span,
                };
            } else if self.match_token(&TokenKind::LeftBracket) {
                let property = self.nested(Self::parse_expression)?;
                self.expect(&TokenKind::RightBracket)?;
                let span = self.span_from(expr.span().start);
                expr = Expression::Member {
                    object: Box::new(expr),
                    property: Box::new(property),
                    computed: true,
                    optional: false,
                    span,
                };
            } else {
                return Ok(expr);
            }
        }
    }

    fn parse_arguments(&mut self) -> Result<Vec<Expression>, ParseError> {
        self.nested(Self::parse_arguments_inner)
    }

    fn parse_arguments_inner(&mut self) -> Result<Vec<Expression>, ParseError> {
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

    /// One entry of an object literal: `key: value`, `nome`, `[k]: v`,
    /// `saluta() { }`, `mo vir carica() { }` or `...altro`.
    fn parse_object_property(&mut self) -> Result<Property, ParseError> {
        if self.match_token(&TokenKind::Ellipsis) {
            return Ok(Property::Spread(self.parse_assignment()?));
        }
        let is_async = self.check(&TokenKind::Mo) && self.check_next(&TokenKind::Vir);
        if is_async {
            self.advance();
            self.advance();
        }
        // `facc` before a method name is optional, as in classes
        if self.check(&TokenKind::Facc)
            && !self.check_next(&TokenKind::Colon)
            && !self.check_next(&TokenKind::LeftParen)
        {
            self.advance();
        }
        let is_generator = self.match_token(&TokenKind::Star);
        // `damme prezzo() { }` / `miette prezzo(v) { }`
        let start = self.current_span();
        let kind = if is_async || is_generator {
            MethodKind::Method
        } else {
            self.accessor_kind()
        };
        if kind != MethodKind::Method {
            self.advance();
        }
        let key_token = self.peek().clone();
        let key = self.parse_property_key()?;
        if self.check(&TokenKind::LeftParen) {
            let params = self.parse_parameters()?;
            self.check_accessor_params(kind, &params, start)?;
            let body = self.parse_function_body(is_generator)?;
            return Ok(Property::Method {
                key,
                kind,
                params,
                body,
                is_async,
                is_generator,
            });
        }
        if is_async || is_generator {
            return Err(ParseError::new(
                format!("Expected '(', but found '{}'", self.peek().kind),
                self.peek().span,
            ));
        }
        if self.match_token(&TokenKind::Colon) {
            let value = self.parse_expression()?;
            return Ok(Property::KeyValue { key, value });
        }
        // Shorthand `{ nome }` needs a name that can be a variable
        let is_name = matches!(key_token.kind, TokenKind::Identifier(_))
            || contextual_name(&key_token).is_some();
        match key {
            PropertyKey::Name(name) if is_name => Ok(Property::Shorthand {
                name,
                span: key_token.span,
            }),
            _ => Err(ParseError::new(
                format!("Expected ':', but found '{}'", self.peek().kind),
                self.peek().span,
            )),
        }
    }

    fn parse_property_key(&mut self) -> Result<PropertyKey, ParseError> {
        if self.match_token(&TokenKind::LeftBracket) {
            let key = self.parse_assignment()?;
            self.expect(&TokenKind::RightBracket)?;
            return Ok(PropertyKey::Computed(Box::new(key)));
        }
        match &self.peek().kind {
            TokenKind::String(s) => {
                let s = s.clone();
                self.advance();
                Ok(PropertyKey::String(s))
            }
            TokenKind::Number(n) => {
                let n = *n;
                self.advance();
                Ok(PropertyKey::Number(n))
            }
            _ => Ok(PropertyKey::Name(self.expect_property_name()?)),
        }
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
        self.nested(Self::parse_primary_inner)
    }

    fn parse_primary_inner(&mut self) -> Result<Expression, ParseError> {
        // Class expressions: `na famiglie { }`, `na famiglie A figlio 'e B { }`
        if self.check(&TokenKind::Na) && self.check_next(&TokenKind::Famiglie) {
            let start = self.current_span();
            let (name, super_class, members) = self.parse_class_parts(false)?;
            return Ok(Expression::Class {
                name,
                super_class,
                members,
                span: self.span_from(start.start),
            });
        }
        // Function expressions: `facc (a) { }`, `mo vir facc nomme() { }`
        let is_async = self.check(&TokenKind::Mo)
            && self.check_next(&TokenKind::Vir)
            && self.check_at(2, &TokenKind::Facc);
        if is_async || self.check(&TokenKind::Facc) {
            return self.parse_function_expression(is_async);
        }
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

        // `#saldo dint'a o`: the only place a private name stands alone
        if let TokenKind::PrivateName(name) = &self.peek().kind {
            if self.check_next(&TokenKind::DintA) {
                let name = name.clone();
                let span = self.advance().span;
                return Ok(Expression::Identifier { name, span });
            }
        }

        // `import.meta` and `new.target` are not member accesses on a name
        if let Some(meta) = self.meta_property_start() {
            let start = self.advance().span;
            self.advance();
            let property = self.expect_identifier()?;
            return Ok(Expression::MetaProperty {
                meta: meta.to_string(),
                property,
                span: self.span_from(start.start),
            });
        }

        let token = self.advance();
        let span = token.span;

        match &token.kind {
            TokenKind::Number(n) => Ok(Expression::Number { value: *n, span }),
            TokenKind::BigInt(digits) => Ok(Expression::BigInt {
                digits: digits.clone(),
                span,
            }),
            TokenKind::Regex { pattern, flags } => Ok(Expression::Regex {
                pattern: pattern.clone(),
                flags: flags.clone(),
                span,
            }),
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
                    .map(|tokens| parse_interpolation(tokens, span, self.depth))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Expression::Template {
                    quasis,
                    expressions,
                    span,
                })
            }
            TokenKind::Jsx(element) => {
                let element = (**element).clone();
                let element = jsx::element(&element, span, self.depth)?;
                Ok(Expression::Jsx {
                    element: Box::new(element),
                    span,
                })
            }
            TokenKind::Overo => Ok(Expression::Boolean { value: true, span }),
            TokenKind::Sfols => Ok(Expression::Boolean { value: false, span }),
            TokenKind::Nisciun => Ok(Expression::Null { span }),
            TokenKind::Boh => Ok(Expression::Undefined { span }),
            // `chiamm("./m")`: dynamic import
            TokenKind::Chiamm => {
                self.expect(&TokenKind::LeftParen)?;
                self.skip_newlines();
                let source = self.parse_expression()?;
                self.skip_newlines();
                self.expect(&TokenKind::RightParen)?;
                Ok(Expression::Import {
                    source: Box::new(source),
                    span: self.span_from(span.start),
                })
            }
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
                // As in JS, the arguments are the first `( )` after the class
                // and anything after them applies to the new object:
                // `nu bell Pizza("m").descrivi()` calls `descrivi` on it.
                let callee = self.parse_new_callee()?;
                let arguments = if self.match_token(&TokenKind::LeftParen) {
                    self.parse_arguments()?
                } else {
                    vec![]
                };
                Ok(Expression::New {
                    callee: Box::new(callee),
                    arguments,
                    span: self.span_from(span.start),
                })
            }
            TokenKind::Stamm => {
                self.expect_identifier_a()?;
                self.expect_di()?;
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
                self.expect_di()?;
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
                self.expect_di()?;
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
                        properties.push(self.parse_object_property()?);
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
                format!("Expected an expression, but found '{}'", token.kind),
                span,
            )),
        }
    }

    // === Helpers ===

    /// Runs `f` one level deeper, or fails once `MAX_DEPTH` is reached.
    fn nested<T>(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<T, ParseError>,
    ) -> Result<T, ParseError> {
        if self.depth >= MAX_DEPTH {
            return Err(ParseError::new(
                "The code is nested too deeply",
                self.peek().span,
            ));
        }
        self.depth += 1;
        let result = f(self);
        self.depth -= 1;
        result
    }

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
    /// Consumes the current token and returns it. At the end of the input
    /// nothing is consumed and the `Eof` token is returned, so callers never
    /// see the token before it a second time.
    fn advance(&mut self) -> &Token {
        if self.is_at_end() {
            return self.peek();
        }
        self.current += 1;
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
    fn check_identifier_at(&self, offset: usize, name: &str) -> bool {
        matches!(self.tokens.get(self.current + offset).map(|t| &t.kind), Some(TokenKind::Identifier(n)) if n == name)
    }
    /// `import` in `import.meta` or `new` in `new.target`.
    fn meta_property_start(&self) -> Option<&'static str> {
        let (meta, property) = match &self.peek().kind {
            TokenKind::Identifier(n) if n == "import" => ("import", "meta"),
            TokenKind::Identifier(n) if n == "new" => ("new", "target"),
            _ => return None,
        };
        (self.check_next(&TokenKind::Dot) && self.check_identifier_at(2, property)).then_some(meta)
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
        } else if self.missing_comma_before(kind) {
            Err(ParseError::new(
                format!(
                    "Expected ',' or '{}' between the elements, but found '{}'",
                    kind,
                    self.peek().kind
                ),
                self.peek().span,
            ))
        } else {
            Err(ParseError::new(
                format!("Expected '{}', but found '{}'", kind, self.peek().kind),
                self.peek().span,
            ))
        }
    }

    /// `f(1 2)`, `[a b]`, `{ a: 1 b: 2 }`: a value right after another value,
    /// where a list wanted its closing bracket. The real mistake is the missing
    /// comma, not an unclosed bracket.
    fn missing_comma_before(&self, closing: &TokenKind) -> bool {
        let closes_list = matches!(
            closing,
            TokenKind::RightParen | TokenKind::RightBracket | TokenKind::RightBrace
        );
        let ends_value = |kind: &TokenKind| {
            matches!(
                kind,
                TokenKind::Identifier(_)
                    | TokenKind::String(_)
                    | TokenKind::Number(_)
                    | TokenKind::BigInt(_)
                    | TokenKind::Template { .. }
                    | TokenKind::RightParen
                    | TokenKind::RightBracket
                    | TokenKind::RightBrace
            )
        };
        let starts_value = |kind: &TokenKind| {
            matches!(
                kind,
                TokenKind::Identifier(_)
                    | TokenKind::String(_)
                    | TokenKind::Number(_)
                    | TokenKind::BigInt(_)
                    | TokenKind::Template { .. }
            )
        };
        closes_list
            && self.current > 0
            && ends_value(&self.previous().kind)
            && starts_value(&self.peek().kind)
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
    /// `è`, or `e` written without the accent (`chist e x = 1`).
    fn at_e(&self, offset: usize) -> bool {
        self.tokens.get(self.current + offset).is_some_and(|t| {
            t.kind == TokenKind::E || (t.kind == TokenKind::And && t.literal == "e")
        })
    }
    fn expect_e(&mut self) -> Result<(), ParseError> {
        if self.at_e(0) {
            self.advance();
            return Ok(());
        }
        self.expect(&TokenKind::E).map(|_| ())
    }
    /// `dì`, or `di` without the accent; `di` stays an ordinary name elsewhere.
    fn expect_di(&mut self) -> Result<(), ParseError> {
        if self.check_identifier_at(0, "di") {
            self.advance();
            return Ok(());
        }
        self.expect(&TokenKind::Di).map(|_| ())
    }
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
    /// The span from the token that starts at byte `start` to the previous
    /// token. Line and column are those of the first token, so that errors
    /// and source maps point at where the construct begins.
    fn span_from(&self, start: usize) -> Span {
        let first = self.tokens[..self.current]
            .iter()
            .rev()
            .find(|t| t.span.start == start)
            .map(|t| t.span)
            .unwrap_or(self.previous().span);
        Span::new(start, self.previous().span.end, first.line, first.column)
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
                | TokenKind::Regex { .. }
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
                | TokenKind::Plus
                | TokenKind::Tilde
                | TokenKind::BigInt(_)
                | TokenKind::PrivateName(_)
                | TokenKind::Aspett
                | TokenKind::CheE
        )
}

/// `=` also takes array and object patterns, as in JS: `[a, b] = [b, a]`,
/// `({ x, y: o.y } = punto)`, `[a = 1, ...resto] = lista`.
fn check_destructuring_target(target: &Expression, op_span: Span) -> Result<(), ParseError> {
    match target {
        Expression::Array { elements, .. } => {
            for (i, element) in elements.iter().enumerate() {
                match element {
                    Expression::Spread { argument, .. } if i + 1 == elements.len() => {
                        check_destructuring_target(argument, op_span)?
                    }
                    // `[a = 1] = …`: the default was already parsed as an assignment
                    Expression::Assignment {
                        operator: AssignOp::Assign,
                        ..
                    } => {}
                    _ => check_destructuring_target(element, op_span)?,
                }
            }
            Ok(())
        }
        Expression::Object { properties, .. } => {
            for (i, property) in properties.iter().enumerate() {
                match property {
                    Property::Shorthand { .. } => {}
                    Property::KeyValue { value, .. } => match value {
                        Expression::Assignment {
                            operator: AssignOp::Assign,
                            ..
                        } => {}
                        _ => check_destructuring_target(value, op_span)?,
                    },
                    Property::Spread(argument) if i + 1 == properties.len() => {
                        check_assignment_target(argument, op_span)?
                    }
                    _ => return Err(ParseError::new("Invalid assignment target", op_span)),
                }
            }
            Ok(())
        }
        _ => check_assignment_target(target, op_span),
    }
}

/// Only names and properties can be assigned or updated (`x = 1`, `a.b += 1`, `i++`).
fn check_assignment_target(target: &Expression, op_span: Span) -> Result<(), ParseError> {
    match target {
        Expression::Member { .. } if in_optional_chain(target) => {
            Err(ParseError::new("Invalid assignment target", op_span))
        }
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
    depth: usize,
) -> Result<Expression, ParseError> {
    let end = tokens.last().map(|t| t.span).unwrap_or(template_span);
    tokens.retain(|t| t.kind != TokenKind::Newline);
    if tokens.is_empty() {
        return Err(ParseError::new("Expected an expression inside '${}'", end));
    }
    tokens.push(Token::new(TokenKind::Eof, end, String::new()));
    let mut parser = Parser::new(tokens);
    parser.depth = depth;
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

/// `a?.b.c = 1` is not valid JS: no link of an assigned chain may be optional.
fn in_optional_chain(expr: &Expression) -> bool {
    match expr {
        Expression::Member {
            object, optional, ..
        } => *optional || in_optional_chain(object),
        Expression::Call {
            callee, optional, ..
        } => *optional || in_optional_chain(callee),
        _ => false,
    }
}
