// FratmScript compiler for Node.js (and bundlers that run in Node).
import { createRequire } from 'node:module';
import { createApi } from './lib/api.js';

const wasm = createRequire(import.meta.url)('./wasm/node/fratm_wasm.js');

export const { compile, format, version, runtimeError, scaffold } = createApi(wasm);
