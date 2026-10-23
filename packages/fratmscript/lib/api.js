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
    const { sourceMap = true, filename = 'input.fratm' } = options;
    const response = wasm.compile(String(source), sourceMap);
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

  return { compile, format, version };
}
