//! `fratm gomorra`: GomorraSQL from the command line.
//!
//! `fratm gomorra "ripigliammo …"` prints the SQL of a query, and
//! `fratm gomorra --db pizzeria.db` opens a shell on a SQLite file, like the
//! shell of the original GomorraSQL. Queries run in a Node.js process with
//! the built-in `node:sqlite` (Node 22.13 or newer), the same database
//! `apri_db` opens.

use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use fratm_core::gomorra::{translate, GomorraError, Query};
use serde_json::Value;

/// Runs in Node: one SQL statement per stdin line, one JSON answer per
/// stdout line. `columns()` (Node 22.16+) tells reads from writes; older
/// versions fall back on the SQL text.
const NODE_SHELL: &str = r#"
const sqlite = process.getBuiltinModule?.("node:sqlite");
if (!sqlite) {
  console.log(JSON.stringify({ fatal: "fratm gomorra --db vo' Node 22.13 o cchiù nuovo (node:sqlite)" }));
  process.exit(1);
}
let db;
try {
  db = new sqlite.DatabaseSync(process.argv[1]);
} catch (e) {
  console.log(JSON.stringify({ fatal: e.message }));
  process.exit(1);
}
require("node:readline").createInterface({ input: process.stdin }).on("line", (sql) => {
  let answer;
  try {
    const statement = db.prepare(sql);
    const reads = typeof statement.columns === "function"
      ? statement.columns().length > 0
      : /^\s*SELECT\b|\bRETURNING\b/i.test(sql);
    if (reads) {
      answer = { rows: statement.all().map((row) => ({ ...row })) };
    } else {
      const r = statement.run();
      answer = { changes: Number(r.changes), lastId: Number(r.lastInsertRowid) };
    }
  } catch (e) {
    answer = { error: e.message };
  }
  process.stdout.write(JSON.stringify(answer, (_, v) => (typeof v === "bigint" ? v.toString() : v)) + "\n");
});
"#;

/// The query as a template with no `${…}`.
pub fn translate_query(query: &str) -> Result<Query, GomorraError> {
    translate(&[query.to_string()])
}

/// A Node.js process holding a SQLite database open.
pub struct Shell {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

/// What a query gave back.
#[derive(Debug, PartialEq)]
pub enum Answer {
    Rows(Vec<serde_json::Map<String, Value>>),
    Changes { changes: u64, last_id: u64 },
}

impl Shell {
    pub fn open(file: &Path) -> Result<Shell, String> {
        let mut child = Command::new("node")
            .arg("--no-warnings")
            .arg("-e")
            .arg(NODE_SHELL)
            .arg(file)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| {
                format!("Nun trovo Node.js ({e}): fratm gomorra --db vo' Node 22.13 o cchiù nuovo")
            })?;
        let stdin = child.stdin.take().expect("stdin is piped");
        let stdout = BufReader::new(child.stdout.take().expect("stdout is piped"));
        Ok(Shell {
            child,
            stdin,
            stdout,
        })
    }

    /// Runs one SQL statement.
    pub fn run(&mut self, sql: &str) -> Result<Answer, String> {
        // One statement per line: the SQL itself never has line breaks
        let line = sql.replace(['\n', '\r'], " ");
        writeln!(self.stdin, "{line}")
            .and_then(|_| self.stdin.flush())
            .map_err(|e| format!("Node.js s'è fermato: {e}"))?;
        let mut answer = String::new();
        self.stdout
            .read_line(&mut answer)
            .map_err(|e| format!("Node.js s'è fermato: {e}"))?;
        let answer: Value = serde_json::from_str(&answer)
            .map_err(|_| "Node.js s'è fermato senza risposta".to_string())?;
        if let Some(message) = answer["fatal"].as_str().or(answer["error"].as_str()) {
            return Err(message.to_string());
        }
        if let Some(rows) = answer["rows"].as_array() {
            return Ok(Answer::Rows(
                rows.iter().filter_map(|r| r.as_object().cloned()).collect(),
            ));
        }
        Ok(Answer::Changes {
            changes: answer["changes"].as_u64().unwrap_or(0),
            last_id: answer["lastId"].as_u64().unwrap_or(0),
        })
    }
}

