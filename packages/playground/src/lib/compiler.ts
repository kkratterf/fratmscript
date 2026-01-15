// Types for WASM module
interface WasmModule {
  compile: (source: string, sourceMap: boolean) => CompileResult
  version: () => string
  default: (input?: { module_or_path?: WebAssembly.Module | URL | string }) => Promise<void>
}

interface CompileResult {
  success: boolean
  code: string
  error?: string
  line?: number
  column?: number
  suggestion?: string
}

let wasmModule: WasmModule | null = null
let isWasmLoaded = false

export async function loadWasm(): Promise<{ loaded: boolean; version?: string }> {
  try {
    // Fetch and initialize WASM module from public directory
    const wasmJsUrl = new URL('/pkg/fratm_wasm.js', window.location.origin).href
    const module = await import(/* @vite-ignore */ wasmJsUrl) as WasmModule
    // Initialize the WASM module with the .wasm file path
    const wasmBinaryUrl = new URL('/pkg/fratm_wasm_bg.wasm', window.location.origin).href
    await module.default({ module_or_path: wasmBinaryUrl })
    wasmModule = module
    isWasmLoaded = true
    return { loaded: true, version: module.version() }
  } catch (e) {
    console.warn('WASM not available, using fallback:', e)
    return { loaded: false }
  }
}

export function isCompilerLoaded(): boolean {
  return isWasmLoaded
}

// Simple demo transpiler (when WASM is not available)
function demoTranspile(source: string): string {
  return source
    .replace(/chist è (\w+) = /g, 'const $1 = ')
    .replace(/tien (\w+) = /g, 'let $1 = ')
    .replace(/facc (\w+)\(/g, 'function $1(')
    .replace(/piglie /g, 'return ')
    .replace(/si \(/g, 'if (')
    .replace(/sinnò/g, 'else')
    .replace(/mentre che \(/g, 'while (')
    .replace(/pe \(/g, 'for (')
    .replace(/stamm a dì\(/g, 'console.log(')
    .replace(/overo/g, 'true')
    .replace(/sfòls/g, 'false')
    .replace(/nisciun/g, 'null')
    .replace(/boh/g, 'undefined')
    .replace(/ e /g, ' && ')
    .replace(/ o /g, ' || ')
    .replace(/no /g, '!')
}

export interface CompilationOutput {
  success: boolean
  code?: string
  error?: string
  line?: number
  column?: number
  suggestion?: string
  compileTime: number
  isDemo: boolean
}

export function compile(source: string): CompilationOutput {
  const startTime = performance.now()

  if (isWasmLoaded && wasmModule) {
    try {
      const result = wasmModule.compile(source, false)
      const compileTime = performance.now() - startTime

      if (result.success) {
        return {
          success: true,
          code: result.code,
          compileTime,
          isDemo: false,
        }
      } else {
        return {
          success: false,
          error: result.error,
          line: result.line,
          column: result.column,
          suggestion: result.suggestion,
          compileTime,
          isDemo: false,
        }
      }
    } catch (e) {
      return {
        success: false,
        error: e instanceof Error ? e.message : 'Internal error',
        compileTime: performance.now() - startTime,
        isDemo: false,
      }
    }
  } else {
    // Demo mode
    try {
      const code = demoTranspile(source)
      return {
        success: true,
        code,
        compileTime: performance.now() - startTime,
        isDemo: true,
      }
    } catch (e) {
      return {
        success: false,
        error: e instanceof Error ? e.message : 'Error',
        compileTime: performance.now() - startTime,
        isDemo: true,
      }
    }
  }
}

export interface LogEntry {
  type: 'log' | 'warn' | 'error'
  args: unknown[]
}

export function executeCode(code: string): { logs: LogEntry[]; error?: string } {
  const logs: LogEntry[] = []
  const sandbox = {
    console: {
      log: (...args: unknown[]) => logs.push({ type: 'log', args }),
      warn: (...args: unknown[]) => logs.push({ type: 'warn', args }),
      error: (...args: unknown[]) => logs.push({ type: 'error', args }),
    },
  }

  try {
    const fn = new Function('console', code)
    fn(sandbox.console)
    return { logs }
  } catch (e) {
    return { logs, error: e instanceof Error ? e.message : 'Runtime error' }
  }
}
