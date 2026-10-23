//! Code Generator for FratmScript

use crate::parser::*;
use crate::sourcemap::{SourceMap, SourceMapBuilder};

pub struct CodeGen {
    indent: usize,
    output: String,
    source_map_enabled: bool,
    source_map_builder: SourceMapBuilder,
    current_line: usize,
    current_col: usize,
}

impl CodeGen {
    pub fn new(source_map: bool) -> Self {
        Self {
            indent: 0,
            output: String::new(),
            source_map_enabled: source_map,
            source_map_builder: SourceMapBuilder::new(),
            current_line: 0,
            current_col: 0,
        }
    }

    pub fn generate(&mut self, program: &Program) -> String {
        for stmt in &program.statements {
            self.gen_statement(stmt);
            self.emit("\n");
        }
        self.output.clone()
    }

    pub fn get_source_map(&self) -> SourceMap {
        self.source_map_builder.clone().build(None)
    }

    fn emit(&mut self, s: &str) {
        for c in s.chars() {
            if c == '\n' {
                self.current_line += 1;
                self.current_col = 0;
                if self.source_map_enabled {
                    self.source_map_builder.new_line();
                }
            } else {
                self.current_col += 1;
            }
        }
        self.output.push_str(s);
    }

    fn add_mapping(&mut self, src_line: usize, src_col: usize) {
        if self.source_map_enabled {
            self.source_map_builder.add_mapping(
                self.current_line,
                self.current_col,
                src_line.saturating_sub(1),
                src_col.saturating_sub(1),
            );
        }
    }

