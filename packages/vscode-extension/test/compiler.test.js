// Runs against the tsc output in out/ and the WASM copied into wasm/.
const assert = require('node:assert/strict');
const { test } = require('node:test');
const { diagnose, formatSource } = require('../out/server/compiler.js');

test('valid code has no diagnostics', () => {
  assert.deepEqual(diagnose('chist è x = 1\nstamm a dì(x)\n'), []);
});

test('compiler errors are reported at their position', () => {
  const [error] = diagnose('stamm a dì(1)\ntien = 1\n');
  assert.equal(error.severity, 1);
  assert.match(error.message, /^Riga 2, colonna 6/);
  assert.deepEqual(error.range, { start: { line: 1, character: 5 }, end: { line: 1, character: 6 } });
});

test('semantic errors from the checker are reported too', () => {
  const [error] = diagnose('tien a = 1\ntien a = 2\n');
  assert.equal(error.severity, 1);
  assert.equal(error.range.start.line, 1);
});

test('JavaScript keywords get a hint, but not inside strings or comments', () => {
  const diagnostics = diagnose('tien a = "const" // let\ntien b = this\n');
  assert.equal(diagnostics.length, 1);
  assert.equal(diagnostics[0].severity, 3);
  assert.match(diagnostics[0].message, /'this' se scrive 'stu cos'/);
  assert.deepEqual(diagnostics[0].range.start, { line: 1, character: 9 });
});

test('formatting uses fratm fmt and refuses broken code', () => {
  assert.equal(formatSource('si (overo) {\nstamm a dì(1)\n}'), 'si (overo) {\n    stamm a dì(1)\n}\n');
  assert.equal(formatSource('tien = 1'), null);
});
