//! Semantic checks over the AST, run after parsing.
//!
//! Today this reports names declared twice in the same scope, which JS
//! would only reject when the generated code is loaded.

use std::collections::HashMap;

use crate::lexer::Span;
use crate::parser::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticError {
    pub message: String,
    pub span: Span,
}

#[derive(Clone, Copy, PartialEq)]
enum Binding {
    /// `tien`, `chist è`, `na famiglie`, `chiamm`
    Lexical,
    /// `facc`: may be redeclared by another `facc`, as in JS
    Function,
    /// Function and catch parameters
    Param,
}

/// Returns the first redeclaration found, in source order.
pub fn check(program: &Program) -> Result<(), SemanticError> {
    let mut checker = Checker {
        scopes: vec![Scope::default()],
    };
    checker.statements(&program.statements);
    match checker.scopes.pop().and_then(|s| s.error) {
        Some(err) => Err(err),
        None => Ok(()),
    }
}

struct Checker {
    scopes: Vec<Scope>,
}

#[derive(Default)]
struct Scope {
    names: HashMap<String, (Binding, usize)>,
    error: Option<SemanticError>,
}

impl Checker {
    fn declare(&mut self, name: &str, binding: Binding, span: Span) {
        let scope = self.scopes.last_mut().expect("a scope is always open");
        if scope.error.is_some() {
            return;
        }
        if let Some(&(previous, line)) = scope.names.get(name) {
            let allowed = matches!(
                (previous, binding),
                (Binding::Function, Binding::Function) | (Binding::Param, Binding::Function)
            );
            if !allowed {
                scope.error = Some(SemanticError {
                    message: format!("'{}' is already declared (line {})", name, line),
                    span,
                });
            }
            return;
        }
        scope.names.insert(name.to_string(), (binding, span.line));
    }

    /// Runs `f` in a new scope and moves its error, if any, to the outer one.
    fn scoped(&mut self, f: impl FnOnce(&mut Self)) {
        self.scopes.push(Scope::default());
        f(self);
        let inner = self.scopes.pop().and_then(|s| s.error);
        let outer = self
            .scopes
            .last_mut()
            .expect("the program scope stays open");
        if outer.error.is_none() {
            outer.error = inner;
        }
    }

    fn block(&mut self, statements: &[Statement]) {
        self.scoped(|c| c.statements(statements));
    }

    fn statements(&mut self, statements: &[Statement]) {
        for s in statements {
            self.statement(s);
        }
    }

    fn function(&mut self, params: &[Pattern], body: &[Statement]) {
        self.scoped(|c| {
            c.params(params);
            c.statements(body);
        });
    }

    fn params(&mut self, params: &[Pattern]) {
        for p in params {
            self.pattern(p, Binding::Param);
        }
        // Defaults may hold arrows with their own bodies
        for p in params {
            self.pattern_defaults(p);
        }
    }

    /// Declares every name bound by `pattern`.
    fn pattern(&mut self, pattern: &Pattern, binding: Binding) {
        match pattern {
            Pattern::Identifier { name, span } => self.declare(name, binding, *span),
            Pattern::Array { elements, .. } => {
                for p in elements.iter().flatten() {
                    self.pattern(p, binding);
                }
            }
            Pattern::Object { properties, .. } => {
                for p in properties {
                    match p {
                        ObjectPatternProperty::Property { value, .. } => {
                            self.pattern(value, binding)
                        }
                        ObjectPatternProperty::Rest(p) => self.pattern(p, binding),
                    }
                }
            }
            Pattern::Default { target, .. } => self.pattern(target, binding),
            Pattern::Rest { argument } => self.pattern(argument, binding),
        }
    }

    fn pattern_defaults(&mut self, pattern: &Pattern) {
        match pattern {
            Pattern::Identifier { .. } => {}
            Pattern::Array { elements, .. } => {
                for p in elements.iter().flatten() {
                    self.pattern_defaults(p);
                }
            }
            Pattern::Object { properties, .. } => {
                for p in properties {
                    match p {
                        ObjectPatternProperty::Property { key, value, .. } => {
                            self.property_key(key);
                            self.pattern_defaults(value);
                        }
                        ObjectPatternProperty::Rest(p) => self.pattern_defaults(p),
                    }
                }
            }
            Pattern::Default { target, default } => {
                self.pattern_defaults(target);
                self.expression(default);
            }
            Pattern::Rest { argument } => self.pattern_defaults(argument),
        }
    }

