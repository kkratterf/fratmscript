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
    /// Local names of the `gomorra` tag
    gomorra: Vec<String>,
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
            gomorra: Vec::new(),
        }
    }

    pub fn generate(&mut self, program: &Program) -> String {
        self.gomorra = crate::gomorra::tag_names(program);
        for stmt in &program.statements {
            self.gen_statement(stmt);
            self.emit("\n");
        }
        self.output.clone()
    }

    /// Writes the import of the JSX runtime before the program.
    pub fn emit_jsx_import(&mut self, source: &str) {
        self.emit(&jsx_runtime_import(source));
        self.emit("\n");
    }

    /// Writes a shebang line before the program, keeping source map lines in step.
    pub fn emit_shebang(&mut self, line: &str) {
        self.emit(line);
        self.emit("\n");
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
                declarations,
                is_const,
                span,
            } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                self.gen_declarations(declarations, *is_const);
                self.emit(";");
            }

            Statement::FunctionDecl {
                name,
                params,
                body,
                is_async,
                is_generator,
                span,
            } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                if *is_async {
                    self.emit("async ");
                }
                self.emit(if *is_generator {
                    "function* "
                } else {
                    "function "
                });
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
                            declarations,
                            is_const,
                            ..
                        } => self.gen_declarations(declarations, *is_const),
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

            Statement::ForEach {
                binding,
                of,
                is_await,
                iterable,
                body,
                span,
            } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                self.emit(if *is_await { "for await (" } else { "for (" });
                match binding {
                    ForBinding::Let(pattern) => {
                        self.emit("let ");
                        self.gen_pattern(pattern);
                    }
                    ForBinding::Const(pattern) => {
                        self.emit("const ");
                        self.gen_pattern(pattern);
                    }
                    ForBinding::Existing(name) => self.emit(&js_name(name)),
                }
                self.emit(if *of { " of " } else { " in " });
                self.gen_expression(iterable);
                self.emit(") {\n");
                self.gen_block_statements(body);
                self.write_indent();
                self.emit("}");
            }
            Statement::DoWhile {
                body,
                condition,
                span,
            } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                self.emit("do {\n");
                self.gen_block_statements(body);
                self.write_indent();
                self.emit("} while (");
                self.gen_expression(condition);
                self.emit(");");
            }
            Statement::Switch {
                discriminant,
                cases,
                span,
            } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                self.emit("switch (");
                self.gen_expression(discriminant);
                self.emit(") {\n");
                self.indent += 1;
                for case in cases {
                    self.write_indent();
                    match &case.test {
                        Some(test) => {
                            self.emit("case ");
                            self.gen_expression(test);
                            self.emit(":\n");
                        }
                        None => self.emit("default:\n"),
                    }
                    self.gen_block_statements(&case.body);
                }
                self.indent -= 1;
                self.write_indent();
                self.emit("}");
            }
            Statement::Break { label, .. } => {
                self.write_indent();
                self.emit("break");
                if let Some(label) = label {
                    self.emit(" ");
                    self.emit(&js_name(label));
                }
                self.emit(";");
            }
            Statement::Continue { label, .. } => {
                self.write_indent();
                self.emit("continue");
                if let Some(label) = label {
                    self.emit(" ");
                    self.emit(&js_name(label));
                }
                self.emit(";");
            }
            Statement::Labeled { label, body, .. } => {
                self.write_indent();
                self.emit(&js_name(label));
                self.emit(":\n");
                self.gen_statement(body);
            }
            Statement::Debugger { .. } => {
                self.write_indent();
                self.emit("debugger;");
            }

            Statement::TryCatch {
                try_body,
                catch_param,
                catch_body,
                finally_body,
                span,
            } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                self.emit("try {\n");
                self.gen_block_statements(try_body);
                self.write_indent();
                self.emit("}");
                if let Some(catch_body) = catch_body {
                    self.emit(" catch");
                    if let Some(param) = catch_param {
                        self.emit(" (");
                        self.gen_pattern(param);
                        self.emit(")");
                    }
                    self.emit(" {\n");
                    self.gen_block_statements(catch_body);
                    self.write_indent();
                    self.emit("}");
                }
                if let Some(finally_body) = finally_body {
                    self.emit(" finally {\n");
                    self.gen_block_statements(finally_body);
                    self.write_indent();
                    self.emit("}");
                }
            }

            Statement::Throw { value, span } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                self.emit("throw ");
                self.gen_expression(value);
                self.emit(";");
            }

            Statement::ClassDecl {
                name,
                super_class,
                members,
                span,
            } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                self.gen_class(Some(name), super_class.as_deref(), members);
            }

            // The fratm library is inlined: one definition per imported helper
            Statement::Import {
                specifiers, source, ..
            } if source == crate::stdlib::MODULE => {
                for (i, s) in specifiers.iter().enumerate() {
                    if i > 0 {
                        self.emit("\n");
                    }
                    self.write_indent();
                    self.emit("const ");
                    self.emit(&js_name(&s.local));
                    self.emit(" = ");
                    self.emit(crate::stdlib::helper(&s.imported).unwrap_or("undefined"));
                    self.emit(";");
                }
            }

            Statement::Import {
                default,
                namespace,
                specifiers,
                source,
                ..
            } => {
                self.write_indent();
                let mut parts = Vec::new();
                if let Some(name) = default {
                    parts.push(js_name(name).into_owned());
                }
                if let Some(name) = namespace {
                    parts.push(format!("* as {}", js_name(name)));
                }
                if !specifiers.is_empty() {
                    let names: Vec<String> = specifiers
                        .iter()
                        .map(|s| alias(&s.imported, &js_name(&s.local)))
                        .collect();
                    parts.push(format!("{{ {} }}", names.join(", ")));
                }
                if parts.is_empty() {
                    self.emit("import ");
                } else {
                    self.emit("import ");
                    self.emit(&parts.join(", "));
                    self.emit(" from ");
                }
                self.emit(&js_string(source));
                self.emit(";");
            }

            Statement::ExportNamed {
                specifiers, source, ..
            } => {
                self.write_indent();
                // Local names are escaped like everywhere else, unless they
                // name another module's exports (`mann for { for } da "m"`)
                let names: Vec<String> = specifiers
                    .iter()
                    .map(|s| match source {
                        Some(_) => alias(&s.local, &s.exported),
                        None => alias(&js_name(&s.local), &s.exported),
                    })
                    .collect();
                if names.is_empty() {
                    self.emit("export {}");
                } else {
                    self.emit("export { ");
                    self.emit(&names.join(", "));
                    self.emit(" }");
                }
                if let Some(source) = source {
                    self.emit(" from ");
                    self.emit(&js_string(source));
                }
                self.emit(";");
            }

            Statement::ExportAll {
                exported, source, ..
            } => {
                self.write_indent();
                self.emit("export * ");
                if let Some(name) = exported {
                    self.emit("as ");
                    self.emit(name);
                    self.emit(" ");
                }
                self.emit("from ");
                self.emit(&js_string(source));
                self.emit(";");
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

            Statement::Expression { expression, span } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                // A statement starting with `{` would be read as a block, and
                // one starting with `function` or `class` as a declaration
                if starts_like_a_declaration(expression) {
                    self.emit("(");
                    self.gen_expression(expression);
                    self.emit(")");
                } else {
                    self.gen_expression(expression);
                }
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
            Expression::BigInt { digits, .. } => {
                self.emit(digits);
                self.emit("n");
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
            Expression::Sequence { expressions, .. } => {
                for (i, expr) in expressions.iter().enumerate() {
                    if i > 0 {
                        self.emit(", ");
                    }
                    self.gen_expression(expr);
                }
            }
            Expression::Regex { pattern, flags, .. } => {
                self.emit("/");
                self.emit(pattern);
                self.emit("/");
                self.emit(flags);
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
            Expression::Class {
                name,
                super_class,
                members,
                ..
            } => self.gen_class(name.as_deref(), super_class.as_deref(), members),
            Expression::Function {
                name,
                params,
                body,
                is_async,
                is_generator,
                ..
            } => {
                if *is_async {
                    self.emit("async ");
                }
                self.emit(if *is_generator {
                    "function*"
                } else {
                    "function"
                });
                self.emit(" ");
                if let Some(name) = name {
                    self.emit(&js_name(name));
                }
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
            Expression::TaggedTemplate { tag, quasi, .. } => {
                if !self.gen_gomorra(tag, quasi) {
                    self.gen_callee(tag);
                    self.gen_expression(quasi);
                }
            }
            Expression::Jsx { element, span } => {
                self.add_mapping(span.line, span.column);
                self.gen_jsx(element);
            }
            Expression::Import { source, .. } => {
                self.emit("import(");
                self.gen_expression(source);
                self.emit(")");
            }
            Expression::MetaProperty { meta, property, .. } => {
                self.emit(meta);
                self.emit(".");
                self.emit(property);
            }
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
                            kind,
                            params,
                            body,
                            is_async,
                            is_generator,
                        } => {
                            if *is_async {
                                self.emit("async ");
                            }
                            if *is_generator {
                                self.emit("*");
                            }
                            match kind {
                                MethodKind::Getter => self.emit("get "),
                                MethodKind::Setter => self.emit("set "),
                                MethodKind::Method => {}
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
                // JS rejects a unary operator right before `**`: `(-a) ** 2`
                let unary_base = *operator == BinaryOp::Power
                    && matches!(
                        **left,
                        Expression::Unary { .. }
                            | Expression::Await { .. }
                            | Expression::TypeOf { .. }
                            | Expression::Delete { .. }
                    );
                if unary_base {
                    self.emit("(");
                    self.gen_expression(left);
                    self.emit(")");
                } else {
                    self.gen_operand(left);
                }
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
                // `- -x`, `- --x`, `+ +x` must not become `--x`, `---x`, `++x`
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
                let starts_with_plus = matches!(
                    **operand,
                    Expression::Unary {
                        operator: UnaryOp::Plus,
                        ..
                    } | Expression::Update {
                        operator: UpdateOp::Increment,
                        prefix: true,
                        ..
                    }
                );
                if matches!(operator, UnaryOp::Negate) && starts_with_minus
                    || matches!(operator, UnaryOp::Plus) && starts_with_plus
                {
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
                callee,
                arguments,
                optional,
                ..
            } => {
                self.gen_callee(callee);
                if *optional {
                    self.emit("?.");
                }
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
                optional,
                ..
            } => {
                self.gen_callee(object);
                if *optional {
                    self.emit("?.");
                }
                if *computed {
                    self.emit("[");
                    self.gen_expression(property);
                    self.emit("]");
                } else if let Expression::Identifier { name, span } = property.as_ref() {
                    // Property names are never escaped: `x.for` is valid JS
                    if !*optional {
                        self.emit(".");
                    }
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
                // `new f()(1)` and `new a?.b()` mean something else, or nothing
                if has_call_or_optional(callee) {
                    self.emit("(");
                    self.gen_expression(callee);
                    self.emit(")");
                } else {
                    self.gen_callee(callee);
                }
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
                    ArrowBody::Expression(e) if starts_like_a_declaration(e) => {
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
            Expression::Yield {
                argument, delegate, ..
            } => {
                self.emit(if *delegate { "yield*" } else { "yield" });
                if let Some(argument) = argument {
                    self.emit(" ");
                    self.gen_expression(argument);
                }
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

    fn gen_declarations(&mut self, declarations: &[Declarator], is_const: bool) {
        self.emit(if is_const { "const " } else { "let " });
        for (i, declarator) in declarations.iter().enumerate() {
            if i > 0 {
                self.emit(", ");
            }
            self.gen_pattern(&declarator.target);
            if let Some(value) = &declarator.value {
                self.emit(" = ");
                self.gen_expression(value);
            }
        }
    }

    /// `<a href="/">Uè</a>` → `_jsx("a", { href: "/", children: "Uè" })`, the
    /// automatic JSX runtime: `_jsxs` when there are several children, `key`
    /// as the third argument.
    fn gen_jsx(&mut self, element: &JsxElement) {
        let intrinsic = element.name.as_deref().is_some_and(is_intrinsic_tag);
        self.emit(if element.children.len() > 1 {
            "_jsxs("
        } else {
            "_jsx("
        });
        match &element.name {
            None => self.emit("_Fragment"),
            Some(name) if intrinsic => self.emit(&js_string(crate::html::tag(name))),
            Some(name) => {
                for (i, part) in name.split('.').enumerate() {
                    if i == 0 {
                        self.emit(&js_name(part));
                    } else {
                        self.emit(".");
                        self.emit(part);
                    }
                }
            }
        }
        self.emit(", ");
        let mut key = None;
        let mut first = true;
        let mut separator = |gen: &mut Self| {
            gen.emit(if first { "{ " } else { ", " });
            first = false;
        };
        for attribute in &element.attributes {
            match attribute {
                JsxAttribute::Named { name, value, .. } if name == "key" => key = Some(value),
                JsxAttribute::Named { name, value, .. } => {
                    separator(self);
                    let name = match name.as_str() {
                        "class" if intrinsic => "className",
                        "for" if intrinsic => "htmlFor",
                        name => name,
                    };
                    if is_plain_name(name) {
                        self.emit(name);
                    } else {
                        self.emit(&js_string(name));
                    }
                    self.emit(": ");
                    match value {
                        Some(value) => self.gen_expression(value),
                        None => self.emit("true"),
                    }
                }
                JsxAttribute::Spread(e) => {
                    separator(self);
                    self.emit("...");
                    self.gen_expression(e);
                }
            }
        }
        if !element.children.is_empty() {
            separator(self);
            self.emit("children: ");
            let many = element.children.len() > 1;
            if many {
                self.emit("[");
            }
            for (i, child) in element.children.iter().enumerate() {
                if i > 0 {
                    self.emit(", ");
                }
                match child {
                    JsxChild::Text(text) => self.emit(&js_string(text)),
                    JsxChild::Expression(e) => self.gen_expression(e),
                    JsxChild::Element(nested) => self.gen_jsx(nested),
                }
            }
            if many {
                self.emit("]");
            }
        }
        self.emit(if first { "{}" } else { " }" });
        if let Some(key) = key {
            self.emit(", ");
            match key {
                Some(value) => self.gen_expression(value),
                None => self.emit("true"),
            }
        }
        self.emit(")");
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

    /// `class [Name] [extends Parent] { … }`, as a declaration or a value.
    fn gen_class(
        &mut self,
        name: Option<&str>,
        super_class: Option<&str>,
        members: &[ClassMember],
    ) {
        self.emit("class");
        if let Some(name) = name {
            self.emit(" ");
            self.emit(&js_name(name));
        }
        if let Some(parent) = super_class {
            self.emit(" extends ");
            self.emit(&js_name(parent));
        }
        self.emit(" {\n");
        self.indent += 1;
        for member in members {
            self.write_indent();
            match member {
                ClassMember::Field {
                    key,
                    value,
                    is_static,
                    ..
                } => {
                    if *is_static {
                        self.emit("static ");
                    }
                    self.gen_property_key(key);
                    if let Some(value) = value {
                        self.emit(" = ");
                        self.gen_expression(value);
                    }
                    self.emit(";\n");
                }
                ClassMember::Method(method) => {
                    if method.is_static {
                        self.emit("static ");
                    }
                    if method.is_async {
                        self.emit("async ");
                    }
                    if method.is_generator {
                        self.emit("*");
                    }
                    match method.kind {
                        MethodKind::Getter => self.emit("get "),
                        MethodKind::Setter => self.emit("set "),
                        MethodKind::Method => {}
                    }
                    // Translate "costruttore" to JavaScript "constructor"
                    match &method.key {
                        PropertyKey::Name(name) if name == "costruttore" => {
                            self.emit("constructor")
                        }
                        key => self.gen_property_key(key),
                    }
                    self.gen_params(&method.params);
                    self.emit(" {\n");
                    self.gen_block_statements(&method.body);
                    self.write_indent();
                    self.emit("}\n");
                }
                ClassMember::StaticBlock { body, .. } => {
                    self.emit("static {\n");
                    self.gen_block_statements(body);
                    self.write_indent();
                    self.emit("}\n");
                }
            }
        }
        self.indent -= 1;
        self.write_indent();
        self.emit("}");
    }

    /// Operand of an operator: arrows and assignments bind looser than any
    /// operator, so they need parentheses (`(x => x) || f`).
    fn gen_operand(&mut self, expr: &Expression) {
        if matches!(
            expr,
            Expression::ArrowFunction { .. }
                | Expression::Assignment { .. }
                | Expression::Yield { .. }
        ) {
            self.emit("(");
            self.gen_expression(expr);
            self.emit(")");
        } else {
            self.gen_expression(expr);
        }
    }

    /// Callee or object of a member access: prefix operators need parentheses
    /// too (`(await p).x`, `(() => 1)()`), and so do numbers (`(5).toString()`,
    /// since `5.` would be read as the number).
    /// `` gomorra`ripigliammo … arò id = ${id}` `` becomes
    /// `gomorra("SELECT … WHERE id = ?", "SELECT … WHERE id = $1", [id])`.
    /// Returns false when `tag` is not the gomorra of the fratm library.
    fn gen_gomorra(&mut self, tag: &Expression, quasi: &Expression) -> bool {
        let (
            Expression::Identifier { name, .. },
            Expression::Template {
                quasis,
                expressions,
                ..
            },
        ) = (tag, quasi)
        else {
            return false;
        };
        if !self.gomorra.contains(name) {
            return false;
        }
        // The checker reports queries that do not translate
        let Ok(query) = crate::gomorra::translate(quasis) else {
            return false;
        };
        self.emit(&js_name(name));
        self.emit("(");
        self.emit(&js_string(&query.sql));
        self.emit(", ");
        self.emit(&js_string(&query.text));
        self.emit(", [");
        for (i, e) in expressions.iter().enumerate() {
            if i > 0 {
                self.emit(", ");
            }
            self.gen_expression(e);
        }
        self.emit("])");
        true
    }

    fn gen_callee(&mut self, expr: &Expression) {
        let prefix = matches!(
            expr,
            Expression::Unary { .. }
                | Expression::Await { .. }
                | Expression::TypeOf { .. }
                | Expression::Delete { .. }
                | Expression::Function { .. }
                | Expression::Class { .. }
                | Expression::Number { .. }
        );
        if prefix {
            self.emit("(");
            self.gen_expression(expr);
            self.emit(")");
        } else {
            self.gen_operand(expr);
        }
    }

    /// The statements of a block, one level deeper, each on its own line.
    fn gen_block_statements(&mut self, statements: &[Statement]) {
        self.indent += 1;
        for s in statements {
            self.gen_statement(s);
            self.emit("\n");
        }
        self.indent -= 1;
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

/// Whether `name` is reserved in JS, so that `js_name` writes it as `$name`.
pub fn is_js_reserved(name: &str) -> bool {
    JS_RESERVED.contains(&name)
}

/// Escapes a FratmScript name that is reserved in JS. The lexer rejects
/// `$` followed by a reserved word, so `$for` never clashes with a user name.
pub fn js_name(name: &str) -> std::borrow::Cow<'_, str> {
    if is_js_reserved(name) {
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

/// `name`, or `name as alias` when the two differ.
fn alias(name: &str, alias: &str) -> String {
    if name == alias {
        name.to_string()
    } else {
        format!("{} as {}", name, alias)
    }
}

/// `div`, `my-element`, `svg:rect` are HTML/SVG tags (strings for the
/// runtime); `Pizza`, `Menu.Voce` and `motion.div` are components.
fn is_intrinsic_tag(name: &str) -> bool {
    name.contains(':')
        || (name.starts_with(|c: char| c.is_ascii_lowercase()) && !name.contains('.'))
}

/// Whether a property name can be written without quotes.
fn is_plain_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars
        .next()
        .is_some_and(|c| c.is_alphabetic() || c == '_' || c == '$')
        && chars.all(|c| c.is_alphanumeric() || c == '_' || c == '$')
}

/// Line that imports the automatic JSX runtime of `source` (`react`, `preact`…).
pub fn jsx_runtime_import(source: &str) -> String {
    format!(
        "import {{ jsx as _jsx, jsxs as _jsxs, Fragment as _Fragment }} from {};",
        js_string(&format!("{}/jsx-runtime", source))
    )
}

/// A module path as a JS string literal.
fn js_string(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| format!("\"{}\"", value))
}

/// Whether the generated code for `expr` begins with an object literal's `{`
/// or with `function` or `class`, which at the start of a statement (or `{`
/// at the start of an arrow body) would be read as a block or a declaration:
/// `({ a: 1 }).a`, `(function () {})`. A function or class used as a callee
/// or object already gets its own parentheses from `gen_callee`.
fn starts_like_a_declaration(expr: &Expression) -> bool {
    matches!(expr, Expression::Function { .. } | Expression::Class { .. })
        || starts_with_object(expr)
}

fn starts_with_object(expr: &Expression) -> bool {
    match expr {
        Expression::Object { .. } => true,
        Expression::Member { object, .. } => starts_with_object(object),
        Expression::Call { callee, .. } => starts_with_object(callee),
        Expression::Assignment { target, .. } => starts_with_object(target),
        Expression::Update {
            prefix: false,
            argument,
            ..
        } => starts_with_object(argument),
        Expression::Sequence { expressions, .. } => {
            expressions.first().is_some_and(starts_like_a_declaration)
        }
        _ => false,
    }
}

/// Whether a `new` callee holds a call or `?.` in its member chain, so it
/// needs parentheses: `new (f())(1)`, `new (a?.b)()`.
fn has_call_or_optional(expr: &Expression) -> bool {
    match expr {
        Expression::Call { .. } => true,
        Expression::Member {
            object, optional, ..
        } => *optional || has_call_or_optional(object),
        _ => false,
    }
}

impl Default for CodeGen {
    fn default() -> Self {
        Self::new(false)
    }
}
