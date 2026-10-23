// Types for WASM module
interface WasmModule {
  compile: (source: string, sourceMap: boolean) => CompileResult
  version: () => string
  default: (input?: { module_or_path?: WebAssembly.Module | URL | string }) => Promise<void>
}

interface CompileResult {
  success: boolean
  code: string
  sourceMap?: string
  error?: string
  line?: number
  column?: number
  suggestion?: string
}

let wasmModule: WasmModule | null = null
let isWasmLoaded = false

export async function loadWasm(): Promise<{ loaded: boolean; version?: string }> {
  try {
    // Use Vite's BASE_URL for correct path on GitHub Pages
    const base = import.meta.env.BASE_URL || '/'
    const wasmJsUrl = `${window.location.origin}${base}pkg/fratm_wasm.js`
    const module = await import(/* @vite-ignore */ wasmJsUrl) as WasmModule
    // Initialize the WASM module with the .wasm file path
    const wasmBinaryUrl = `${window.location.origin}${base}pkg/fratm_wasm_bg.wasm`
    await module.default({ module_or_path: wasmBinaryUrl })
    wasmModule = module
    isWasmLoaded = true
    return { loaded: true, version: module.version() }
  } catch (e) {
    console.error('Failed to load the FratmScript compiler:', e)
    return { loaded: false }
  }
}

export function isCompilerLoaded(): boolean {
  return isWasmLoaded
}

export interface CompilationOutput {
  success: boolean
  code?: string
  sourceMap?: string
  error?: string
  line?: number
  column?: number
  suggestion?: string
  compileTime: number
}

export function compile(source: string): CompilationOutput {
  const startTime = performance.now()

  if (isWasmLoaded && wasmModule) {
    try {
      const result = wasmModule.compile(source, true)
      const compileTime = performance.now() - startTime

      if (result.success) {
        return {
          success: true,
          code: result.code,
          sourceMap: result.sourceMap,
          compileTime,
        }
      } else {
        return {
          success: false,
          // The position is shown separately, so drop the "Riga X, colonna Y: " prefix
          error: result.line ? result.error?.replace(/^Riga \d+, colonna \d+: /, '') : result.error,
          line: result.line,
          column: result.column,
          suggestion: result.suggestion,
          compileTime,
        }
      }
    } catch (e) {
      return {
        success: false,
        error: e instanceof Error ? e.message : 'Internal error',
        compileTime: performance.now() - startTime,
      }
    }
  }
  return {
    success: false,
    error: 'The compiler is not loaded',
    compileTime: performance.now() - startTime,
  }
}
