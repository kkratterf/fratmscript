//! Code Generator for FratmScript

use crate::lexer::Span;
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
    /// Local names of the `stile` tag
    stile: Vec<String>,
    /// Non-fatal warnings, such as JSX words from older versions
    warnings: Vec<String>,
    /// Lists never break: set while measuring how wide a list is on one line
    flat: bool,
    /// Inside the init of a `for (…; …; …)`, where `in` needs parentheses
    no_in: bool,
    /// The arrow function about to be written is the hugged last argument
    /// of a call, so its body may go on the next line
    hugged_arrow: bool,
    /// The hugged arrow put its body on the next line: the call closes on
    /// a line of its own
    arrow_broke: bool,
    /// The next `a && b` chain is the condition of an `if (` on lines of
    /// its own, so its operands are not indented further
    chain_same_indent: bool,
}

/// Span of nodes the compiler makes up (the `_jsx` callee): line 0 adds no
/// source map mapping.
const SYNTHETIC: Span = Span {
    start: 0,
    end: 0,
    line: 0,
    column: 0,
};

/// Column after which objects, arrays and argument lists break into lines.
const WIDTH: usize = 80;

/// Precedence levels of JavaScript expressions, loosest first. An operand
/// gets parentheses only when its level is below the one its place needs.
mod prec {
    pub const SEQUENCE: u8 = 1;
    pub const ASSIGN: u8 = 2;
    pub const CONDITIONAL: u8 = 3;
    pub const NULLISH: u8 = 4;
    pub const OR: u8 = 4;
    pub const AND: u8 = 5;
    pub const BIT_OR: u8 = 6;
    pub const BIT_XOR: u8 = 7;
    pub const BIT_AND: u8 = 8;
    pub const EQUALITY: u8 = 9;
    pub const RELATIONAL: u8 = 10;
    pub const SHIFT: u8 = 11;
    pub const ADDITIVE: u8 = 12;
    pub const MULTIPLICATIVE: u8 = 13;
    pub const POWER: u8 = 14;
    pub const UNARY: u8 = 15;
    pub const UPDATE: u8 = 16;
    pub const CALL: u8 = 17;
    pub const PRIMARY: u8 = 20;
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
            stile: Vec::new(),
            warnings: Vec::new(),
            flat: false,
            no_in: false,
            hugged_arrow: false,
            arrow_broke: false,
            chain_same_indent: false,
        }
    }

    /// The warnings collected by `generate`, each one once.
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    fn warn(&mut self, warning: Option<String>) {
        if let Some(warning) = warning {
            if !self.warnings.contains(&warning) {
                self.warnings.push(warning);
            }
        }
    }

    pub fn generate(&mut self, program: &Program) -> String {
        self.gomorra = crate::gomorra::tag_names(program);
        self.stile = crate::stdlib::local_names(program, "stile");
        // A blank line around functions, classes and blocks, as people
        // write them
        let mut previous: Option<&Statement> = None;
        // Imports stay together, with a blank line after the last one
        let mut after_import = false;
        for stmt in &program.statements {
            let import =
                matches!(stmt, Statement::Import { source, .. } if source != crate::stdlib::MODULE);
            let blank = previous.is_some_and(is_block_like)
                || previous.is_some() && is_block_like(stmt)
                || after_import && !import;
            if blank {
                self.emit("\n");
            }
            after_import = import;
            self.gen_statement(stmt);
            self.emit("\n");
            previous = Some(stmt);
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
        // Line 0 marks nodes the compiler made up, such as the `_jsx` callee
        if self.source_map_enabled && src_line > 0 {
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
                self.gen_body(body);
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
                self.emit("if ");
                self.gen_condition(condition);
                self.emit(" {\n");
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
                self.emit("while ");
                self.gen_condition(condition);
                self.gen_body(body);
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
                self.no_in = true;
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
                self.no_in = false;
                self.emit("; ");
                if let Some(c) = condition {
                    self.gen_expression(c);
                }
                self.emit("; ");
                if let Some(u) = update {
                    self.gen_expression(u);
                }
                self.emit(")");
                self.gen_body(body);
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
                self.gen_value(iterable);
                self.emit(")");
                self.gen_body(body);
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
                self.emit("} while ");
                self.gen_condition(condition);
                self.emit(";");
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
                let helpers: Vec<&str> = specifiers
                    .iter()
                    .map(|s| crate::stdlib::helper(&s.imported).unwrap_or("undefined"))
                    .collect();
                // Helpers that take several lines get a blank line between them
                let separator = if helpers.iter().any(|h| h.contains('\n')) {
                    "\n\n"
                } else {
                    "\n"
                };
                for (i, (s, helper)) in specifiers.iter().zip(helpers).enumerate() {
                    if i > 0 {
                        self.emit(separator);
                    }
                    self.write_indent();
                    self.emit("const ");
                    self.emit(&js_name(&s.local));
                    self.emit(" = ");
                    for (j, line) in helper.lines().enumerate() {
                        if j > 0 {
                            self.emit("\n");
                            if !line.is_empty() {
                                self.write_indent();
                            }
                        }
                        self.emit(line);
                    }
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
                    self.gen_value(val);
                    // `export default function F() {}` is a declaration
                    if !matches!(val, Expression::Function { .. } | Expression::Class { .. }) {
                        self.emit(";");
                    }
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

    /// An expression in a place that takes any expression (a statement, a
    /// condition, `${ … }`).
    fn gen_expression(&mut self, expr: &Expression) {
        self.gen_expr(expr, 0);
    }

    /// An expression in a place that takes one value (an argument, an array
    /// element, an initializer), where a sequence needs parentheses.
    fn gen_value(&mut self, expr: &Expression) {
        self.gen_expr(expr, prec::ASSIGN);
    }

    /// `expr` in a place that needs at least precedence `min`, with
    /// parentheses only when it binds looser than that.
    fn gen_expr(&mut self, expr: &Expression, min: u8) {
        let in_for_init = self.no_in
            && matches!(
                expr,
                Expression::Binary {
                    operator: BinaryOp::In,
                    ..
                }
            );
        if precedence(expr) < min || in_for_init {
            let no_in = std::mem::replace(&mut self.no_in, false);
            self.emit("(");
            self.gen_bare(expr);
            self.emit(")");
            self.no_in = no_in;
        } else {
            self.gen_bare(expr);
        }
    }

    /// `expr` without parentheses around it.
    fn gen_bare(&mut self, expr: &Expression) {
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
                    self.gen_value(expr);
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
                self.gen_body(body);
            }
            Expression::TaggedTemplate { tag, quasi, .. } => {
                if !self.gen_gomorra(tag, quasi) && !self.gen_stile(tag, quasi) {
                    self.gen_callee(tag);
                    self.gen_bare(quasi);
                }
            }
            Expression::Jsx { element, span } => {
                self.add_mapping(span.line, span.column);
                let call = self.jsx_call(element);
                self.gen_bare(&call);
            }
            Expression::Import { source, .. } => {
                self.emit("import");
                self.gen_arguments(std::slice::from_ref(source.as_ref()));
            }
            Expression::MetaProperty { meta, property, .. } => {
                self.emit(meta);
                self.emit(".");
                self.emit(property);
            }
            Expression::Array { elements, .. } => {
                self.gen_list("[", "]", false, elements, Hug::None, |gen, e| {
                    gen.gen_value(e)
                });
            }
            Expression::Object { properties, .. } => {
                self.gen_list("{", "}", true, properties, Hug::None, |gen, p| {
                    gen.gen_property(p)
                });
            }
            Expression::Spread { argument, .. } => {
                self.emit("...");
                self.gen_value(argument);
            }
            Expression::Binary {
                left,
                operator,
                right,
                ..
            } => {
                // Inside `if (` on lines of their own, every operand breaks
                let same_indent = std::mem::take(&mut self.chain_same_indent);
                if same_indent || self.chain_breaks(expr) {
                    self.gen_binary_chain(expr, same_indent);
                    return;
                }
                let level = binary_precedence(operator);
                // `**` is right-associative and takes no unary operator on
                // its left: `(-a) ** 2`, `a ** b ** c`
                let (left_min, right_min) = if *operator == BinaryOp::Power {
                    (prec::UPDATE, prec::POWER)
                } else {
                    (level, level + 1)
                };
                self.gen_binary_operand(left, left_min, operator);
                self.emit(" ");
                self.emit(operator.to_js());
                self.emit(" ");
                self.gen_binary_operand(right, right_min, operator);
            }
            Expression::Unary {
                operator, operand, ..
            } => {
                self.emit(operator.to_js());
                // `- -x`, `- --x`, `+ +x` must not become `--x`, `---x`, `++x`
                let (minus, plus) = starts_with_sign(operand);
                if matches!(operator, UnaryOp::Negate) && minus
                    || matches!(operator, UnaryOp::Plus) && plus
                {
                    self.emit("(");
                    self.gen_expression(operand);
                    self.emit(")");
                } else {
                    self.gen_expr(operand, prec::UNARY);
                }
            }
            Expression::Assignment {
                target,
                operator,
                value,
                ..
            } => {
                self.gen_expr(target, prec::CALL);
                self.emit(" ");
                self.emit(operator.to_js());
                self.emit(" ");
                self.gen_value(value);
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
                self.gen_expr(argument, prec::CALL);
                if !*prefix {
                    self.emit(operator.to_js());
                }
            }
            Expression::Call {
                callee,
                arguments,
                optional,
                ..
            } if !self.gen_member_chain(expr) => {
                self.gen_callee(callee);
                if *optional {
                    self.emit("?.");
                }
                self.gen_arguments(arguments);
            }
            Expression::Call { .. } => {}
            Expression::Member { .. } if self.gen_member_chain(expr) => {}
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
                    self.gen_bare(property);
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
                self.gen_arguments(arguments);
            }
            Expression::ArrowFunction {
                params,
                body,
                is_async,
                ..
            } => {
                let hugged = std::mem::take(&mut self.hugged_arrow);
                if *is_async {
                    self.emit("async ");
                }
                self.gen_params(params);
                self.emit(" =>");
                match body {
                    // A hugged arrow whose body does not fit goes on: `.map((x) =>`
                    // and the body on the next line
                    ArrowBody::Expression(e)
                        if hugged
                            && !self.flat
                            && !starts_like_a_declaration(e)
                            && !matches!(**e, Expression::Array { .. })
                            && {
                                let flat = self.measure(|g| g.gen_value(e));
                                !flat.contains('\n') && !self.fits(flat.chars().count() + 1)
                            } =>
                    {
                        self.emit("\n");
                        self.indent += 1;
                        self.write_indent();
                        self.gen_value(e);
                        self.indent -= 1;
                        self.arrow_broke = true;
                    }
                    // `() => ({ … })`: a bare `{` would start a block
                    ArrowBody::Expression(e) if starts_like_a_declaration(e) => {
                        self.emit(" (");
                        self.gen_expression(e);
                        self.emit(")");
                    }
                    ArrowBody::Expression(e) => {
                        self.emit(" ");
                        self.gen_value(e);
                    }
                    ArrowBody::Block(stmts) => self.gen_body(stmts),
                }
            }
            Expression::Await { argument, .. } => {
                self.emit("await ");
                self.gen_expr(argument, prec::UNARY);
            }
            Expression::Yield {
                argument, delegate, ..
            } => {
                self.emit(if *delegate { "yield*" } else { "yield" });
                if let Some(argument) = argument {
                    self.emit(" ");
                    self.gen_value(argument);
                }
            }
            Expression::Ternary {
                condition,
                consequent,
                alternate,
                ..
            } => {
                // `cond` / `  ? a` / `  : b` when it does not fit on one line
                let broken = !self.flat && {
                    let flat = self.measure(|g| g.gen_bare(expr));
                    flat.contains('\n') || !self.fits(flat.chars().count())
                };
                self.gen_expr(condition, prec::NULLISH);
                if broken {
                    self.indent += 1;
                    self.emit("\n");
                    self.write_indent();
                    self.emit("? ");
                    self.gen_value(consequent);
                    self.emit("\n");
                    self.write_indent();
                    self.emit(": ");
                    self.gen_value(alternate);
                    self.indent -= 1;
                } else {
                    self.emit(" ? ");
                    self.gen_value(consequent);
                    self.emit(" : ");
                    self.gen_value(alternate);
                }
            }
            Expression::ConsoleLog { arguments, .. } => {
                self.emit("console.log");
                self.gen_arguments(arguments);
            }
            Expression::ConsoleWarn { arguments, .. } => {
                self.emit("console.warn");
                self.gen_arguments(arguments);
            }
            Expression::ConsoleError { arguments, .. } => {
                self.emit("console.error");
                self.gen_arguments(arguments);
            }
            Expression::TypeOf { operand, .. } => {
                self.emit("typeof ");
                self.gen_expr(operand, prec::UNARY);
            }
            Expression::Delete { operand, .. } => {
                self.emit("delete ");
                self.gen_expr(operand, prec::UNARY);
            }
        }
    }

    /// `a.b(x).c(y)` that does not fit on one line, or holds a function
    /// body, written one `.c(y)` per line as Prettier does. Returns false,
    /// writing nothing, when the chain has fewer than two calls or fits.
    fn gen_member_chain(&mut self, expr: &Expression) -> bool {
        if self.flat {
            return false;
        }
        // The links from the root outwards: `a`, then `.b`, `(x)`, `.c`, `(y)`
        let mut links = Vec::new();
        let mut root = expr;
        loop {
            match root {
                Expression::Call { callee, .. } => {
                    links.push(root);
                    root = callee;
                }
                Expression::Member { object, .. } => {
                    links.push(root);
                    root = object;
                }
                _ => break,
            }
        }
        links.reverse();
        // A group starts at each `.name`; calls and `[key]` stay in it
        let starts_group = |link: &Expression| {
            matches!(
                link,
                Expression::Member {
                    computed: false,
                    ..
                }
            )
        };
        let mut head = links.iter().take_while(|l| !starts_group(l)).count();
        // Short or capitalized roots keep their first group: `this.voci`,
        // `Object.keys(o)`, `z.object(…)`
        let short_root = match root {
            Expression::This { .. } => true,
            Expression::Identifier { name, .. } => {
                name.chars().count() <= 2 || name.starts_with(|c: char| c.is_uppercase())
            }
            _ => false,
        };
        if short_root && head < links.len() {
            head += 1 + links[head + 1..]
                .iter()
                .take_while(|l| !starts_group(l))
                .count();
        }
        let groups_with_calls = links[head..]
            .iter()
            .enumerate()
            .filter(|(i, l)| {
                matches!(l, Expression::Call { .. })
                    && (*i == 0 || !matches!(links[head + i - 1], Expression::Call { .. }))
            })
            .count();
        if groups_with_calls < 2 {
            return false;
        }
        let flat = self.measure(|g| g.gen_bare(expr));
        if !flat.contains('\n') && self.fits(flat.chars().count()) {
            return false;
        }
        self.gen_callee(root);
        for link in &links[..head] {
            self.gen_link(link);
        }
        self.indent += 1;
        for link in &links[head..] {
            if starts_group(link) {
                self.emit("\n");
                self.write_indent();
            }
            self.gen_link(link);
        }
        self.indent -= 1;
        true
    }

    /// One link of a member chain: `.name`, `?.name`, `[key]` or `(args)`.
    fn gen_link(&mut self, link: &Expression) {
        match link {
            Expression::Call {
                arguments,
                optional,
                ..
            } => {
                if *optional {
                    self.emit("?.");
                }
                self.gen_arguments(arguments);
            }
            Expression::Member {
                property,
                computed,
                optional,
                ..
            } => {
                if *optional {
                    self.emit("?.");
                }
                if *computed {
                    self.emit("[");
                    self.gen_expression(property);
                    self.emit("]");
                } else if let Expression::Identifier { name, span } = property.as_ref() {
                    if !*optional {
                        self.emit(".");
                    }
                    self.add_mapping(span.line, span.column);
                    self.emit(name);
                } else {
                    self.emit(".");
                    self.gen_bare(property);
                }
            }
            _ => {}
        }
    }

    /// `(cond)` of `if` and `while`: a long `a && b && c` goes inside on
    /// lines of its own.
    fn gen_condition(&mut self, condition: &Expression) {
        if !self.flat && matches!(condition, Expression::Binary { .. }) {
            let flat = self.measure(|g| g.gen_expression(condition));
            if !flat.contains('\n') && !self.fits(flat.chars().count() + 4) {
                self.emit("(\n");
                self.indent += 1;
                self.write_indent();
                self.chain_same_indent = true;
                self.gen_expression(condition);
                self.chain_same_indent = false;
                self.indent -= 1;
                self.emit("\n");
                self.write_indent();
                self.emit(")");
                return;
            }
        }
        self.emit("(");
        self.gen_expression(condition);
        self.emit(")");
    }

    /// Whether `a + b + c` is too long for its line, so that it breaks after
    /// each operator.
    fn chain_breaks(&self, expr: &Expression) -> bool {
        if self.flat {
            return false;
        }
        let flat = self.measure(|g| g.gen_bare(expr));
        !flat.contains('\n') && !self.fits(flat.chars().count())
    }

    /// Whether `width` more characters fit on the current line, leaving one
    /// column for the `,` or `;` that usually follows.
    fn fits(&self, width: usize) -> bool {
        self.current_col + width < WIDTH
    }

    /// `a +` / `b +` / `c`: the operands of operators at the same level, one
    /// per line, indented one level unless `same_indent` (inside `if (`).
    fn gen_binary_chain(&mut self, expr: &Expression, same_indent: bool) {
        let Expression::Binary { operator, .. } = expr else {
            return;
        };
        let level = binary_precedence(operator);
        let mut operands = Vec::new();
        let mut first = expr;
        while let Expression::Binary {
            left,
            operator,
            right,
            ..
        } = first
        {
            if binary_precedence(operator) != level || *operator == BinaryOp::Power {
                break;
            }
            operands.push((operator, right.as_ref()));
            first = left;
            // `a % b * c` keeps `(a % b)` as one operand
            if let Expression::Binary {
                operator: inner, ..
            } = first
            {
                if needs_clarity(operator, inner) {
                    break;
                }
            }
        }
        operands.reverse();
        let Some((first_operator, _)) = operands.first() else {
            return;
        };
        if !same_indent {
            self.indent += 1;
        }
        self.gen_binary_operand(first, level, first_operator);
        for (operator, operand) in operands {
            self.emit(" ");
            self.emit(operator.to_js());
            self.emit("\n");
            self.write_indent();
            self.gen_binary_operand(operand, level + 1, operator);
        }
        if !same_indent {
            self.indent -= 1;
        }
    }

    /// An operand of `operator`, with the parentheses its precedence needs
    /// plus the ones that make mixed operators readable or legal:
    /// `(a && b) || c`, `(a ?? b) || c`, `(a % b) * c`.
    fn gen_binary_operand(&mut self, operand: &Expression, min: u8, operator: &BinaryOp) {
        match operand {
            Expression::Binary {
                operator: inner, ..
            } if needs_clarity(operator, inner) => {
                let no_in = std::mem::replace(&mut self.no_in, false);
                self.emit("(");
                self.gen_bare(operand);
                self.emit(")");
                self.no_in = no_in;
            }
            _ => self.gen_expr(operand, min),
        }
    }

    /// An entry of an object literal.
    fn gen_property(&mut self, property: &Property) {
        match property {
            Property::KeyValue { key, value } => {
                self.gen_property_key(key);
                self.emit(": ");
                self.gen_value(value);
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
                self.gen_body(body);
            }
            Property::Spread(argument) => {
                self.emit("...");
                self.gen_value(argument);
            }
        }
    }

    /// ` { … }` with the statements of a function or loop body, one level
    /// deeper, or ` {}` when there are none.
    fn gen_body(&mut self, body: &[Statement]) {
        if body.is_empty() {
            self.emit(" {}");
            return;
        }
        self.emit(" {\n");
        self.gen_block_statements(body);
        self.write_indent();
        self.emit("}");
    }

    /// `(a, b)` of a call: on one line when it fits, otherwise hugging a
    /// function or object at the end (`f(a, () => {` … `})`) or at the start
    /// (`setTimeout(() => {` … `}, 100)`), otherwise one argument per line.
    fn gen_arguments(&mut self, arguments: &[Expression]) {
        let hug = match arguments {
            [.., last] if is_huggable(last) => Hug::Last,
            [first, ..] if is_function(first) => Hug::First,
            _ => Hug::None,
        };
        self.gen_list("(", ")", false, arguments, hug, |gen, e| gen.gen_value(e));
    }

    /// A comma-separated list between `open` and `close`, laid out like
    /// Prettier does: on one line when it fits in `WIDTH` columns and holds
    /// no function body, otherwise with one item per line and a trailing
    /// comma. `pad` puts spaces inside the brackets of a one-line list
    /// (`{ a: 1 }`).
    fn gen_list<T>(
        &mut self,
        open: &str,
        close: &str,
        pad: bool,
        items: &[T],
        hug: Hug,
        gen: fn(&mut Self, &T),
    ) {
        if items.is_empty() {
            self.emit(open);
            self.emit(close);
            return;
        }
        let one_line = |g: &mut Self, items: &[T]| {
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    g.emit(", ");
                }
                gen(g, item);
            }
        };
        let fits = self.flat || {
            let flat = self.measure(|g| {
                g.emit(open);
                one_line(g, items);
                g.emit(close);
            });
            self.fits(flat.chars().count() + usize::from(pad) * 2) && !flat.contains('\n')
        };
        if fits {
            self.emit(open);
            if pad {
                self.emit(" ");
            }
            one_line(self, items);
            if pad {
                self.emit(" ");
            }
            self.emit(close);
            return;
        }
        let n = items.len();
        let hugged = match hug {
            Hug::Last => {
                let head = self.measure(|g| one_line(g, &items[..n - 1]));
                let width = open.len() + head.chars().count() + 3;
                (!head.contains('\n') && self.fits(width))
                    .then_some((&items[..n - 1], &items[n - 1..]))
            }
            Hug::First if n > 1 => {
                let rest = self.measure(|g| one_line(g, &items[1..]));
                (!rest.contains('\n') && rest.chars().count() <= WIDTH / 2)
                    .then_some((&items[..1], &items[1..]))
            }
            _ => None,
        };
        if let Some((head, tail)) = hugged {
            self.emit(open);
            one_line(self, head);
            if !head.is_empty() {
                self.emit(", ");
            }
            self.hugged_arrow = matches!(hug, Hug::Last);
            one_line(self, tail);
            self.hugged_arrow = false;
            // `f((x) =>` / `  body,` / `)`
            if std::mem::take(&mut self.arrow_broke) {
                self.emit(",\n");
                self.write_indent();
            }
            self.emit(close);
            return;
        }
        self.emit(open);
        self.emit("\n");
        self.indent += 1;
        for item in items {
            self.write_indent();
            gen(self, item);
            self.emit(",\n");
        }
        self.indent -= 1;
        self.write_indent();
        self.emit(close);
    }

    /// What `write` emits from the current column with every list kept on
    /// one line, without touching the output or the source map.
    fn measure(&self, write: impl FnOnce(&mut Self)) -> String {
        let mut scratch = CodeGen {
            indent: self.indent,
            output: String::new(),
            source_map_enabled: false,
            source_map_builder: SourceMapBuilder::default(),
            current_line: 0,
            current_col: self.current_col,
            gomorra: self.gomorra.clone(),
            stile: self.stile.clone(),
            warnings: Vec::new(),
            flat: true,
            no_in: self.no_in,
            hugged_arrow: false,
            arrow_broke: false,
            chain_same_indent: false,
        };
        write(&mut scratch);
        scratch.output
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
                self.gen_value(value);
            }
        }
    }

    /// `<a href="/">Uè</a>` → `_jsx("a", { href: "/", children: "Uè" })`, the
    /// automatic JSX runtime: `_jsxs` when there are several children, `key`
    /// as the third argument. Built as a call so that it breaks into lines
    /// like any other.
    fn jsx_call(&mut self, element: &JsxElement) -> Expression {
        let span = SYNTHETIC;
        let identifier = |name: &str| Expression::Identifier {
            name: name.to_string(),
            span,
        };
        let string = |value: &str| Expression::String {
            value: value.to_string(),
            span,
        };
        let intrinsic = element.name.as_deref().is_some_and(is_intrinsic_tag);
        let html_tag = match element.name.as_deref() {
            Some(name) if intrinsic => {
                let (html, warning) = crate::html::jsx_tag(name);
                self.warn(warning);
                Some(html)
            }
            _ => None,
        };
        let tag = match &element.name {
            None => identifier("_Fragment"),
            Some(_) if intrinsic => string(html_tag.unwrap_or_default()),
            Some(name) => {
                let mut parts = name.split('.');
                let first = identifier(parts.next().unwrap_or_default());
                parts.fold(first, |object, part| Expression::Member {
                    object: Box::new(object),
                    property: Box::new(identifier(part)),
                    computed: false,
                    optional: false,
                    span,
                })
            }
        };
        let mut key = None;
        let mut properties = Vec::new();
        for attribute in &element.attributes {
            match attribute {
                JsxAttribute::Named { name, value, .. }
                    if name == "key" || (intrinsic && name == "chiave") =>
                {
                    key = Some(value)
                }
                JsxAttribute::Named { name, value, .. } => {
                    // `tipo="fattimiei"` → `type: "password"`, as in htmlemmurt
                    let html_value = match (html_tag, value) {
                        (Some(tag), Some(Expression::String { value, .. })) => {
                            crate::html::value(name, tag, value)
                        }
                        _ => None,
                    };
                    let name = match html_tag {
                        Some(tag) => {
                            let (prop, warning) = crate::html::jsx_attribute(name, tag);
                            self.warn(warning);
                            prop
                        }
                        None => name,
                    };
                    let key = if is_plain_name(name) {
                        PropertyKey::Name(name.to_string())
                    } else {
                        PropertyKey::String(name.to_string())
                    };
                    let value = match (html_value, value) {
                        (Some(html), _) => string(html),
                        (None, Some(Expression::Object { properties, span }))
                            if html_tag.is_some() && name == "style" =>
                        {
                            Expression::Object {
                                properties: style_object(properties),
                                span: *span,
                            }
                        }
                        (None, Some(value)) => value.clone(),
                        (None, None) => Expression::Boolean { value: true, span },
                    };
                    properties.push(Property::KeyValue { key, value });
                }
                JsxAttribute::Spread(e) => properties.push(Property::Spread(e.clone())),
            }
        }
        let mut children: Vec<Expression> = element
            .children
            .iter()
            .map(|child| match child {
                JsxChild::Text(text) => string(text),
                JsxChild::Expression(e) => e.clone(),
                JsxChild::Element(nested) => self.jsx_call(nested),
            })
            .collect();
        let many = children.len() > 1;
        if !children.is_empty() {
            let value = if many {
                Expression::Array {
                    elements: children,
                    span,
                }
            } else {
                children.remove(0)
            };
            properties.push(Property::KeyValue {
                key: PropertyKey::Name("children".to_string()),
                value,
            });
        }
        let mut arguments = vec![tag, Expression::Object { properties, span }];
        if let Some(key) = key {
            arguments.push(match key {
                Some(value) => value.clone(),
                None => Expression::Boolean { value: true, span },
            });
        }
        Expression::Call {
            callee: Box::new(identifier(if many { "_jsxs" } else { "_jsx" })),
            arguments,
            optional: false,
            span,
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
                self.gen_value(expr);
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
                self.gen_value(default);
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
        let is_field = |member: &ClassMember| matches!(member, ClassMember::Field { .. });
        for (i, member) in members.iter().enumerate() {
            // Methods are set apart by a blank line, fields stay together
            if i > 0 && !(is_field(member) && is_field(&members[i - 1])) {
                self.emit("\n");
            }
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
                        self.gen_value(value);
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
                    self.gen_body(&method.body);
                    self.emit("\n");
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
        // A call like any other, so that long queries break into lines
        let call = Expression::Call {
            callee: Box::new(tag.clone()),
            arguments: vec![
                Expression::String {
                    value: query.sql,
                    span: SYNTHETIC,
                },
                Expression::String {
                    value: query.text,
                    span: SYNTHETIC,
                },
                Expression::Array {
                    elements: expressions.clone(),
                    span: SYNTHETIC,
                },
            ],
            optional: false,
            span: SYNTHETIC,
        };
        self.gen_bare(&call);
        true
    }

    /// `` stile`citofono { culore: russo }` `` becomes
    /// `` stile(`button { color: red }`) ``, with every `${ … }` kept.
    /// Returns false when `tag` is not the stile of the fratm library.
    fn gen_stile(&mut self, tag: &Expression, quasi: &Expression) -> bool {
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
        if !self.stile.contains(name) {
            return false;
        }
        // The checker reports CSS that does not translate
        let Ok(css) = crate::stile::translate(quasis) else {
            return false;
        };
        self.emit(&js_name(name));
        self.emit("(`");
        for (i, chunk) in css.iter().enumerate() {
            self.emit(chunk);
            if let Some(expr) = expressions.get(i) {
                self.emit("${");
                self.gen_expression(expr);
                self.emit("}");
            }
        }
        self.emit("`)");
        true
    }

    /// Callee or object of a member access: prefix operators need parentheses
    /// too (`(await p).x`, `(() => 1)()`), and so do numbers (`(5).toString()`,
    /// since `5.` would be read as the number).
    fn gen_callee(&mut self, expr: &Expression) {
        if matches!(
            expr,
            Expression::Function { .. } | Expression::Class { .. } | Expression::Number { .. }
        ) {
            self.emit("(");
            self.gen_expression(expr);
            self.emit(")");
        } else {
            self.gen_expr(expr, prec::CALL);
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

/// The entries of a `cess={{ … }}` object on an HTML tag with Neapolitan CSS
/// translated: `"sfunno-culore": "russo"` → `backgroundColor: "red"`.
/// Other keys, non-string values, spreads and methods stay as written.
fn style_object(properties: &[Property]) -> Vec<Property> {
    properties
        .iter()
        .map(|property| match property {
            Property::KeyValue { key, value } => {
                let key = match key {
                    PropertyKey::Name(k) | PropertyKey::String(k) => {
                        crate::stile::style_key(k).map_or_else(|| key.clone(), PropertyKey::Name)
                    }
                    _ => key.clone(),
                };
                let value = match value {
                    Expression::String { value, span } => Expression::String {
                        value: crate::stile::style_value(value),
                        span: *span,
                    },
                    _ => value.clone(),
                };
                Property::KeyValue { key, value }
            }
            // `{ culore }` → `color: culore`
            Property::Shorthand { name, span } => match crate::stile::style_key(name) {
                Some(react) => Property::KeyValue {
                    key: PropertyKey::Name(react),
                    value: Expression::Identifier {
                        name: name.clone(),
                        span: *span,
                    },
                },
                None => property.clone(),
            },
            _ => property.clone(),
        })
        .collect()
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
    leftmost_opens_block(expr, false)
}

/// Follows the operands written first (`a` in `a + b`, `a.b`, `a ? b : c`)
/// down to the token the code starts with. `in_callee` is set once the walk
/// went through a callee or object, where functions get parentheses anyway.
fn leftmost_opens_block(expr: &Expression, in_callee: bool) -> bool {
    // Operands that get parentheses of their own start with `(`
    let first = |operand: &Expression, min: u8| {
        precedence(operand) >= min && leftmost_opens_block(operand, in_callee)
    };
    match expr {
        Expression::Object { .. } => true,
        Expression::Function { .. } | Expression::Class { .. } => !in_callee,
        Expression::Member { object, .. } => leftmost_opens_block(object, true),
        Expression::Call { callee, .. } => leftmost_opens_block(callee, true),
        Expression::TaggedTemplate { tag, .. } => leftmost_opens_block(tag, true),
        Expression::Binary { left, operator, .. } => {
            let min = match operator {
                BinaryOp::Power => prec::UPDATE,
                op => binary_precedence(op),
            };
            first(left, min)
        }
        Expression::Ternary { condition, .. } => first(condition, prec::NULLISH),
        Expression::Assignment { target, .. } => first(target, prec::CALL),
        Expression::Update {
            prefix: false,
            argument,
            ..
        } => first(argument, prec::CALL),
        Expression::Sequence { expressions, .. } => {
            expressions.first().is_some_and(|e| first(e, prec::ASSIGN))
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

/// Which item of an argument list may stay on the line of the call when the
/// list does not fit: `f(a, () => {` … `})`.
#[derive(Clone, Copy)]
enum Hug {
    None,
    First,
    Last,
}

/// Precedence of the generated code for `expr` (see `prec`).
fn precedence(expr: &Expression) -> u8 {
    match expr {
        Expression::Sequence { .. } => prec::SEQUENCE,
        Expression::Assignment { .. }
        | Expression::ArrowFunction { .. }
        | Expression::Yield { .. }
        | Expression::Spread { .. } => prec::ASSIGN,
        Expression::Ternary { .. } => prec::CONDITIONAL,
        Expression::Binary { operator, .. } => binary_precedence(operator),
        Expression::Unary { .. }
        | Expression::Await { .. }
        | Expression::TypeOf { .. }
        | Expression::Delete { .. } => prec::UNARY,
        Expression::Number { value, .. } if *value < 0.0 => prec::UNARY,
        Expression::Update { .. } => prec::UPDATE,
        Expression::Call { .. }
        | Expression::Member { .. }
        | Expression::New { .. }
        | Expression::Import { .. }
        | Expression::TaggedTemplate { .. }
        | Expression::Jsx { .. }
        | Expression::ConsoleLog { .. }
        | Expression::ConsoleWarn { .. }
        | Expression::ConsoleError { .. } => prec::CALL,
        _ => prec::PRIMARY,
    }
}

fn binary_precedence(operator: &BinaryOp) -> u8 {
    match operator {
        BinaryOp::Nullish => prec::NULLISH,
        BinaryOp::Or => prec::OR,
        BinaryOp::And => prec::AND,
        BinaryOp::BitOr => prec::BIT_OR,
        BinaryOp::BitXor => prec::BIT_XOR,
        BinaryOp::BitAnd => prec::BIT_AND,
        BinaryOp::Equal | BinaryOp::StrictEqual | BinaryOp::NotEqual | BinaryOp::StrictNotEqual => {
            prec::EQUALITY
        }
        BinaryOp::LessThan
        | BinaryOp::GreaterThan
        | BinaryOp::LessEqual
        | BinaryOp::GreaterEqual
        | BinaryOp::InstanceOf
        | BinaryOp::In => prec::RELATIONAL,
        BinaryOp::ShiftLeft | BinaryOp::ShiftRight | BinaryOp::UnsignedShiftRight => prec::SHIFT,
        BinaryOp::Add | BinaryOp::Subtract => prec::ADDITIVE,
        BinaryOp::Multiply | BinaryOp::Divide | BinaryOp::Modulo => prec::MULTIPLICATIVE,
        BinaryOp::Power => prec::POWER,
    }
}

/// Whether `inner` as an operand of `outer` keeps its parentheses even
/// when precedence does not need them: JS rejects `??` mixed with `&&` or
/// `||`, and `a && b || c`, `a % b * c` or `a & b | c` read better with them.
fn needs_clarity(outer: &BinaryOp, inner: &BinaryOp) -> bool {
    use BinaryOp::*;
    let logical = |op: &BinaryOp| matches!(op, And | Or);
    let bitwise = |op: &BinaryOp| {
        matches!(
            op,
            BitAnd | BitOr | BitXor | ShiftLeft | ShiftRight | UnsignedShiftRight
        )
    };
    match (outer, inner) {
        (Nullish, i) if logical(i) => true,
        (o, Nullish) if logical(o) => true,
        (Or, And) => true,
        (Modulo, Multiply | Divide | Modulo) | (Multiply | Divide, Modulo) => outer != inner,
        (o, i) if bitwise(o) && bitwise(i) => o != i,
        _ => false,
    }
}

/// Whether the code for `expr` starts with `-` or `+`: (minus, plus).
fn starts_with_sign(expr: &Expression) -> (bool, bool) {
    match expr {
        Expression::Unary { operator, .. } => (
            matches!(operator, UnaryOp::Negate),
            matches!(operator, UnaryOp::Plus),
        ),
        Expression::Update {
            operator,
            prefix: true,
            ..
        } => (
            matches!(operator, UpdateOp::Decrement),
            matches!(operator, UpdateOp::Increment),
        ),
        Expression::Number { value, .. } => (*value < 0.0, false),
        _ => (false, false),
    }
}

fn is_function(expr: &Expression) -> bool {
    matches!(
        expr,
        Expression::Function { .. } | Expression::ArrowFunction { .. }
    )
}

/// A last argument that can open on the line of the call.
fn is_huggable(expr: &Expression) -> bool {
    match expr {
        Expression::Object { properties, .. } => !properties.is_empty(),
        Expression::Array { elements, .. } => !elements.is_empty(),
        _ => is_function(expr),
    }
}

/// Statements that get a blank line before them at the top level.
fn is_block_like(stmt: &Statement) -> bool {
    match stmt {
        Statement::FunctionDecl { .. }
        | Statement::ClassDecl { .. }
        | Statement::If { .. }
        | Statement::While { .. }
        | Statement::DoWhile { .. }
        | Statement::For { .. }
        | Statement::ForEach { .. }
        | Statement::Switch { .. }
        | Statement::TryCatch { .. }
        | Statement::Block { .. }
        | Statement::Labeled { .. } => true,
        // The definitions of the fratm helpers
        Statement::Import { source, .. } => source == crate::stdlib::MODULE,
        Statement::Export {
            declaration: Some(decl),
            ..
        } => is_block_like(decl),
        Statement::Export {
            default_value: Some(value),
            ..
        } => matches!(
            value,
            Expression::Function { .. } | Expression::Class { .. }
        ),
        _ => false,
    }
}

impl Default for CodeGen {
    fn default() -> Self {
        Self::new(false)
    }
}

#[cfg(test)]
mod tests {
    fn js(source: &str) -> String {
        crate::compile(source, Default::default()).unwrap().code
    }

    #[test]
    fn parentheses_only_where_precedence_needs_them() {
        assert_eq!(js("x = (a + b) * c - d"), "x = (a + b) * c - d;\n");
        assert_eq!(js("x = a - (b - c)"), "x = a - (b - c);\n");
        assert_eq!(js("x = a ** b ** c"), "x = a ** b ** c;\n");
        assert_eq!(js("x = (a ** b) ** c"), "x = (a ** b) ** c;\n");
        assert_eq!(js("x = -(a + b)"), "x = -(a + b);\n");
        assert_eq!(js("x = no (a e b)"), "x = !(a && b);\n");
        assert_eq!(js("x = (a ? b : c) ? d : e"), "x = (a ? b : c) ? d : e;\n");
        assert_eq!(js("x = a ? b : c ? d : e"), "x = a ? b : c ? d : e;\n");
        assert_eq!(js("x = (() => 1)()"), "x = (() => 1)();\n");
        assert_eq!(js("x = a || (() => 1)"), "x = a || (() => 1);\n");
        assert_eq!(
            js("mo vir facc f() { piglie (aspett p).x }"),
            "async function f() {\n  return (await p).x;\n}\n"
        );
        // Mixed operators keep the parentheses that make them readable
        assert_eq!(js("x = a % b * c"), "x = (a % b) * c;\n");
        assert_eq!(js("x = a & b | c"), "x = (a & b) | c;\n");
    }

    #[test]
    fn in_inside_a_for_header_keeps_its_parentheses() {
        assert_eq!(
            js("pe (tien x = \"a\" dint'a o; x; ) { }"),
            "for (let x = (\"a\" dint'a o); x; ) {}\n".replace("dint'a", "in")
        );
        assert_eq!(js("tien x = \"a\" dint'a o"), "let x = \"a\" in o;\n");
    }

    #[test]
    fn statements_starting_with_an_object_or_function_stay_expressions() {
        assert_eq!(js("({ a: 1 }).a + 1"), "({ a: 1 }.a + 1);\n");
        assert_eq!(
            js("tien f = () => ({ n: 2 }).n * 2"),
            "let f = () => ({ n: 2 }.n * 2);\n"
        );
    }

    #[test]
    fn long_lists_break_one_item_per_line() {
        assert_eq!(
            js("tien pizza = { nomme: \"Margherita\", prezzo: 5, ingredienti: [\"pummarola\", \"muzzarella\"] }"),
            "let pizza = {\n  nomme: \"Margherita\",\n  prezzo: 5,\n  ingredienti: [\"pummarola\", \"muzzarella\"],\n};\n"
        );
        assert_eq!(
            js("stamm a dì(\"primo argomento lungo\", \"secondo argomento lungo\", \"terzo argomento\")"),
            "console.log(\n  \"primo argomento lungo\",\n  \"secondo argomento lungo\",\n  \"terzo argomento\",\n);\n"
        );
        // An object with a method never stays on one line
        assert_eq!(
            js("tien o = { saluta() { piglie 1 } }"),
            "let o = {\n  saluta() {\n    return 1;\n  },\n};\n"
        );
        assert_eq!(
            js("tien o = { a: 1, b: [1, 2] }"),
            "let o = { a: 1, b: [1, 2] };\n"
        );
    }

    #[test]
    fn a_last_function_or_object_argument_hugs_the_call() {
        assert_eq!(
            js("xs.forEach((x) => { stamm a dì(x) })"),
            "xs.forEach((x) => {\n  console.log(x);\n});\n"
        );
        assert_eq!(
            js("aspetta(() => { f() }, 1000)"),
            "aspetta(() => {\n  f();\n}, 1000);\n"
        );
        assert_eq!(
            js("risposta.json({ nomme: \"Margherita\", prezzo: 5, ingredienti: \"pummarola, muzzarella\" })"),
            "risposta.json({\n  nomme: \"Margherita\",\n  prezzo: 5,\n  ingredienti: \"pummarola, muzzarella\",\n});\n"
        );
        // An arrow whose body does not fit puts it on the next line
        assert_eq!(
            js("chist è nomi = pizze.map((pizza) => pizza.nomme + \" (\" + pizza.prezzo + \" euro)\" + pizza.forno)"),
            "const nomi = pizze.map((pizza) =>\n  pizza.nomme + \" (\" + pizza.prezzo + \" euro)\" + pizza.forno,\n);\n"
        );
    }

    #[test]
    fn long_chains_ternaries_and_conditions_break() {
        assert_eq!(
            js("chist è totale = ordine.filter((p) => p.pronta).map((p) => p.prezzo).reduce((a, b) => a + b, 0)"),
            "const totale = ordine\n  .filter((p) => p.pronta)\n  .map((p) => p.prezzo)\n  .reduce((a, b) => a + b, 0);\n"
        );
        assert_eq!(
            js("chist è messaggio = ordine.length == 0 ? \"Ancora nisciuna pizza ordinata\" : \"Pizze ordinate: \" + ordine.length"),
            "const messaggio = ordine.length == 0\n  ? \"Ancora nisciuna pizza ordinata\"\n  : \"Pizze ordinate: \" + ordine.length;\n"
        );
        assert_eq!(
            js("si (pizza.pronta e pizza.prezzo < budget e cliente.affamato e no cliente.saziato) { f() }"),
            "if (\n  pizza.pronta &&\n  pizza.prezzo < budget &&\n  cliente.affamato &&\n  !cliente.saziato\n) {\n  f();\n}\n"
        );
        assert_eq!(
            js("stamm a dì(\"Turno 'e \" + nomeGiocatore(giocatore) + \" cu 'o simbolo \" + simbolo + \", mossa: \" + mossa)"),
            "console.log(\n  \"Turno 'e \" +\n    nomeGiocatore(giocatore) +\n    \" cu 'o simbolo \" +\n    simbolo +\n    \", mossa: \" +\n    mossa,\n);\n"
        );
    }

    #[test]
    fn jsx_breaks_like_any_call() {
        let code = crate::compile(
            "x = <ul classe=\"menu\">{pizze.map((p) => <li chiave={p.id}>{p.nomme}</li>)}</ul>",
            Default::default(),
        )
        .unwrap()
        .code;
        assert_eq!(
            code.lines().skip(1).collect::<Vec<_>>().join("\n"),
            "x = _jsx(\"ul\", {\n  className: \"menu\",\n  children: pizze.map((p) => _jsx(\"li\", { children: p.nomme }, p.id)),\n});"
        );
    }

    #[test]
    fn blank_lines_set_apart_functions_classes_and_blocks() {
        assert_eq!(
            js("chiamm { a } da \"m\"\ntien x = 1\nfacc f() { piglie x }\nf()\nna famiglie C {\ny = 1\nz = 2\nfacc a() { }\nfacc b() { }\n}"),
            "import { a } from \"m\";\n\nlet x = 1;\n\nfunction f() {\n  return x;\n}\n\nf();\n\nclass C {\n  y = 1;\n  z = 2;\n\n  a() {}\n\n  b() {}\n}\n"
        );
    }

    #[test]
    fn export_default_function_has_no_stray_semicolon() {
        assert_eq!(
            js("mann for predefinit facc App() { piglie 1 }"),
            "export default function App() {\n  return 1;\n}\n"
        );
        assert_eq!(js("mann for predefinit 1 + 2"), "export default 1 + 2;\n");
    }

    #[test]
    fn fratm_helpers_are_written_on_several_lines() {
        let code = js("chiamm { mischia } da \"fratm\"");
        assert!(
            code.starts_with("const mischia = (lista) => {\n  const copia = [...lista];\n"),
            "{code}"
        );
        assert!(code.lines().all(|l| l.chars().count() <= 80), "{code}");
    }
}
