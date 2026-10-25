//! Semantic checks over the AST, run after parsing.
//!
//! Today this reports names declared twice in the same scope, `piglie`,
//! `aspett`, `rompe`, `salta` and `'o pate` outside the construct they need,
//! `#private` names a class does not declare, `leva` on a plain name and
//! exports of names that do not exist: all things JS would only reject when
//! the generated code is loaded. It also rejects imports of helpers the
//! `fratm` library does not have.

use std::collections::{HashMap, HashSet};

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
        context: Context {
            allows_await: true,
            ..Context::default()
        },
        labels: Vec::new(),
        private_names: Vec::new(),
        exports: Vec::new(),
    };
    checker.statements(&program.statements);
    // Exports may name declarations further down, so check them last
    for (name, span) in std::mem::take(&mut checker.exports) {
        if !checker.scopes[0].names.contains_key(&name) {
            checker.fail(SemanticError {
                message: format!("'{}' is exported but not declared", name),
                span,
            });
        }
    }
    match checker.scopes.pop().and_then(|s| s.error) {
        Some(err) => Err(err),
        None => Ok(()),
    }
}

struct Checker {
    scopes: Vec<Scope>,
    context: Context,
    /// Labels around the statement being checked, and whether each one names
    /// a loop (only those work with `salta`). Each function starts with none.
    labels: Vec<(String, bool)>,
    /// The `#private` names of each class around the current position
    private_names: Vec<HashSet<String>>,
    /// Top-level `mann for { … }` names, checked once everything is declared
    exports: Vec<(String, Span)>,
}

/// Where the statement being checked sits, for `piglie`, `aspett`, `rompe`,
/// `salta` and `'o pate`. Each function starts a fresh context; arrows keep
/// the `'o pate` of the method around them.
#[derive(Clone, Copy, Default)]
struct Context {
    in_function: bool,
    /// `mo vir` function, or the top level (modules allow `await` there)
    allows_await: bool,
    loops: usize,
    switches: usize,
    /// Inside a method, where `'o pate.metodo()` works
    super_property: bool,
    /// Inside the constructor of a class with a parent: `'o pate(…)`
    super_call: bool,
}

impl Context {
    fn function(is_async: bool) -> Self {
        Context {
            in_function: true,
            allows_await: is_async,
            ..Context::default()
        }
    }

    fn method(is_async: bool, super_call: bool) -> Self {
        Context {
            super_property: true,
            super_call,
            ..Context::function(is_async)
        }
    }
}

#[derive(Default)]
struct Scope {
    names: HashMap<String, (Binding, usize)>,
    error: Option<SemanticError>,
}

impl Checker {
    /// Records an error unless the current scope already has one.
    fn fail(&mut self, error: SemanticError) {
        let scope = self.scopes.last_mut().expect("a scope is always open");
        if scope.error.is_none() {
            scope.error = Some(error);
        }
    }

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

    fn function(&mut self, params: &[Pattern], body: &[Statement], is_async: bool) {
        self.function_with(Context::function(is_async), params, body);
    }

    fn function_with(&mut self, context: Context, params: &[Pattern], body: &[Statement]) {
        self.with_context(context, |c| {
            c.scoped(|c| {
                c.params(params);
                c.statements(body);
            })
        });
    }

    /// The context of an arrow body: a new function that keeps `'o pate`.
    fn arrow_context(&self, is_async: bool) -> Context {
        Context {
            super_property: self.context.super_property,
            super_call: self.context.super_call,
            ..Context::function(is_async)
        }
    }

    fn with_context(&mut self, context: Context, f: impl FnOnce(&mut Self)) {
        let outer = std::mem::replace(&mut self.context, context);
        let outer_labels = std::mem::take(&mut self.labels);
        f(self);
        self.context = outer;
        self.labels = outer_labels;
    }

    /// Checks a loop body, where `rompe` and `salta` are allowed.
    fn loop_body(&mut self, body: &[Statement]) {
        self.context.loops += 1;
        self.block(body);
        self.context.loops -= 1;
    }

    /// `rompe x` / `salta x` without a label `x:` around them (on a loop, for `salta`).
    fn unknown_label(&mut self, keyword: &str, label: &str, span: Span) {
        self.fail(SemanticError {
            message: format!("'{} {}': no enclosing label '{}'", keyword, label, label),
            span,
        });
    }

    fn misplaced(&mut self, keyword: &str, span: Span) {
        self.fail(SemanticError {
            message: format!("'{}' is not allowed here", keyword),
            span,
        });
    }