    fn statement(&mut self, statement: &Statement) {
        match statement {
            Statement::VariableDecl { declarations, .. } => {
                for d in declarations {
                    self.pattern(&d.target, Binding::Lexical);
                    self.pattern_defaults(&d.target);
                    if let Some(value) = &d.value {
                        self.expression(value);
                    }
                }
            }
            Statement::FunctionDecl {
                name,
                params,
                body,
                span,
                ..
            } => {
                self.declare(name, Binding::Function, *span);
                self.function(params, body);
            }
            Statement::ClassDecl {
                name,
                members,
                span,
                ..
            } => {
                self.declare(name, Binding::Lexical, *span);
                for member in members {
                    match member {
                        ClassMember::Method(m) => {
                            self.property_key(&m.key);
                            self.function(&m.params, &m.body);
                        }
                        ClassMember::Field { key, value, .. } => {
                            self.property_key(key);
                            if let Some(value) = value {
                                self.expression(value);
                            }
                        }
                    }
                }
            }
            Statement::Return { value, .. } => {
                if let Some(value) = value {
                    self.expression(value);
                }
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                self.expression(condition);
                self.block(then_branch);
                if let Some(else_branch) = else_branch {
                    self.block(else_branch);
                }
            }
            Statement::While {
                condition, body, ..
            }
            | Statement::DoWhile {
                condition, body, ..
            } => {
                self.expression(condition);
                self.block(body);
            }
            Statement::For {
                init,
                condition,
                update,
                body,
                ..
            } => self.scoped(|c| {
                if let Some(init) = init {
                    c.statement(init);
                }
                if let Some(condition) = condition {
                    c.expression(condition);
                }
                if let Some(update) = update {
                    c.expression(update);
                }
                c.block(body);
            }),
            Statement::ForEach {
                binding,
                iterable,
                body,
                ..
            } => self.scoped(|c| {
                match binding {
                    ForBinding::Let(p) | ForBinding::Const(p) => {
                        c.pattern(p, Binding::Lexical);
                        c.pattern_defaults(p);
                    }
                    ForBinding::Existing(_) => {}
                }
                c.expression(iterable);
                c.block(body);
            }),
            Statement::Switch {
                discriminant,
                cases,
                ..
            } => {
                self.expression(discriminant);
                // All the cases share one scope, as in JS
                self.scoped(|c| {
                    for case in cases {
                        if let Some(test) = &case.test {
                            c.expression(test);
                        }
                        c.statements(&case.body);
                    }
                });
            }
            Statement::Break { .. } | Statement::Continue { .. } | Statement::Debugger { .. } => {}
            Statement::TryCatch {
                try_body,
                catch_param,
                catch_body,
                finally_body,
                ..
            } => {
                self.block(try_body);
                if let Some(catch_body) = catch_body {
                    self.scoped(|c| {
                        if let Some(param) = catch_param {
                            c.pattern(param, Binding::Param);
                            c.pattern_defaults(param);
                        }
                        c.statements(catch_body);
                    });
                }
                if let Some(finally_body) = finally_body {
                    self.block(finally_body);
                }
            }
            Statement::Throw { value, .. } => self.expression(value),
            Statement::Import {
                specifiers, span, ..
            } => {
                for s in specifiers {
                    self.declare(&s.local, Binding::Lexical, *span);
                }
            }
            Statement::Export {
                declaration,
                default_value,
                ..
            } => {
                if let Some(declaration) = declaration {
                    self.statement(declaration);
                }
                if let Some(value) = default_value {
                    self.expression(value);
                }
            }
            Statement::Expression { expression, .. } => self.expression(expression),
            Statement::Block { statements, .. } => self.block(statements),
        }
    }

    fn property_key(&mut self, key: &PropertyKey) {
        if let PropertyKey::Computed(e) = key {
            self.expression(e);
        }
    }

