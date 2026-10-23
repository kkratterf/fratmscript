// Builds the fratm-wasm crate twice: for Node (sync load) and for browsers.
import { execFileSync } from 'node:child_process';
import { rmSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = dirname(dirname(fileURLToPath(import.meta.url)));
const crate = join(root, '..', '..', 'crates', 'fratm-wasm');

for (const target of ['nodejs', 'web']) {
  const outDir = join(root, 'wasm', target === 'nodejs' ? 'node' : 'web');
  rmSync(outDir, { recursive: true, force: true });
  execFileSync(
    'wasm-pack',
    ['build', crate, '--release', '--target', target, '--out-dir', outDir, '--no-pack'],
    { stdio: 'inherit' }
  );
  // wasm-pack writes a `*` .gitignore, which would also hide the files from npm
  rmSync(join(outDir, '.gitignore'), { force: true });
  if (target === 'nodejs') {
    // The Node build is CommonJS, while this package is `"type": "module"`
    writeFileSync(join(outDir, 'package.json'), '{ "type": "commonjs" }\n');
  }
}
