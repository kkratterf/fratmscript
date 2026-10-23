export type { CompileOptions, CompileResult, FormatResult, FratmError, SourceMapV3 } from './index.js';
import type { CompileOptions, CompileResult, FormatResult } from './index.js';

/**
 * Loads the WebAssembly module. By default it is fetched next to this file;
 * pass a URL, a Response or a compiled WebAssembly.Module to load it yourself.
 */
export function init(input?: string | URL | Request | Response | BufferSource | WebAssembly.Module): Promise<void>;
export function compile(source: string, options?: CompileOptions): CompileResult;
export function format(source: string): FormatResult;
export function version(): string;