impl Drop for Shell {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn cell(value: &Value) -> String {
    match value {
        Value::Null => "nisciun".to_string(),
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

/// The answer as text: a table of rows, or how many rows changed.
pub fn render(answer: &Answer) -> String {
    match answer {
        Answer::Changes { changes: 0, .. } => "✓ Fatto".to_string(),
        Answer::Changes { changes, last_id } => {
            format!("✓ Righe cagnate: {changes} (ultimo id: {last_id})")
        }
        Answer::Rows(rows) if rows.is_empty() => "(nisciuna riga)".to_string(),
        Answer::Rows(rows) => {
            let columns: Vec<&String> = rows[0].keys().collect();
            let cells: Vec<Vec<String>> = rows
                .iter()
                .map(|row| columns.iter().map(|c| cell(&row[c.as_str()])).collect())
                .collect();
            let widths: Vec<usize> = columns
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    cells
                        .iter()
                        .map(|row| row[i].chars().count())
                        .chain([c.chars().count()])
                        .max()
                        .unwrap_or(0)
                })
                .collect();
            let line = |values: Vec<&str>| {
                values
                    .iter()
                    .zip(&widths)
                    .map(|(v, w)| format!("{v}{}", " ".repeat(w - v.chars().count())))
                    .collect::<Vec<_>>()
                    .join(" │ ")
                    .trim_end()
                    .to_string()
            };
            let mut out = vec![
                line(columns.iter().map(|c| c.as_str()).collect()),
                widths
                    .iter()
                    .map(|w| "─".repeat(*w))
                    .collect::<Vec<_>>()
                    .join("─┼─"),
            ];
            for row in &cells {
                out.push(line(row.iter().map(String::as_str).collect()));
            }
            let count = rows.len();
            out.push(format!(
                "({count} {})",
                if count == 1 { "riga" } else { "righe" }
            ));
            out.join("\n")
        }
    }
}

/// `(line, column)` from 1 of an error in a one-line or multi-line query.
pub fn error_position(error: &GomorraError) -> (usize, usize) {
    (error.position.line + 1, error.position.column + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translates_a_query() {
        let query = translate_query("ripigliammo nomme mmiez 'a pizze arò id = 1").unwrap();
        assert_eq!(query.sql, "SELECT nomme FROM pizze WHERE id = 1");
        let error = translate_query("ripigliammo nomme\nmmiez a pizze").unwrap_err();
        assert_eq!(error_position(&error), (2, 1));
    }

    #[test]
    fn renders_rows_as_a_table() {
        let rows: Vec<serde_json::Map<String, Value>> = serde_json::from_str(
            r#"[{"id": 1, "nomme": "Margherita", "note": null}, {"id": 12, "nomme": "Pizza fritta", "note": "bona"}]"#,
        )
        .unwrap();
        assert_eq!(
            render(&Answer::Rows(rows)),
            "id │ nomme        │ note\n\
             ───┼──────────────┼────────\n\
             1  │ Margherita   │ nisciun\n\
             12 │ Pizza fritta │ bona\n\
             (2 righe)"
        );
        assert_eq!(render(&Answer::Rows(vec![])), "(nisciuna riga)");
        assert_eq!(
            render(&Answer::Changes {
                changes: 1,
                last_id: 4
            }),
            "✓ Righe cagnate: 1 (ultimo id: 4)"
        );
    }

    /// Needs Node 22.13+ with node:sqlite, as in CI.
    #[test]
    fn runs_queries_on_sqlite() {
        let mut shell = Shell::open(Path::new(":memory:")).unwrap();
        let sql = |q: &str| translate_query(q).unwrap().sql;
        assert_eq!(
            shell
                .run(&sql(
                    "facimm 'a tavula pizze (id INTEGER PRIMARY KEY, nomme TEXT)"
                ))
                .unwrap(),
            Answer::Changes {
                changes: 0,
                last_id: 0
            }
        );
        assert_eq!(
            shell
                .run(&sql("nzipp 'ngoppa pizze nomme chist \"Marinara\""))
                .unwrap(),
            Answer::Changes {
                changes: 1,
                last_id: 1
            }
        );
        let rows = shell
            .run(&sql(
                "ripigliammo tutto chillo ch'era 'o nuostro mmiez 'a pizze",
            ))
            .unwrap();
        assert_eq!(
            render(&rows),
            "id │ nomme\n───┼─────────\n1  │ Marinara\n(1 riga)"
        );
        let error = shell
            .run(&sql("ripigliammo nomme mmiez 'a pasticcerie"))
            .unwrap_err();
        assert!(error.contains("no such table: pasticcerie"), "{error}");
    }
}
