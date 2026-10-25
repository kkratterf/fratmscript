import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

const hook = fileURLToPath(new URL('../node.js', import.meta.url));

function run(files, entry) {
  const dir = mkdtempSync(join(tmpdir(), 'fratm-node-'));
  for (const [name, source] of Object.entries(files)) writeFileSync(join(dir, name), source);
  const result = spawnSync(process.execPath, ['--import', hook, join(dir, entry)], { encoding: 'utf8' });
  return { ...result, dir };
}

test('node --import fratmscript/node runs .fratm modules that import each other and node:', () => {
  const result = run(
    {
      'util.fratm': 'mann for facc saluta(nome) {\n    piglie `Uè, ${nome}!`\n}\n',
      'main.fratm': 'chiamm { saluta } da "./util.fratm"\nchiamm { basename } da "node:path"\nstamm a dì(saluta(basename("/a/Fé")))\n',
    },
    'main.fratm',
  );
  assert.equal(result.stderr, '');
  assert.equal(result.stdout, 'Uè, Fé!\n');
  assert.equal(result.status, 0);
});

test('stack traces point at the .fratm lines', () => {
  const result = run(
    { 'main.fratm': 'facc sbaglia() {\n    iett nu bell Error("guaio")\n}\nsbaglia()\n' },
    'main.fratm',
  );
  assert.equal(result.status, 1);
  assert.ok(result.stderr.includes(`at sbaglia (${join(result.dir, 'main.fratm')}:2:5)`), result.stderr);
});

test('compile errors are printed in Neapolitan with file, line and column', () => {
  const result = run({ 'main.fratm': 'stamm a dì(1)\nchist è x = ' }, 'main.fratm');
  assert.equal(result.status, 1);
  const file = join(result.dir, 'main.fratm');
  // Node 20 prints "SyntaxError [Error]:", later versions "SyntaxError:"
  assert.match(result.stderr, /SyntaxError( \[Error\])?: FratmScript /);
  assert.ok(result.stderr.includes(`FratmScript ${file}:2:`), result.stderr);
  assert.ok(result.stderr.includes('Ccà ce vo'), result.stderr);
  assert.ok(!result.stderr.includes('node-hooks'), result.stderr);
});
