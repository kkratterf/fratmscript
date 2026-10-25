// Vite (and Rollup) plugin that compiles imported `.fratm` files.
import { compile } from './index.js';
import { sendResponse, toRequest, underPrefix } from './lib/fetch-middleware.js';

const FRATM = /\.fratm$/;
// A request for a .fratm file that Vite would serve as it is
const RAW_FRATM_REQUEST = /^[^?#]*\.fratm(?:$|[?#])/;

export default function fratmscript(options = {}) {
  const { sourceMap = true, jsxImportSource } = options;
  // `api: "server/api.fratm"` or `{ entry, prefix }`: a module whose default
  // export has `fetch(request)` (a Hono app) answers the requests under prefix
  const api = typeof options.api === 'string' ? { entry: options.api } : options.api;
  const apiPrefix = api?.prefix ?? '/api';
  // Vite resolves "/server/api.fratm" from the project root; "server/…" would be a package
  const apiEntry = api && (/^[./]/.test(api.entry) ? api.entry : `/${api.entry}`);
  return {
    name: 'fratmscript',
    enforce: 'pre',
    configureServer(server) {
      // The dev server only compiles requests it knows are modules. Imports
      // between modules get `?import` from Vite, but the `<script src=…>`
      // in index.html does not: add it, so the entry is compiled too.
      server.middlewares.use((req, _res, next) => {
        if (req.url && RAW_FRATM_REQUEST.test(req.url) && !/[?&]import(?:[=&#]|$)/.test(req.url)) {
          req.url += req.url.includes('?') ? '&import' : '?import';
        }
        next();
      });
      if (!api) return;
      server.middlewares.use(async (req, res, next) => {
        if (!underPrefix(req.url, apiPrefix)) return next();
        try {
          // Loaded on each request: Vite reloads it when a server file changes
          const module = await server.ssrLoadModule(apiEntry);
          const app = module.default;
          if (typeof app?.fetch !== 'function') {
            throw new Error(`${api.entry} adda mannà for predefinit n'app cu fetch(), tipo nu bell Hono()`);
          }
          await sendResponse(res, await app.fetch(await toRequest(req)));
        } catch (error) {
          server.ssrFixStacktrace?.(error);
          next(error);
        }
      });
    },
    transform(source, id) {
      const file = id.split('?', 1)[0];
      if (!FRATM.test(file)) return null;
      const result = compile(source, { sourceMap, filename: file, jsxImportSource });
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
