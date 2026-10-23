// Bundles the extension and its language server into dist/ (so the .vsix
// needs no node_modules) and copies the WASM compiler into wasm/.
import { build } from 'esbuild';
import { cpSync, existsSync, mkdirSync, rmSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = dirname(dirname(fileURLToPath(import.meta.url)));
const wasmSource = join(root, '..', 'fratmscript', 'wasm', 'node');

if (!existsSync(join(wasmSource, 'fratm_wasm_bg.wasm'))) {
  console.error('The WASM compiler is missing: run `npm run build` in packages/fratmscript first.');
  process.exit(1);
}
rmSync(join(root, 'wasm'), { recursive: true, force: true });
mkdirSync(join(root, 'wasm'));
for (const file of ['fratm_wasm.js', 'fratm_wasm_bg.wasm', 'package.json']) {
  cpSync(join(wasmSource, file), join(root, 'wasm', file));
}

await build({
  entryPoints: {
    extension: join(root, 'src', 'extension.ts'),
    server: join(root, 'src', 'server', 'server.ts')
  },
  outdir: join(root, 'dist'),
  bundle: true,
  platform: 'node',
  format: 'cjs',
  target: 'node18',
  external: ['vscode'],
  sourcemap: true,
  logLevel: 'warning'
});
