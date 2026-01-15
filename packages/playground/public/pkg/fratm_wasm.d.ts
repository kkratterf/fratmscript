/* tslint:disable */
/* eslint-disable */

/**
 * Compile FratmScript source to JavaScript
 *
 * Returns a JSON object with:
 * - `success`: boolean
 * - `code`: string (if success)
 * - `sourceMap`: string (if success and requested)
 * - `error`: string (if failure)
 * - `line`: number (if failure)
 * - `column`: number (if failure)
 */
export function compile(source: string, generate_source_map: boolean): any;

/**
 * Initialize panic hook for better error messages in browser console
 */
export function init(): void;

/**
 * Tokenize source code (for syntax highlighting)
 */
export function tokenize(source: string): any;

/**
 * Get the compiler version
 */
export function version(): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly compile: (a: number, b: number, c: number) => any;
    readonly tokenize: (a: number, b: number) => any;
    readonly version: () => [number, number];
    readonly init: () => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