    /// Expressions declare nothing themselves, but arrows and object methods
    /// open function scopes.
    fn expression(&mut self, expression: &Expression) {
        match expression {
            Expression::Identifier { .. }
            | Expression::Number { .. }
            | Expression::String { .. }
            | Expression::Boolean { .. }
            | Expression::Null { .. }
            | Expression::Undefined { .. }
            | Expression::This { .. }
            | Expression::Super { .. }
            | Expression::Regex { .. } => {}
            Expression::Array { elements: list, .. }
            | Expression::Sequence {
                expressions: list, ..
            }
            | Expression::Template {
                expressions: list, ..
            }
            | Expression::ConsoleLog {
                arguments: list, ..
            }
            | Expression::ConsoleWarn {
                arguments: list, ..
            }
            | Expression::ConsoleError {
                arguments: list, ..
            } => {
                for e in list {
                    self.expression(e);
                }
            }
            Expression::Object { properties, .. } => {
                for p in properties {
                    match p {
                        Property::KeyValue { key, value } => {
                            self.property_key(key);
                            self.expression(value);
                        }
                        Property::Shorthand { .. } => {}
                        Property::Method {
                            key, params, body, ..
                        } => {
                            self.property_key(key);
                            self.function(params, body);
                        }
                        Property::Spread(e) => self.expression(e),
                    }
                }
            }
            Expression::Spread { argument, .. }
            | Expression::Await { argument, .. }
            | Expression::Update { argument, .. }
            | Expression::Unary {
                operand: argument, ..
            }
            | Expression::TypeOf {
                operand: argument, ..
            }
            | Expression::Delete {
                operand: argument, ..
            } => self.expression(argument),
            Expression::Yield { argument, .. } => {
                if let Some(argument) = argument {
                    self.expression(argument);
                }
            }
            Expression::Binary { left, right, .. } => {
                self.expression(left);
                self.expression(right);
            }
            Expression::Assignment { target, value, .. } => {
                self.expression(target);
                self.expression(value);
            }
            Expression::Call {
                callee, arguments, ..
            }
            | Expression::New {
                callee, arguments, ..
            } => {
                self.expression(callee);
                for a in arguments {
                    self.expression(a);
                }
            }
            Expression::Member {
                object, property, ..
            } => {
                self.expression(object);
                self.expression(property);
            }
            Expression::ArrowFunction { params, body, .. } => match body {
                ArrowBody::Block(body) => self.function(params, body),
                ArrowBody::Expression(e) => self.scoped(|c| {
                    c.params(params);
                    c.expression(e);
                }),
            },
            Expression::Ternary {
                condition,
                consequent,
                alternate,
                ..
            } => {
                self.expression(condition);
                self.expression(consequent);
                self.expression(alternate);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::compile;

    /// The `(line, column)` of the reported redeclaration, if any.
    fn redeclared(source: &str) -> Option<(usize, usize)> {
        match compile(source, Default::default()) {
            Err(crate::errors::CompileError::SemanticError { line, column, .. }) => {
                Some((line, column))
            }
            Err(e) => panic!("unexpected error for {:?}: {}", source, e),
            Ok(_) => None,
        }
    }

    #[test]
    fn test_reports_the_second_declaration() {
        assert_eq!(redeclared("tien x = 1\ntien x = 2"), Some((2, 6)));
        assert_eq!(redeclared("chist è x = 1\ntien x = 2"), Some((2, 6)));
        assert_eq!(redeclared("tien a = 1, a = 2"), Some((1, 13)));
        assert_eq!(redeclared("tien [a, { b }] = c\ntien b = 1"), Some((2, 6)));
        assert_eq!(redeclared("facc f() { }\ntien f = 1"), Some((2, 6)));
        assert_eq!(redeclared("tien P = 1\nna famiglie P { }"), Some((2, 1)));
        assert_eq!(
            redeclared("chiamm { a } da \"m\"\ntien a = 1"),
            Some((2, 6))
        );
    }

    #[test]
    fn test_scopes_follow_js() {
        // Inner blocks, loop bodies and functions open new scopes
        assert_eq!(redeclared("tien x = 1\nsi (x) { tien x = 2 }"), None);
        assert_eq!(
            redeclared("pe (tien i = 0; i < 1; i++) { tien i = 2 }"),
            None
        );
        assert_eq!(redeclared("pe (chist è v dint vs) { tien v = 1 }"), None);
        assert_eq!(redeclared("tien x\nchist è f = () => { tien x = 1 }"), None);
        assert_eq!(redeclared("tien x\nfacc f(x) { }"), None);
        // ...but a block is still one scope
        assert_eq!(
            redeclared("si (overo) {\n tien x = 1\n tien x = 2\n}"),
            Some((3, 7))
        );
        // Parameters clash with declarations in the body, and with each other
        assert_eq!(redeclared("facc f(a) {\n tien a = 1\n}"), Some((2, 7)));
        assert_eq!(redeclared("facc f(a, a) { }"), Some((1, 11)));
        assert_eq!(
            redeclared("chist è f = (a) => { tien a = 1 }"),
            Some((1, 27))
        );
        assert_eq!(
            redeclared("pruvamm { } e si schiatta (e) { tien e = 1 }"),
            Some((1, 38))
        );
        // Every case of a vedimm shares one scope
        assert_eq!(
            redeclared("vedimm (k) {\n caso 1: tien x = 1\n caso 2: tien x = 2\n}"),
            Some((3, 15))
        );
    }

    #[test]
    fn test_function_redeclarations_allowed_by_js() {
        assert_eq!(redeclared("facc f() { }\nfacc f() { }"), None);
        assert_eq!(redeclared("facc f(g) { facc g() { } }"), None);
    }

    #[test]
    fn test_message_is_neapolitan() {
        let err = compile("tien x = 1\ntien x = 2", Default::default()).unwrap_err();
        assert!(
            err.to_string()
                .contains("'x' già esiste: l'hai dichiarato a riga 1!"),
            "{}",
            err
        );
    }
}
