import { useState, useEffect, useCallback } from 'react'
import { loadWasm, compile, executeCode, isCompilerLoaded, type CompilationOutput, type LogEntry } from '@/lib/compiler'

interface CompilerState {
  isLoading: boolean
  isWasmLoaded: boolean
  version: string
  jsOutput: string
  consoleLogs: LogEntry[]
  error: CompilationOutput | null
  compileTime: string
  status: 'loading' | 'ready' | 'compiling' | 'error'
  statusText: string
}

export function useCompiler() {
  const [state, setState] = useState<CompilerState>({
    isLoading: true,
    isWasmLoaded: false,
    version: '0.1.0',
    jsOutput: '// Press "Run" to compile',
    consoleLogs: [],
    error: null,
    compileTime: '',
    status: 'loading',
    statusText: 'Loading...',
  })

  useEffect(() => {
    loadWasm().then(({ loaded, version }) => {
      setState(prev => ({
        ...prev,
        isLoading: false,
        isWasmLoaded: loaded,
        version: version || '0.1.0',
        status: 'ready',
        statusText: loaded ? 'WASM loaded' : 'Demo mode (WASM not available)',
      }))
    })
  }, [])

  const runCode = useCallback((source: string) => {
    setState(prev => ({
      ...prev,
      status: 'compiling',
      statusText: 'Compiling...',
      error: null,
    }))

    const result = compile(source)

    if (result.success && result.code) {
      const { logs, error } = executeCode(result.code)

      setState(prev => ({
        ...prev,
        jsOutput: result.code!,
        consoleLogs: logs,
        error: error ? { ...result, success: false, error } : null,
        compileTime: result.isDemo ? 'Demo mode' : `Compiled in ${result.compileTime.toFixed(1)}ms`,
        status: error ? 'error' : 'ready',
        statusText: error ? 'Runtime error' : (result.isDemo ? 'Demo - compiled' : 'Compilation successful'),
      }))
    } else {
      setState(prev => ({
        ...prev,
        jsOutput: '// Compilation error',
        consoleLogs: [],
        error: result,
        compileTime: `${result.compileTime.toFixed(1)}ms`,
        status: 'error',
        statusText: 'Compilation error',
      }))
    }
  }, [])

  return {
    ...state,
    runCode,
    isReady: !state.isLoading && isCompilerLoaded() || !state.isLoading,
  }
}
