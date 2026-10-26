import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, readdirSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { test } from 'node:test';
import { parseArgs } from '../index.js';

const cli = fileURLToPath(new URL('../index.js', import.meta.url));

test('parseArgs reads the folder, the template and --tailwind', () => {
  assert.deepEqual(parseArgs(['pizzeria']), { dir: 'pizzeria', template: 'react', tailwind: false });
  assert.deepEqual(parseArgs(['app', '--completa', '--tailwind']), {
    dir: 'app',
    template: 'completa',
    tailwind: true,
  });
  assert.deepEqual(parseArgs(['--semplice']), { dir: undefined, template: 'semplice', tailwind: false });
  assert.equal(parseArgs(['--help']).help, true);
  assert.match(parseArgs(['--nonesiste']).error, /opzione/);
  assert.match(parseArgs(['a', 'b']).error, /cartella sola/);
  assert.match(parseArgs(['--semplice', '--tailwind']).error, /--tailwind/);
});

test('the CLI writes a project and refuses a full folder', () => {
  const dir = mkdtempSync(join(tmpdir(), 'create-fratm-'));
  const out = execFileSync(process.execPath, [cli, 'pizzeria'], { cwd: dir, encoding: 'utf8' });
  assert.match(out, /Progetto pronto/);
  const files = readdirSync(join(dir, 'pizzeria'));
  assert.ok(files.includes('package.json'), files.join(' '));
  const pkg = JSON.parse(readFileSync(join(dir, 'pizzeria', 'package.json'), 'utf8'));
  assert.equal(pkg.name, 'pizzeria');
  assert.throws(() => execFileSync(process.execPath, [cli, 'pizzeria'], { cwd: dir, stdio: 'pipe' }), /esiste già/);
});
