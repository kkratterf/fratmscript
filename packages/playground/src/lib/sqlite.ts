// `apri_db` opens its database through node:sqlite, which a browser lacks.
// In the playground worker we hand it a small stand-in built on sql.js
// (SQLite compiled to WebAssembly), so database examples run in the page.
// Only the parts of DatabaseSync that apri_db uses are implemented.

import type { Database, SqlValue, Statement } from 'sql.js'
import wasmUrl from 'sql.js/dist/sql-wasm-browser.wasm?url'

type Row = Record<string, SqlValue>

function bind(statement: Statement, values: unknown[]) {
  if (values.length > 0) statement.bind(values as SqlValue[])
}

class StatementSync {
  constructor(
    private db: Database,
    private sql: string,
  ) {
    // Fail on bad SQL now, as node:sqlite does
    db.prepare(sql).free()
  }

  private withStatement<T>(values: unknown[], use: (statement: Statement) => T): T {
    const statement = this.db.prepare(this.sql)
    try {
      bind(statement, values)
      return use(statement)
    } finally {
      statement.free()
    }
  }

  all(...values: unknown[]): Row[] {
    return this.withStatement(values, statement => {
      const rows: Row[] = []
      while (statement.step()) rows.push(statement.getAsObject())
      return rows
    })
  }

  get(...values: unknown[]): Row | undefined {
    return this.withStatement(values, statement =>
      statement.step() ? statement.getAsObject() : undefined,
    )
  }

  run(...values: unknown[]) {
    this.withStatement(values, statement => {
      while (statement.step()) {
        // drain RETURNING rows
      }
    })
    const lastId = this.db.exec('SELECT last_insert_rowid()')[0]?.values[0][0] ?? 0
    return { changes: this.db.getRowsModified(), lastInsertRowid: Number(lastId) }
  }
}

/**
 * Loads sql.js and installs `process.getBuiltinModule("node:sqlite")`.
 * Every database lives in memory, whatever file name the code passes.
 */
export async function installSqlite(scope: Record<string, unknown>) {
  const { default: initSqlJs } = await import('sql.js')
  const SQL = await initSqlJs({ locateFile: () => wasmUrl })

  class DatabaseSync {
    private db: Database
    constructor(_file?: string) {
      this.db = new SQL.Database()
    }
    prepare(sql: string) {
      return new StatementSync(this.db, sql)
    }
    exec(sql: string) {
      this.db.exec(sql)
    }
    close() {
      this.db.close()
    }
  }

  const sqlite = { DatabaseSync }
  scope.process = {
    getBuiltinModule: (name: string) => (name === 'node:sqlite' || name === 'sqlite' ? sqlite : undefined),
  }
}
