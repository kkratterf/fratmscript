// Vite (and Rollup) plugin that compiles imported `.fratm` files.
import { compile } from './index.js';

const FRATM = /\.fratm$/;

export default function fratmscript(options = {}) {
  const { sourceMap = true } = options;
  return {
    name: 'fratmscript',
    enforce: 'pre',
    transform(source, id) {
      const file = id.split('?', 1)[0];
      if (!FRATM.test(file)) return null;
      const result = compile(source, { sourceMap, filename: file });
      if (result.errors.length > 0) {
        const [error] = result.errors;
        this.error({
          message: error.suggestion ? `${error.message}\n${error.suggestion}` : error.message,
          id: file,
          loc: error.line == null ? undefined : { file, line: error.line, column: (error.column ?? 1) - 1 },
        });
      }
      return { code: result.code, map: result.map };
    },
  };
}

export { fratmscript };
