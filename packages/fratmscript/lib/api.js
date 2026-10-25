// Turns the raw responses of the WASM bindings into the public API.

/** @param {any} response */
function toErrors(response) {
  if (response.success) return [];
  return [
    {
      message: response.error,
      line: response.line ?? null,
      column: response.column ?? null,
      suggestion: response.suggestion ?? null,
    },
  ];
}

/** @param {any} wasm */
export function createApi(wasm) {
  function compile(source, options = {}) {
    const { sourceMap = true, filename = 'input.fratm', jsxImportSource } = options;
    const response = wasm.compile(String(source), sourceMap, jsxImportSource ?? undefined);
    if (!response.success) {
      return { code: null, map: null, errors: toErrors(response) };
    }
    let map = null;
    if (response.sourceMap) {
      map = JSON.parse(response.sourceMap);
      map.sources = [filename];
      map.file = filename.replace(/\.fratm$/, '') + '.js';
    }
    return { code: response.code, map, errors: [] };
  }

  function format(source) {
    const response = wasm.format(String(source));
    return response.success
      ? { code: response.code, errors: [] }
      : { code: null, errors: toErrors(response) };
  }

  function version() {
    return wasm.version();
  }

  function runtimeError(error) {
    const name = error && error.name ? String(error.name) : 'Error';
    const message = error && error.message !== undefined ? String(error.message) : String(error);
    return wasm.runtimeError(name, message);
  }

  return { compile, format, version, runtimeError };
}
