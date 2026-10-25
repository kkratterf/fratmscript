// Module hooks registered by `fratmscript/node`. They run on Node's loader
// thread and compile every .fratm file before it is evaluated.
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { compile } from '../index.js';
import { formatError } from './scripts.js';

const FRATM = /\.fratm$/;

export async function load(url, context, nextLoad) {
  if (!FRATM.test(url.split(/[?#]/, 1)[0])) return nextLoad(url, context);

  const path = fileURLToPath(url);
  const source = await readFile(path, 'utf8');
  const result = compile(source, { filename: path });
  if (result.errors.length > 0) {
    const [first] = result.errors;
    const error = new SyntaxError(formatError(path, first));
    // Replace the loader's own frames with the .fratm position (a stack with
    // no frame at all would be printed in brackets)
    error.stack = `${error.name}: ${error.message}\n    at ${path}:${first.line ?? 1}:${first.column ?? 1}`;
    throw error;
  }

  let code = result.code;
  if (result.map) {
    result.map.sources = [url];
    const map = Buffer.from(JSON.stringify(result.map)).toString('base64');
    code += `\n//# sourceMappingURL=data:application/json;charset=utf-8;base64,${map}\n`;
  }
  return { format: 'module', source: code, shortCircuit: true };
}
