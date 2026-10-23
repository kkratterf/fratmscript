export interface CompileOptions {
  /** Generate a source map (default: true). */
  sourceMap?: boolean;
  /** Name of the source file, used in the source map (default: "input.fratm"). */
  filename?: string;
}

export interface FratmError {
  /** Error message, in Neapolitan */
  message: string;
  /** 1-based line, when known */
  line: number | null;
  /** 1-based column, when known */
  column: number | null;
  /** A hint on how to fix it, when there is one */
  suggestion: string | null;
}

export interface SourceMapV3 {
  version: number;
  file?: string;
  sources: string[];
  sourcesContent?: (string | null)[];
  names: string[];
  mappings: string;
}

export interface CompileResult {
  /** Generated JavaScript, or null when there are errors */
  code: string | null;
  map: SourceMapV3 | null;
  errors: FratmError[];
}

export interface FormatResult {
  /** Formatted source, or null when the source does not compile */
  code: string | null;
  errors: FratmError[];
}

/** Compiles FratmScript source to JavaScript. Never throws on invalid source. */
export function compile(source: string, options?: CompileOptions): CompileResult;
/** Formats FratmScript source, like `fratm fmt`. */
export function format(source: string): FormatResult;
/** Version of the compiler. */
export function version(): string;