    fn gen_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VariableDecl {
                target,
                value,
                is_const,
                span,
                ..
            } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                self.emit(if *is_const { "const " } else { "let " });
                self.gen_pattern(target);
                if let Some(val) = value {
                    self.emit(" = ");
                    self.gen_expression(val);
                }
                self.emit(";");
            }

            Statement::FunctionDecl {
                name,
                params,
                body,
                is_async,
                span,
                ..
            } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                if *is_async {
                    self.emit("async ");
                }
                self.emit("function ");
                self.emit(&js_name(name));
                self.gen_params(params);
                self.emit(" {\n");
                self.indent += 1;
                for s in body {
                    self.gen_statement(s);
                    self.emit("\n");
                }
                self.indent -= 1;
                self.write_indent();
                self.emit("}");
            }

            Statement::Return { value, span, .. } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                self.emit("return");
                if let Some(val) = value {
                    self.emit(" ");
                    self.gen_expression(val);
                }
                self.emit(";");
            }

            Statement::If {
                condition,
                then_branch,
                else_branch,
                span,
                ..
            } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                self.emit("if (");
                self.gen_expression(condition);
                self.emit(") {\n");
                self.indent += 1;
                for s in then_branch {
                    self.gen_statement(s);
                    self.emit("\n");
                }
                self.indent -= 1;
                self.write_indent();
                self.emit("}");
                if let Some(else_body) = else_branch {
                    self.emit(" else ");
                    if else_body.len() == 1 {
                        if let Statement::If { .. } = &else_body[0] {
                            self.gen_statement(&else_body[0]);
                            return;
                        }
                    }
                    self.emit("{\n");
                    self.indent += 1;
                    for s in else_body {
                        self.gen_statement(s);
                        self.emit("\n");
                    }
                    self.indent -= 1;
                    self.write_indent();
                    self.emit("}");
                }
            }

            Statement::While {
                condition,
                body,
                span,
                ..
            } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                self.emit("while (");
                self.gen_expression(condition);
                self.emit(") {\n");
                self.indent += 1;
                for s in body {
                    self.gen_statement(s);
                    self.emit("\n");
                }
                self.indent -= 1;
                self.write_indent();
                self.emit("}");
            }

            Statement::For {
                init,
                condition,
                update,
                body,
                span,
                ..
            } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                self.emit("for (");
                if let Some(i) = init {
                    match i.as_ref() {
                        Statement::VariableDecl {
                            target,
                            value,
                            is_const,
                            ..
                        } => {
                            self.emit(if *is_const { "const " } else { "let " });
                            self.gen_pattern(target);
                            if let Some(val) = value {
                                self.emit(" = ");
                                self.gen_expression(val);
                            }
                        }
                        Statement::Expression { expression, .. } => self.gen_expression(expression),
                        _ => {}
                    }
                }
                self.emit("; ");
                if let Some(c) = condition {
                    self.gen_expression(c);
                }
                self.emit("; ");
                if let Some(u) = update {
                    self.gen_expression(u);
                }
                self.emit(") {\n");
                self.indent += 1;
                for s in body {
                    self.gen_statement(s);
                    self.emit("\n");
                }
                self.indent -= 1;
                self.write_indent();
                self.emit("}");
            }

            Statement::Break { .. } => {
                self.write_indent();
                self.emit("break;");
            }
            Statement::Continue { .. } => {
                self.write_indent();
                self.emit("continue;");
            }
            Statement::Debugger { .. } => {
                self.write_indent();
                self.emit("debugger;");
            }

            Statement::TryCatch {
                try_body,
                catch_param,
                catch_body,
                span,
                ..
            } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                self.emit("try {\n");
                self.indent += 1;
                for s in try_body {
                    self.gen_statement(s);
                    self.emit("\n");
                }
                self.indent -= 1;
                self.write_indent();
                self.emit("} catch");
                if let Some(param) = catch_param {
                    self.emit(" (");
                    self.gen_pattern(param);
                    self.emit(")");
                }
                self.emit(" {\n");
                self.indent += 1;
                for s in catch_body {
                    self.gen_statement(s);
                    self.emit("\n");
                }
                self.indent -= 1;
                self.write_indent();
                self.emit("}");
            }

            Statement::Throw { value, .. } => {
                self.write_indent();
                self.emit("throw ");
                self.gen_expression(value);
                self.emit(";");
            }

            Statement::ClassDecl {
                name,
                super_class,
                methods,
                span,
                ..
            } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                self.emit("class ");
                self.emit(&js_name(name));
                if let Some(parent) = super_class {
                    self.emit(" extends ");
                    self.emit(&js_name(parent));
                }
                self.emit(" {\n");
                self.indent += 1;
                for method in methods {
                    self.write_indent();
                    if method.is_static {
                        self.emit("static ");
                    }
                    if method.is_async {
                        self.emit("async ");
                    }
                    // Translate "costruttore" to JavaScript "constructor"
                    let js_method_name = if method.name == "costruttore" {
                        "constructor"
                    } else {
                        method.name.as_str()
                    };
                    self.emit(js_method_name);
                    self.gen_params(&method.params);
                    self.emit(" {\n");
                    self.indent += 1;
                    for s in &method.body {
                        self.gen_statement(s);
                        self.emit("\n");
                    }
                    self.indent -= 1;
                    self.write_indent();
                    self.emit("}\n");
                }
                self.indent -= 1;
                self.write_indent();
                self.emit("}");
            }

            Statement::Import {
                specifiers, source, ..
            } => {
                self.write_indent();
                self.emit("import { ");
                let names: Vec<String> = specifiers.iter().map(|s| s.local.clone()).collect();
                self.emit(&js_names(&names));
                self.emit(" } from \"");
                self.emit(source);
                self.emit("\";");
            }

            Statement::Export {
                declaration,
                default_value,
                ..
            } => {
                self.write_indent();
                if let Some(val) = default_value {
                    self.emit("export default ");
                    self.gen_expression(val);
                    self.emit(";");
                } else if let Some(decl) = declaration {
                    self.emit("export ");
                    let saved = self.indent;
                    self.indent = 0;
                    self.gen_statement(decl);
                    self.indent = saved;
                }
            }

            Statement::Expression { expression, .. } => {
                self.write_indent();
                self.gen_expression(expression);
                self.emit(";");
            }

            Statement::Block { statements, .. } => {
                self.write_indent();
                self.emit("{\n");
                self.indent += 1;
                for s in statements {
                    self.gen_statement(s);
                    self.emit("\n");
                }
                self.indent -= 1;
                self.write_indent();
                self.emit("}");
            }
        }
    }

    fn gen_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Identifier { name, span, .. } => {
                self.add_mapping(span.line, span.column);
                self.emit(&js_name(name));
            }
            Expression::Number { value, .. } => {
                if *value == value.floor() && value.abs() < 1e15 {
                    self.emit(&(*value as i64).to_string());
                } else if value.abs() >= 1e21 || value.abs() < 1e-6 {
                    // Use exponent notation instead of hundreds of digits (`1e300`)
                    self.emit(&format!("{:e}", value));
                } else {
                    self.emit(&value.to_string());
                }
            }
            Expression::String { value, .. } => {
                self.emit("\"");
                for c in value.chars() {
                    match c {
                        '"' => self.emit("\\\""),
                        '\\' => self.emit("\\\\"),
                        '\n' => self.emit("\\n"),
                        '\r' => self.emit("\\r"),
                        '\t' => self.emit("\\t"),
                        _ => self.emit(&c.to_string()),
                    }
                }
                self.emit("\"");
            }
            Expression::Template {
                quasis,
                expressions,
                ..
            } => {
                self.emit("`");
                for (i, quasi) in quasis.iter().enumerate() {
                    self.emit(quasi);
                    if let Some(expr) = expressions.get(i) {
                        self.emit("${");
                        self.gen_expression(expr);
                        self.emit("}");
                    }
                }
                self.emit("`");
            }
            Expression::Boolean { value, .. } => self.emit(if *value { "true" } else { "false" }),
            Expression::Null { .. } => self.emit("null"),
            Expression::Undefined { .. } => self.emit("undefined"),
            Expression::This { .. } => self.emit("this"),
            Expression::Super { .. } => self.emit("super"),
            Expression::Array { elements, .. } => {
                self.emit("[");
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        self.emit(", ");
                    }
                    self.gen_expression(elem);
                }
                self.emit("]");
            }
            Expression::Object { properties, .. } if properties.is_empty() => self.emit("{}"),
            Expression::Object { properties, .. } => {
                self.emit("{ ");
                for (i, property) in properties.iter().enumerate() {
                    if i > 0 {
                        self.emit(", ");
                    }
                    match property {
                        Property::KeyValue { key, value } => {
                            self.gen_property_key(key);
                            self.emit(": ");
                            self.gen_expression(value);
                        }
                        Property::Shorthand { name, span } => {
                            let escaped = js_name(name);
                            if escaped != name.as_str() {
                                // `{ for }` must become `{ for: $for }`
                                self.emit(name);
                                self.emit(": ");
                            }
                            self.add_mapping(span.line, span.column);
                            self.emit(&escaped);
                        }
                        Property::Method {
                            key,
                            params,
                            body,
                            is_async,
                        } => {
                            if *is_async {
                                self.emit("async ");
                            }
                            self.gen_property_key(key);
                            self.gen_params(params);
                            self.emit(" {\n");
                            self.indent += 1;
                            for s in body {
                                self.gen_statement(s);
                                self.emit("\n");
                            }
                            self.indent -= 1;
                            self.write_indent();
                            self.emit("}");
                        }
                        Property::Spread(argument) => {
                            self.emit("...");
                            self.gen_expression(argument);
                        }
                    }
                }
                self.emit(" }");
            }
            Expression::Spread { argument, .. } => {
                self.emit("...");
                self.gen_expression(argument);
            }
            Expression::Binary {
                left,
                operator,
                right,
                ..
            } => {
                self.emit("(");
                self.gen_operand(left);
                self.emit(" ");
                self.emit(operator.to_js());
                self.emit(" ");
                self.gen_operand(right);
                self.emit(")");
            }
            Expression::Unary {
                operator, operand, ..
            } => {
                self.emit(operator.to_js());
                // `- -x` and `- --x` must not become `--x` / `---x`
                let starts_with_minus = matches!(
                    **operand,
                    Expression::Unary {
                        operator: UnaryOp::Negate,
                        ..
                    } | Expression::Update {
                        operator: UpdateOp::Decrement,
                        prefix: true,
                        ..
                    }
                );
                if matches!(operator, UnaryOp::Negate) && starts_with_minus {
                    self.emit("(");
                    self.gen_expression(operand);
                    self.emit(")");
                } else {
                    self.gen_operand(operand);
                }
            }
            Expression::Assignment {
                target,
                operator,
                value,
                ..
            } => {
                self.gen_expression(target);
                self.emit(" ");
                self.emit(operator.to_js());
                self.emit(" ");
                self.gen_expression(value);
            }
            Expression::Update {
                operator,
                prefix,
                argument,
                ..
            } => {
                if *prefix {
                    self.emit(operator.to_js());
                }
                self.gen_expression(argument);
                if !*prefix {
                    self.emit(operator.to_js());
                }
            }
            Expression::Call {
                callee, arguments, ..
            } => {
                self.gen_callee(callee);
                self.emit("(");
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        self.emit(", ");
                    }
                    self.gen_expression(arg);
                }
                self.emit(")");
            }
            Expression::Member {
                object,
                property,
                computed,
                ..
            } => {
                self.gen_callee(object);
                if *computed {
                    self.emit("[");
                    self.gen_expression(property);
                    self.emit("]");
                } else if let Expression::Identifier { name, span } = property.as_ref() {
                    // Property names are never escaped: `x.for` is valid JS
                    self.emit(".");
                    self.add_mapping(span.line, span.column);
                    self.emit(name);
                } else {
                    self.emit(".");
                    self.gen_expression(property);
                }
            }
            Expression::New {
                callee, arguments, ..
            } => {
                self.emit("new ");
                self.gen_callee(callee);
                self.emit("(");
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        self.emit(", ");
                    }
                    self.gen_expression(arg);
                }
                self.emit(")");
            }
            Expression::ArrowFunction {
                params,
                body,
                is_async,
                ..
            } => {
                if *is_async {
                    self.emit("async ");
                }
                self.gen_params(params);
                self.emit(" => ");
                match body {
                    // `() => ({ … })`: a bare `{` would start a block
                    ArrowBody::Expression(e) if matches!(**e, Expression::Object { .. }) => {
                        self.emit("(");
                        self.gen_expression(e);
                        self.emit(")");
                    }
                    ArrowBody::Expression(e) => self.gen_expression(e),
                    ArrowBody::Block(stmts) => {
                        self.emit("{\n");
                        self.indent += 1;
                        for s in stmts {
                            self.gen_statement(s);
                            self.emit("\n");
                        }
                        self.indent -= 1;
                        self.write_indent();
                        self.emit("}");
                    }
                }
            }
            Expression::Await { argument, .. } => {
                self.emit("await ");
                self.gen_operand(argument);
            }
            Expression::Ternary {
                condition,
                consequent,
                alternate,
                ..
            } => {
                self.emit("(");
                self.gen_operand(condition);
                self.emit(" ? ");
                self.gen_expression(consequent);
                self.emit(" : ");
                self.gen_expression(alternate);
                self.emit(")");
            }
            Expression::ConsoleLog { arguments, .. } => {
                self.emit("console.log(");
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        self.emit(", ");
                    }
                    self.gen_expression(arg);
                }
                self.emit(")");
            }
            Expression::ConsoleWarn { arguments, .. } => {
                self.emit("console.warn(");
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        self.emit(", ");
                    }
                    self.gen_expression(arg);
                }
                self.emit(")");
            }
            Expression::ConsoleError { arguments, .. } => {
                self.emit("console.error(");
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        self.emit(", ");
                    }
                    self.gen_expression(arg);
                }
                self.emit(")");
            }
            Expression::TypeOf { operand, .. } => {
                self.emit("typeof ");
                self.gen_operand(operand);
            }
            Expression::Delete { operand, .. } => {
                self.emit("delete ");
                self.gen_operand(operand);
            }
        }
    }

    fn gen_property_key(&mut self, key: &PropertyKey) {
        match key {
            PropertyKey::Name(name) => self.emit(name),
            PropertyKey::String(value) => self.gen_expression(&Expression::String {
                value: value.clone(),
                span: Default::default(),
            }),
            PropertyKey::Number(value) => self.gen_expression(&Expression::Number {
                value: *value,
                span: Default::default(),
            }),
            PropertyKey::Computed(expr) => {
                self.emit("[");
                self.gen_expression(expr);
                self.emit("]");
            }
        }
    }

    fn gen_params(&mut self, params: &[Pattern]) {
        self.emit("(");
        for (i, param) in params.iter().enumerate() {
            if i > 0 {
                self.emit(", ");
            }
            self.gen_pattern(param);
        }
        self.emit(")");
    }

    fn gen_pattern(&mut self, pattern: &Pattern) {
        match pattern {
            Pattern::Identifier { name, span } => {
                self.add_mapping(span.line, span.column);
                self.emit(&js_name(name));
            }
            Pattern::Default { target, default } => {
                self.gen_pattern(target);
                self.emit(" = ");
                self.gen_expression(default);
            }
            Pattern::Rest { argument } => {
                self.emit("...");
                self.gen_pattern(argument);
            }
            Pattern::Array { elements, .. } => {
                self.emit("[");
                for (i, element) in elements.iter().enumerate() {
                    if i > 0 {
                        self.emit(", ");
                    }
                    if let Some(element) = element {
                        self.gen_pattern(element);
                    }
                }
                // A trailing hole needs its own comma: `[a, ]` has one element
                if matches!(elements.last(), Some(None)) {
                    self.emit(",");
                }
                self.emit("]");
            }
            Pattern::Object { properties, .. } if properties.is_empty() => self.emit("{}"),
            Pattern::Object { properties, .. } => {
                self.emit("{ ");
                for (i, property) in properties.iter().enumerate() {
                    if i > 0 {
                        self.emit(", ");
                    }
                    match property {
                        ObjectPatternProperty::Property {
                            key,
                            value,
                            shorthand,
                        } => {
                            let keeps_name = match (key, pattern_name(value)) {
                                (PropertyKey::Name(k), Some(n)) => js_name(n) == k.as_str(),
                                _ => false,
                            };
                            // `{ for }` binds `$for`, so it must be spelled `{ for: $for }`
                            if !(*shorthand && keeps_name) {
                                self.gen_property_key(key);
                                self.emit(": ");
                            }
                            self.gen_pattern(value);
                        }
                        ObjectPatternProperty::Rest(argument) => {
                            self.emit("...");
                            self.gen_pattern(argument);
                        }
                    }
                }
                self.emit(" }");
            }
        }
    }

    /// Operand of an operator: arrows and assignments bind looser than any
    /// operator, so they need parentheses (`(x => x) || f`).
    fn gen_operand(&mut self, expr: &Expression) {
        if matches!(
            expr,
            Expression::ArrowFunction { .. } | Expression::Assignment { .. }
        ) {
            self.emit("(");
            self.gen_expression(expr);
            self.emit(")");
        } else {
            self.gen_expression(expr);
        }
    }

    /// Callee or object of a member access: prefix operators need parentheses
    /// too (`(await p).x`, `(() => 1)()`).
    fn gen_callee(&mut self, expr: &Expression) {
        let prefix = matches!(
            expr,
            Expression::Unary { .. }
                | Expression::Await { .. }
                | Expression::TypeOf { .. }
                | Expression::Delete { .. }
        );
        if prefix {
            self.emit("(");
            self.gen_expression(expr);
            self.emit(")");
        } else {
            self.gen_operand(expr);
        }
    }

    fn write_indent(&mut self) {
        for _ in 0..self.indent {
            self.emit("  ");
        }
    }
}

