// FratmScript compiler for browsers. Call `init()` once before `compile()`.
import initWasm, * as wasm from './wasm/web/fratm_wasm.js';
import { createApi } from './lib/api.js';

const api = createApi(wasm);
let ready = null;
let loaded = false;

export function init(input) {
  ready ??= initWasm(input === undefined ? undefined : { module_or_path: input }).then(() => {
    loaded = true;
  });
  return ready;
}

function assertLoaded() {
  if (!loaded) throw new Error('fratmscript/web: await init() before using the compiler');
}

export function compile(source, options) {
  assertLoaded();
  return api.compile(source, options);
}

export function format(source) {
  assertLoaded();
  return api.format(source);
}

export function version() {
  assertLoaded();
  return api.version();
}