    fn class_members(&mut self, members: &[ClassMember], has_parent: bool) {
        let names = self.declare_private_names(members);
        self.private_names.push(names);
        // Field values and static blocks are not functions, but `'o pate.x`
        // works in them
        let member_context = Context {
            super_property: true,
            ..Context::default()
        };
        for member in members {
            match member {
                ClassMember::Method(m) => {
                    self.property_key(&m.key);
                    let constructor = !m.is_static
                        && matches!(&m.key, PropertyKey::Name(n) if n == "costruttore");
                    let context = Context::method(m.is_async, constructor && has_parent);
                    self.function_with(context, &m.params, &m.body);
                }
                ClassMember::Field { key, value, .. } => {
                    self.property_key(key);
                    if let Some(value) = value {
                        self.with_context(member_context, |c| c.expression(value));
                    }
                }
                // Not a function: `piglie` and `aspett` are not allowed in it
                ClassMember::StaticBlock { body, .. } => {
                    self.with_context(member_context, |c| c.block(body))
                }
            }
        }
        self.private_names.pop();
    }

    /// The `#private` names of a class body. Each is declared once, except
    /// for a getter and a setter of the same name.
    fn declare_private_names(&mut self, members: &[ClassMember]) -> HashSet<String> {
        let mut names = HashSet::new();
        let mut accessors: HashMap<&str, MethodKind> = HashMap::new();
        for member in members {
            let (key, kind, span) = match member {
                ClassMember::Method(m) => (&m.key, Some(m.kind), m.span),
                ClassMember::Field { key, span, .. } => (key, None, *span),
                ClassMember::StaticBlock { .. } => continue,
            };
            let PropertyKey::Name(name) = key else {
                continue;
            };
            if !name.starts_with('#') {
                continue;
            }
            let pair = matches!(
                (kind, accessors.get(name.as_str())),
                (Some(MethodKind::Getter), Some(MethodKind::Setter))
                    | (Some(MethodKind::Setter), Some(MethodKind::Getter))
            );
            if !names.insert(name.clone()) && !pair {
                self.fail(SemanticError {
                    message: format!("'{}' is declared twice in the class", name),
                    span,
                });
            }
            if let Some(kind @ (MethodKind::Getter | MethodKind::Setter)) = kind {
                accessors.insert(name, kind);
            }
        }
        names
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
        // Modules import and export only at the top level
        let module_keyword = match statement {
            Statement::Import { .. } => Some("chiamm"),
            Statement::Export { .. }
            | Statement::ExportNamed { .. }
            | Statement::ExportAll { .. } => Some("mann for"),
            _ => None,
        };
        if let Some(keyword) = module_keyword {
            if self.scopes.len() > 1 {
                self.misplaced(keyword, statement_span(statement));
            }
        }
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
                is_async,
                span,
                ..
            } => {
                self.declare(name, Binding::Function, *span);
                self.function(params, body, *is_async);
            }
            Statement::ClassDecl {
                name,
                super_class,
                members,
                span,
            } => {
                self.declare(name, Binding::Lexical, *span);
                self.class_members(members, super_class.is_some());
            }
            Statement::Return { value, span } => {
                if !self.context.in_function {
                    self.misplaced("piglie", *span);
                }
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
                self.loop_body(body);
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
                c.loop_body(body);
            }),
            Statement::ForEach {
                binding,
                iterable,
                body,
                is_await,
                span,
                ..
            } => self.scoped(|c| {
                if *is_await && !c.context.allows_await {
                    c.misplaced("pe aspett", *span);
                }
                match binding {
                    ForBinding::Let(p) | ForBinding::Const(p) => {
                        c.pattern(p, Binding::Lexical);
                        c.pattern_defaults(p);
                    }
                    ForBinding::Existing(_) => {}
                }
                c.expression(iterable);
                c.loop_body(body);
            }),
            Statement::Switch {
                discriminant,
                cases,
                ..
            } => {
                self.expression(discriminant);
                // All the cases share one scope, as in JS
                self.context.switches += 1;
                self.scoped(|c| {
                    for case in cases {
                        if let Some(test) = &case.test {
                            c.expression(test);
                        }
                        c.statements(&case.body);
                    }
                });
                self.context.switches -= 1;
            }
            Statement::Break { label: None, span } => {
                if self.context.loops == 0 && self.context.switches == 0 {
                    self.misplaced("rompe", *span);
                }
            }
            Statement::Continue { label: None, span } => {
                if self.context.loops == 0 {
                    self.misplaced("salta", *span);
                }
            }
            Statement::Break {
                label: Some(label),
                span,
            } => {
                if !self.labels.iter().any(|(name, _)| name == label) {
                    self.unknown_label("rompe", label, *span);
                }
            }
            Statement::Continue {
                label: Some(label),
                span,
            } => {
                if !self
                    .labels
                    .iter()
                    .any(|(name, on_loop)| name == label && *on_loop)
                {
                    self.unknown_label("salta", label, *span);
                }
            }
            Statement::Labeled { label, body, .. } => {
                let on_loop = matches!(
                    **body,
                    Statement::While { .. }
                        | Statement::DoWhile { .. }
                        | Statement::For { .. }
                        | Statement::ForEach { .. }
                );
                self.labels.push((label.clone(), on_loop));
                self.statement(body);
                self.labels.pop();
            }
            Statement::Debugger { .. } => {}
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
                default,
                namespace,
                specifiers,
                source,
                span,
            } => {
                let whole = default.iter().chain(namespace.iter());
                if source == crate::stdlib::MODULE && (default.is_some() || namespace.is_some()) {
                    self.fail(SemanticError {
                        message: format!(
                            "The fratm library is imported by name: chiamm {{ {} }} da \"fratm\"",
                            crate::stdlib::names()
                        ),
                        span: *span,
                    });
                }
                for name in whole {
                    self.declare(name, Binding::Lexical, *span);
                }
                for s in specifiers {
                    if source == crate::stdlib::MODULE
                        && crate::stdlib::helper(&s.imported).is_none()
                    {
                        self.fail(SemanticError {
                            message: format!(
                                "'{}' is not in the fratm library (available: {})",
                                s.imported,
                                crate::stdlib::names()
                            ),
                            span: *span,
                        });
                    }
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
            Statement::ExportNamed {
                specifiers,
                source: None,
                span,
            } => {
                for s in specifiers {
                    self.exports.push((s.local.clone(), *span));
                }
            }
            Statement::ExportNamed { .. } | Statement::ExportAll { .. } => {}
            Statement::Expression { expression, .. } => self.expression(expression),
            Statement::Block { statements, .. } => self.block(statements),
        }
    }

    fn jsx(&mut self, element: &JsxElement) {
        for attribute in &element.attributes {
            match attribute {
                JsxAttribute::Named { value, .. } => {
                    if let Some(value) = value {
                        self.expression(value);
                    }
                }
                JsxAttribute::Spread(e) => self.expression(e),
            }
        }
        for child in &element.children {
            match child {
                JsxChild::Text(_) => {}
                JsxChild::Expression(e) => self.expression(e),
                JsxChild::Element(nested) => self.jsx(nested),
            }
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
            Expression::TaggedTemplate { tag, quasi, .. } => {
                self.expression(tag);
                self.expression(quasi);
            }
            Expression::Jsx { element, .. } => self.jsx(element),
            // `#saldo` read through `stu cos.#saldo` or in `#saldo dint'a o`
            Expression::Identifier { name, span } if name.starts_with('#') => {
                if !self.private_names.iter().any(|names| names.contains(name)) {
                    self.fail(SemanticError {
                        message: format!("'{}' is not declared in the class", name),
                        span: *span,
                    });
                }
            }
            Expression::Super { span } => self.misplaced("'o pate", *span),
            Expression::Delete { operand, span }
                if matches!(**operand, Expression::Identifier { .. }) =>
            {
                self.misplaced("leva", *span)
            }
            Expression::Identifier { .. }
            | Expression::Number { .. }
            | Expression::BigInt { .. }
            | Expression::String { .. }
            | Expression::Boolean { .. }
            | Expression::Null { .. }
            | Expression::Undefined { .. }
            | Expression::This { .. }
            | Expression::MetaProperty { .. }
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
                            key,
                            params,
                            body,
                            is_async,
                            ..
                        } => {
                            self.property_key(key);
                            self.function_with(Context::method(*is_async, false), params, body);
                        }
                        Property::Spread(e) => self.expression(e),
                    }
                }
            }
            Expression::Await { argument, span } => {
                if !self.context.allows_await {
                    self.misplaced("aspett", *span);
                }
                self.expression(argument);
            }
            Expression::Spread { argument, .. }
            | Expression::Update { argument, .. }
            | Expression::Unary {
                operand: argument, ..
            }
            | Expression::TypeOf {
                operand: argument, ..
            }
            | Expression::Delete {
                operand: argument, ..
            }
            | Expression::Import {
                source: argument, ..
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
                match (&**callee, expression) {
                    (Expression::Super { span }, Expression::Call { .. }) => {
                        if !self.context.super_call {
                            self.misplaced("'o pate()", *span);
                        }
                    }
                    _ => self.expression(callee),
                }
                for a in arguments {
                    self.expression(a);
                }
            }
            Expression::Member {
                object, property, ..
            } => {
                match &**object {
                    Expression::Super { span } => {
                        if !self.context.super_property {
                            self.misplaced("'o pate", *span);
                        }
                    }
                    _ => self.expression(object),
                }
                self.expression(property);
            }
            // The name of a class expression is visible only inside it
            Expression::Class {
                name,
                super_class,
                members,
                span,
            } => self.scoped(|c| {
                if let Some(name) = name {
                    c.declare(name, Binding::Lexical, *span);
                }
                c.class_members(members, super_class.is_some());
            }),
            // The name of a function expression is visible only inside it
            Expression::Function {
                name,
                params,
                body,
                is_async,
                span,
                ..
            } => self.scoped(|c| {
                if let Some(name) = name {
                    c.declare(name, Binding::Function, *span);
                }
                c.function(params, body, *is_async);
            }),
            Expression::ArrowFunction {
                params,
                body,
                is_async,
                ..
            } => match body {
                ArrowBody::Block(body) => {
                    self.function_with(self.arrow_context(*is_async), params, body)
                }
                ArrowBody::Expression(e) => self.with_context(self.arrow_context(*is_async), |c| {
                    c.scoped(|c| {
                        c.params(params);
                        c.expression(e);
                    })
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

fn statement_span(statement: &Statement) -> Span {
    match statement {
        Statement::Import { span, .. }
        | Statement::Export { span, .. }
        | Statement::ExportNamed { span, .. }
        | Statement::ExportAll { span, .. } => *span,
        _ => Span::default(),
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

    /// The `(line, column, message)` of a misplaced keyword, if any.
    fn misplaced(source: &str) -> Option<(usize, usize, String)> {
        match compile(source, Default::default()) {
            Err(e @ crate::errors::CompileError::SemanticError { .. }) => {
                Some((e.line()?, e.column()?, e.to_string()))
            }
            Err(e) => panic!("unexpected error for {:?}: {}", source, e),
            Ok(_) => None,
        }
    }

    #[test]
    fn test_piglie_only_inside_functions() {
        let (line, column, message) = misplaced("stamm a dì(1)\npiglie 2").unwrap();
        assert_eq!((line, column), (2, 1));
        assert!(message.contains("'piglie' se po' usà sulo dint'a na funzione"));
        assert_eq!(misplaced("facc f() { piglie 1 }"), None);
        assert_eq!(misplaced("f = () => { piglie 1 }"), None);
        assert_eq!(misplaced("o = { m() { piglie 1 } }"), None);
    }

    #[test]
    fn test_aspett_only_in_async_functions_or_at_the_top() {
        assert_eq!(
            misplaced("facc f() { aspett g() }").map(|e| (e.0, e.1)),
            Some((1, 12))
        );
        assert!(misplaced("f = () => aspett g()").is_some());
        assert!(misplaced("mo vir facc f() { facc h() { aspett g() } }").is_some());
        assert!(misplaced("facc f() { pe aspett (chist è x dint y) { } }").is_some());
        assert_eq!(misplaced("x = aspett g()"), None);
        assert_eq!(misplaced("mo vir facc f() { aspett g() }"), None);
        assert_eq!(misplaced("f = mo vir () => aspett g()"), None);
        assert_eq!(
            misplaced("na famiglie A { mo vir m() { aspett g() } }"),
            None
        );
    }

    #[test]
    fn test_rompe_and_salta_only_in_loops() {
        assert_eq!(misplaced("rompe").map(|e| (e.0, e.1)), Some((1, 1)));
        assert!(misplaced("salta").is_some());
        // A function body is not inside the loop around it
        assert!(misplaced("mentre che (overo) { facc f() { rompe } }").is_some());
        assert!(misplaced("vedimm (x) { caso 1: salta }").is_some());
        assert_eq!(misplaced("vedimm (x) { caso 1: rompe }"), None);
        assert_eq!(misplaced("mentre che (overo) { rompe }"), None);
        assert_eq!(
            misplaced("pe (tien i = 0; i < 2; i++) { si (i) { salta } }"),
            None
        );
        assert_eq!(
            misplaced("pe (chist è x dint xs) { vedimm (x) { caso 1: salta } }"),
            None
        );
        assert_eq!(misplaced("primm fa { rompe } mentre che (overo)"), None);
    }

    #[test]
    fn test_function_and_class_expressions_have_their_own_context() {
        assert_eq!(misplaced("f = facc () { piglie 1 }"), None);
        assert_eq!(misplaced("f = mo vir facc () { aspett g() }"), None);
        assert!(misplaced("f = facc () { aspett g() }").is_some());
        assert!(misplaced("mentre che (overo) { f = facc () { salta } }").is_some());
        assert_eq!(misplaced("A = na famiglie { m() { piglie 1 } }"), None);
        // A static block is not a function body
        assert!(misplaced("na famiglie A { fisso { piglie 1 } }").is_some());
    }

    /// The Neapolitan message of the semantic error for `source`, if any.
    fn semantic_error(source: &str) -> Option<String> {
        misplaced(source).map(|(_, _, message)| message)
    }

    #[test]
    fn test_private_names_must_be_declared_by_the_class() {
        let err = semantic_error("na famiglie A { m() { piglie stu cos.#x } }").unwrap();
        assert!(
            err.contains("'#x' nun sta dichiarato dint'a sta famiglie"),
            "{err}"
        );
        assert!(semantic_error("x = stu cos.#y").is_some());
        assert!(semantic_error("na famiglie A { #x = 1\n#x = 2 }").is_some());
        // Declared after use, in an arrow, in an outer class, as accessors
        assert_eq!(
            semantic_error("na famiglie A { m() { piglie () => stu cos.#x }\n#x = 1 }"),
            None
        );
        assert_eq!(
            semantic_error("na famiglie A { #x = 1\nm() { piglie na famiglie { n() { piglie stu cos.#x } } } }"),
            None
        );
        assert_eq!(
            semantic_error("na famiglie A { damme #x() { piglie 1 }\nmiette #x(v) { } }"),
            None
        );
        assert_eq!(
            semantic_error("na famiglie A { #x = 1\nha(o) { piglie #x dint'a o } }"),
            None
        );
    }

    #[test]
    fn test_o_pate_only_in_methods_and_derived_constructors() {
        assert!(semantic_error("facc f() { 'o pate.m() }").is_some());
        assert!(semantic_error("na famiglie A { facc costruttore() { 'o pate() } }").is_some());
        assert!(semantic_error("na famiglie A figlio 'e B { m() { 'o pate() } }").is_some());
        assert!(semantic_error("na famiglie A { m() { facc g() { 'o pate.m() } } }").is_some());
        assert_eq!(
            semantic_error("na famiglie A { m() { 'o pate.m() } }"),
            None
        );
        assert_eq!(
            semantic_error("na famiglie A figlio 'e B { facc costruttore() { 'o pate() } }"),
            None
        );
        assert_eq!(
            semantic_error(
                "na famiglie A figlio 'e B { facc costruttore() { f = () => 'o pate() } }"
            ),
            None
        );
        assert_eq!(semantic_error("x = { m() { piglie 'o pate.m() } }"), None);
        assert_eq!(
            semantic_error("na famiglie A figlio 'e B { x = 'o pate.y }"),
            None
        );
    }

    #[test]
    fn test_leva_needs_a_property() {
        assert!(semantic_error("tien x = 1\nleva x").is_some());
        assert_eq!(semantic_error("tien a = { b: 1 }\nleva a.b"), None);
        assert_eq!(semantic_error("tien a = { b: 1 }\nleva a[\"b\"]"), None);
    }

    #[test]
    fn test_exports_name_declared_things_at_the_top_level() {
        let err = semantic_error("mann for { a }").unwrap();
        assert!(err.contains("'a' nun esiste"), "{err}");
        assert!(semantic_error("si (x) { mann for { x } }").is_some());
        assert!(semantic_error("facc f() { chiamm { a } da \"m\" }").is_some());
        assert_eq!(semantic_error("tien a = 1\nmann for { a comme b }"), None);
        assert_eq!(semantic_error("mann for { b }\nfacc b() { }"), None);
        assert_eq!(semantic_error("mann for { a } da \"m\""), None);
    }
}
