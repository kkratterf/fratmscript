/**
 * Diagnostics and formatting from the real FratmScript compiler.
 *
 * The compiler is the `fratm-wasm` crate built for Node, copied into the
 * extension's `wasm/` folder at build time (see scripts/bundle.mjs).
 */

import * as fs from 'fs';
import * as path from 'path';
import { Diagnostic, DiagnosticSeverity, Range } from 'vscode-languageserver/node';

interface CompileResponse {
  success: boolean;
  code?: string;
  error?: string;
  line?: number;
  column?: number;
  suggestion?: string;
}

interface WasmToken {
  kind: string | { Identifier?: string };
  span: { line: number; column: number };
  literal: string;
}

interface FratmWasm {
  compile(source: string, sourceMap: boolean): CompileResponse;
  format(source: string): CompileResponse;
  tokenize(source: string): WasmToken[];
}

// `dist/server.js` (bundled) sits one level below the extension root,
// `out/server/compiler.js` (tests) two levels.
const WASM_CANDIDATES = [
  path.join(__dirname, '..', 'wasm', 'fratm_wasm.js'),
  path.join(__dirname, '..', '..', 'wasm', 'fratm_wasm.js')
];

let wasm: FratmWasm | null = null;

function loadWasm(): FratmWasm {
  if (!wasm) {
    const file = WASM_CANDIDATES.find((candidate) => fs.existsSync(candidate));
    if (!file) {
      throw new Error('FratmScript compiler (wasm/fratm_wasm.js) not found');
    }
    wasm = require(file) as FratmWasm;
  }
  return wasm;
}

/** JavaScript keywords that have a FratmScript spelling */
const JS_KEYWORDS: Record<string, string> = {
  const: "chist ... è",
  let: 'tien',
  var: 'tien',
  function: 'facc',
  return: 'piglie',
  if: 'si',
  else: 'sinnò',
  while: 'mentre che',
  true: 'overo',
  false: 'sfòls',
  null: 'nisciun',
  undefined: 'boh',
  class: 'na famiglie',
  new: 'nu bell',
  this: 'stu cos',
  import: 'chiamm',
  export: 'mann for',
  async: 'mo vir',
  await: 'aspett',
  try: 'pruvamm',
  catch: 'e si schiatta',
  finally: 'aropp',
  throw: 'iett',
  switch: 'vedimm',
  yield: 'caccia'
};

/** Range from a 1-based line/column to the end of the word that starts there. */
function rangeAt(lines: string[], line: number, column: number): Range {
  const lineIndex = Math.max(0, Math.min(line - 1, lines.length - 1));
  const text = lines[lineIndex] ?? '';
  const start = Math.max(0, Math.min(column - 1, text.length));
  const word = /^[\p{L}\p{N}_$#']+/u.exec(text.slice(start));
  const length = word ? word[0].length : 1;
  return {
    start: { line: lineIndex, character: start },
    end: { line: lineIndex, character: Math.min(text.length, start + length) }
  };
}

/** Compiler errors plus hints where JavaScript keywords are used as names. */
export function diagnose(source: string): Diagnostic[] {
  const compiler = loadWasm();
  const lines = source.split(/\r?\n/);
  const diagnostics: Diagnostic[] = [];

  const result = compiler.compile(source, false);
  if (!result.success) {
    const message = result.suggestion
      ? `${result.error}\n${result.suggestion}`
      : result.error ?? 'Errore';
    diagnostics.push({
      severity: DiagnosticSeverity.Error,
      range: result.line ? rangeAt(lines, result.line, result.column ?? 1) : rangeAt(lines, 1, 1),
      message,
      source: 'fratmscript'
    });
  }

  for (const token of compiler.tokenize(source)) {
    const name = typeof token.kind === 'object' ? token.kind.Identifier : undefined;
    const fratm = name !== undefined ? JS_KEYWORDS[name] : undefined;
    if (fratm) {
      const start = { line: token.span.line - 1, character: token.span.column - 1 };
      diagnostics.push({
        severity: DiagnosticSeverity.Information,
        range: { start, end: { line: start.line, character: start.character + name!.length } },
        message: `In FratmScript '${name}' se scrive '${fratm}'`,
        source: 'fratmscript'
      });
    }
  }

  return diagnostics;
}

/** The formatted document, or null when it does not compile. */
export function formatSource(source: string): string | null {
  const result = loadWasm().format(source);
  return result.success && result.code !== undefined ? result.code : null;
}
