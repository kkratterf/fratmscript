import assert from 'node:assert/strict';
import { test } from 'node:test';
import { compile, format, version, runtimeError } from '../index.js';
import fratmscript from '../vite.js';

test('compile returns code, a source map and no errors', () => {
  const result = compile('chist è x = 42\nstamm a dì(x)', { filename: 'src/main.fratm' });
  assert.deepEqual(result.errors, []);
  assert.equal(result.code, 'const x = 42;\nconsole.log(x);\n');
  assert.deepEqual(result.map.sources, ['src/main.fratm']);
  assert.equal(result.map.file, 'src/main.js');
  assert.equal(typeof result.map.mappings, 'string');
});

test('compile reports errors in Neapolitan instead of throwing', () => {
  const result = compile('chist è x = ');
  assert.equal(result.code, null);
  assert.equal(result.map, null);
  assert.equal(result.errors.length, 1);
  assert.match(result.errors[0].message, /^Riga 1/);
  assert.equal(result.errors[0].line, 1);
});

test('compile can skip the source map', () => {
  assert.equal(compile('tien a = 1', { sourceMap: false }).map, null);
});

test('compile turns JSX into react/jsx-runtime calls, or another library', () => {
  const react = compile('chist è b = <b class="x">Uè</b>', { sourceMap: false });
  assert.equal(
    react.code,
    'import { jsx as _jsx, jsxs as _jsxs, Fragment as _Fragment } from "react/jsx-runtime";\n' +
      'const b = _jsx("b", { className: "x", children: "Uè" });\n',
  );
  const preact = compile('chist è b = <b />', { sourceMap: false, jsxImportSource: 'preact' });
  assert.match(preact.code, /from "preact\/jsx-runtime"/);
  const plugin = fratmscript({ jsxImportSource: 'preact' });
  assert.match(plugin.transform.call({}, 'x = <b />', '/src/a.fratm').code, /preact\/jsx-runtime/);
});

test('format reindents and keeps comments', () => {
  assert.deepEqual(format('si (overo) {\nstamm a dì(1) // uno\n}'), {
    code: 'si (overo) {\n    stamm a dì(1) // uno\n}\n',
    errors: [],
  });
  assert.equal(format('tien = 1').code, null);
});

test('runtime errors are translated', () => {
  let error;
  try {
    // eslint-disable-next-line no-undef
    nonEsiste;
  } catch (e) {
    error = e;
  }
  assert.match(runtimeError(error), /^'nonEsiste' nun esiste.*\(ReferenceError: nonEsiste is not defined\)$/);
  assert.equal(runtimeError(new Error('Pizza bruciata')), 'Error: Pizza bruciata');
});

test('version matches the package', async () => {
  const pkg = JSON.parse(await import('node:fs').then((fs) => fs.readFileSync(new URL('../package.json', import.meta.url), 'utf8')));
  assert.equal(version(), pkg.version);
});

test('vite plugin compiles .fratm files only', () => {
  const plugin = fratmscript();
  assert.equal(plugin.name, 'fratmscript');
  assert.equal(plugin.transform.call({}, 'const a = 1', '/src/a.js'), null);
  const out = plugin.transform.call({}, 'mann for chist è a = 1', '/src/a.fratm?import');
  assert.equal(out.code, 'export const a = 1;\n');
  assert.deepEqual(out.map.sources, ['/src/a.fratm']);
});

test('vite dev server compiles the .fratm entry of index.html', () => {
  let middleware;
  fratmscript().configureServer({ middlewares: { use: (fn) => (middleware = fn) } });
  const url = (u) => {
    const req = { url: u };
    middleware(req, null, () => {});
    return req.url;
  };
  assert.equal(url('/src/main.fratm'), '/src/main.fratm?import');
  assert.equal(url('/src/main.fratm?t=1'), '/src/main.fratm?t=1&import');
  assert.equal(url('/src/App.fratm?import'), '/src/App.fratm?import');
  assert.equal(url('/src/main.js'), '/src/main.js');
});

test('vite plugin reports errors with their location', () => {
  const plugin = fratmscript();
  let reported;
  const context = {
    error(e) {
      reported = e;
      throw new Error(e.message);
    },
  };
  assert.throws(() => plugin.transform.call(context, '\ntien = 1', '/src/b.fratm'));
  assert.equal(reported.id, '/src/b.fratm');
  assert.deepEqual(reported.loc, { file: '/src/b.fratm', line: 2, column: 5 });
});