/// JavaScript reserved words that FratmScript allows as names (`tien for = 1`).
const JS_RESERVED: &[&str] = &[
    "await",
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "debugger",
    "default",
    "delete",
    "do",
    "else",
    "enum",
    "export",
    "extends",
    "false",
    "finally",
    "for",
    "function",
    "if",
    "implements",
    "import",
    "in",
    "instanceof",
    "interface",
    "let",
    "new",
    "null",
    "package",
    "private",
    "protected",
    "public",
    "return",
    "static",
    "super",
    "switch",
    "this",
    "throw",
    "true",
    "try",
    "typeof",
    "var",
    "void",
    "while",
    "with",
    "yield",
];

/// Escapes a FratmScript name that is reserved in JS. `$` cannot appear in
/// FratmScript identifiers, so `$for` never clashes with a user name.
pub fn js_name(name: &str) -> std::borrow::Cow<'_, str> {
    if JS_RESERVED.contains(&name) {
        format!("${}", name).into()
    } else {
        name.into()
    }
}

/// The bound name of a shorthand pattern (`a` or `a = 1`).
fn pattern_name(pattern: &Pattern) -> Option<&str> {
    match pattern {
        Pattern::Identifier { name, .. } => Some(name),
        Pattern::Default { target, .. } => pattern_name(target),
        _ => None,
    }
}

fn js_names(names: &[String]) -> String {
    names
        .iter()
        .map(|n| js_name(n))
        .collect::<Vec<_>>()
        .join(", ")
}

impl Default for CodeGen {
    fn default() -> Self {
        Self::new(false)
    }
}
