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
            Statement::VariableDecl { name, value, is_const, span, .. } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                self.emit(if *is_const { "const " } else { "let " });
                self.emit(name);
                if let Some(val) = value {
                    self.emit(" = ");
                    self.gen_expression(val);
                }
                self.emit(";");
            }

            Statement::FunctionDecl { name, params, body, is_async, span, .. } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                if *is_async { self.emit("async "); }
                self.emit("function ");
                self.emit(name);
                self.emit("(");
                self.emit(&params.join(", "));
                self.emit(") {\n");
                self.indent += 1;
                for s in body { self.gen_statement(s); self.emit("\n"); }
                self.indent -= 1;
                self.write_indent();
                self.emit("}");
            }

            Statement::Return { value, span, .. } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                self.emit("return");
                if let Some(val) = value { self.emit(" "); self.gen_expression(val); }
                self.emit(";");
            }

            Statement::If { condition, then_branch, else_branch, span, .. } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                self.emit("if (");
                self.gen_expression(condition);
                self.emit(") {\n");
                self.indent += 1;
                for s in then_branch { self.gen_statement(s); self.emit("\n"); }
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
                    for s in else_body { self.gen_statement(s); self.emit("\n"); }
                    self.indent -= 1;
                    self.write_indent();
                    self.emit("}");
                }
            }

            Statement::While { condition, body, span, .. } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                self.emit("while (");
                self.gen_expression(condition);
                self.emit(") {\n");
                self.indent += 1;
                for s in body { self.gen_statement(s); self.emit("\n"); }
                self.indent -= 1;
                self.write_indent();
                self.emit("}");
            }

            Statement::For { init, condition, update, body, span, .. } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                self.emit("for (");
                if let Some(i) = init {
                    match i.as_ref() {
                        Statement::VariableDecl { name, value, is_const, .. } => {
                            self.emit(if *is_const { "const " } else { "let " });
                            self.emit(name);
                            if let Some(val) = value { self.emit(" = "); self.gen_expression(val); }
                        }
                        Statement::Expression { expression, .. } => self.gen_expression(expression),
                        _ => {}
                    }
                }
                self.emit("; ");
                if let Some(c) = condition { self.gen_expression(c); }
                self.emit("; ");
                if let Some(u) = update { self.gen_expression(u); }
                self.emit(") {\n");
                self.indent += 1;
                for s in body { self.gen_statement(s); self.emit("\n"); }
                self.indent -= 1;
                self.write_indent();
                self.emit("}");
            }

            Statement::Break { .. } => { self.write_indent(); self.emit("break;"); }
            Statement::Continue { .. } => { self.write_indent(); self.emit("continue;"); }
            Statement::Debugger { .. } => { self.write_indent(); self.emit("debugger;"); }

            Statement::TryCatch { try_body, catch_param, catch_body, span, .. } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                self.emit("try {\n");
                self.indent += 1;
                for s in try_body { self.gen_statement(s); self.emit("\n"); }
                self.indent -= 1;
                self.write_indent();
                self.emit("} catch");
                if let Some(param) = catch_param { self.emit(" ("); self.emit(param); self.emit(")"); }
                self.emit(" {\n");
                self.indent += 1;
                for s in catch_body { self.gen_statement(s); self.emit("\n"); }
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

            Statement::ClassDecl { name, methods, span, .. } => {
                self.write_indent();
                self.add_mapping(span.line, span.column);
                self.emit("class ");
                self.emit(name);
                self.emit(" {\n");
                self.indent += 1;
                for method in methods {
                    if let Statement::FunctionDecl { name, params, body, is_async, .. } = method {
                        self.write_indent();
                        if *is_async { self.emit("async "); }
                        self.emit(name);
                        self.emit("(");
                        self.emit(&params.join(", "));
                        self.emit(") {\n");
                        self.indent += 1;
                        for s in body { self.gen_statement(s); self.emit("\n"); }
                        self.indent -= 1;
                        self.write_indent();
                        self.emit("}\n");
                    }
                }
                self.indent -= 1;
                self.write_indent();
                self.emit("}");
            }

            Statement::Import { specifiers, source, .. } => {
                self.write_indent();
                self.emit("import { ");
                let names: Vec<&str> = specifiers.iter().map(|s| s.local.as_str()).collect();
                self.emit(&names.join(", "));
                self.emit(" } from \"");
                self.emit(source);
                self.emit("\";");
            }

            Statement::Export { declaration, default_value, .. } => {
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
                for s in statements { self.gen_statement(s); self.emit("\n"); }
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
                self.emit(name);
            }
            Expression::Number { value, .. } => {
                if *value == value.floor() && value.abs() < 1e15 {
                    self.emit(&(*value as i64).to_string());
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
            Expression::Boolean { value, .. } => self.emit(if *value { "true" } else { "false" }),
            Expression::Null { .. } => self.emit("null"),
            Expression::Undefined { .. } => self.emit("undefined"),
            Expression::This { .. } => self.emit("this"),
            Expression::Array { elements, .. } => {
                self.emit("[");
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 { self.emit(", "); }
                    self.gen_expression(elem);
                }
                self.emit("]");
            }
            Expression::Object { properties, .. } => {
                self.emit("{ ");
                for (i, (key, value)) in properties.iter().enumerate() {
                    if i > 0 { self.emit(", "); }
                    self.emit(key);
                    self.emit(": ");
                    self.gen_expression(value);
                }
                self.emit(" }");
            }
            Expression::Binary { left, operator, right, .. } => {
                self.emit("(");
                self.gen_expression(left);
                self.emit(" ");
                self.emit(operator.to_js());
                self.emit(" ");
                self.gen_expression(right);
                self.emit(")");
            }
            Expression::Unary { operator, operand, .. } => {
                self.emit(operator.to_js());
                self.gen_expression(operand);
            }
            Expression::Assignment { target, value, .. } => {
                self.gen_expression(target);
                self.emit(" = ");
                self.gen_expression(value);
            }
            Expression::Call { callee, arguments, .. } => {
                self.gen_expression(callee);
                self.emit("(");
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 { self.emit(", "); }
                    self.gen_expression(arg);
                }
                self.emit(")");
            }
            Expression::Member { object, property, computed, .. } => {
                self.gen_expression(object);
                if *computed {
                    self.emit("[");
                    self.gen_expression(property);
                    self.emit("]");
                } else {
                    self.emit(".");
                    self.gen_expression(property);
                }
            }
            Expression::New { callee, arguments, .. } => {
                self.emit("new ");
                self.gen_expression(callee);
                self.emit("(");
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 { self.emit(", "); }
                    self.gen_expression(arg);
                }
                self.emit(")");
            }
            Expression::ArrowFunction { params, body, .. } => {
                self.emit("(");
                self.emit(&params.join(", "));
                self.emit(") => ");
                match body {
                    ArrowBody::Expression(e) => self.gen_expression(e),
                    ArrowBody::Block(stmts) => {
                        self.emit("{\n");
                        self.indent += 1;
                        for s in stmts { self.gen_statement(s); self.emit("\n"); }
                        self.indent -= 1;
                        self.write_indent();
                        self.emit("}");
                    }
                }
            }
            Expression::Await { argument, .. } => {
                self.emit("await ");
                self.gen_expression(argument);
            }
            Expression::Ternary { condition, consequent, alternate, .. } => {
                self.emit("(");
                self.gen_expression(condition);
                self.emit(" ? ");
                self.gen_expression(consequent);
                self.emit(" : ");
                self.gen_expression(alternate);
                self.emit(")");
            }
            Expression::ConsoleLog { arguments, .. } => {
                self.emit("console.log(");
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 { self.emit(", "); }
                    self.gen_expression(arg);
                }
                self.emit(")");
            }
            Expression::ConsoleWarn { arguments, .. } => {
                self.emit("console.warn(");
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 { self.emit(", "); }
                    self.gen_expression(arg);
                }
                self.emit(")");
            }
            Expression::ConsoleError { arguments, .. } => {
                self.emit("console.error(");
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 { self.emit(", "); }
                    self.gen_expression(arg);
                }
                self.emit(")");
            }
            Expression::TypeOf { operand, .. } => {
                self.emit("typeof ");
                self.gen_expression(operand);
            }
            Expression::Delete { operand, .. } => {
                self.emit("delete ");
                self.gen_expression(operand);
            }
        }
    }

    fn write_indent(&mut self) {
        for _ in 0..self.indent { self.emit("  "); }
    }
}

impl Default for CodeGen {
    fn default() -> Self { Self::new(false) }
}
