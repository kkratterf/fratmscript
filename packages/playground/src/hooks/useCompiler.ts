import { useState, useEffect, useCallback, useRef } from 'react'
import { loadWasm, compile, isCompilerLoaded } from '@/lib/compiler'
import { runInWorker, SYNC_TIME_LIMIT_MS, TOTAL_TIME_LIMIT_MS, type LogEntry, type RunEnd } from '@/lib/runner'
import { parseSourceMap, originalPosition } from '@/lib/sourcemap'

export type PlaygroundError =
  | { kind: 'compile'; message: string; line?: number; column?: number; suggestion?: string }
  | { kind: 'runtime'; name: string; message: string; line?: number; column?: number }
  | { kind: 'timeout'; message: string }

interface CompilerState {
  isLoading: boolean
  isWasmLoaded: boolean
  version: string
  jsOutput: string
  consoleLogs: LogEntry[]
  errors: PlaygroundError[]
  /** Muted note shown under the console (stopped, truncated, …) */
  notice: string
  compileTime: string
  status: 'loading' | 'ready' | 'compiling' | 'running' | 'error'
  statusText: string
}

export function useCompiler() {
  const [state, setState] = useState<CompilerState>({
    isLoading: true,
    isWasmLoaded: false,
    version: '0.1.0',
    jsOutput: '// Press "Run" to compile',
    consoleLogs: [],
    errors: [],
    notice: '',
    compileTime: '',
    status: 'loading',
    statusText: 'Loading...',
  })

  const stopRef = useRef<(() => void) | null>(null)
  const runIdRef = useRef(0)

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
    return () => stopRef.current?.()
  }, [])

  const runCode = useCallback((source: string) => {
    stopRef.current?.()
    const runId = ++runIdRef.current

    const result = compile(source)
    const compileTime = result.isDemo ? 'Demo mode' : `Compiled in ${result.compileTime.toFixed(1)}ms`

    if (!result.success || !result.code) {
      setState(prev => ({
        ...prev,
        jsOutput: '// Compilation error',
        consoleLogs: [],
        errors: [{
          kind: 'compile',
          // The position is shown separately, drop the "Riga N, colonna M:" prefix
          message: (result.error || 'Unknown error').replace(/^Riga \d+, colonna \d+:\s*/, ''),
          line: result.line,
          column: result.column,
          suggestion: result.suggestion,
        }],
        notice: '',
        compileTime: `${result.compileTime.toFixed(1)}ms`,
        status: 'error',
        statusText: 'Compilation error',
      }))
      return
    }

    const sourceMap = result.sourceMap ? parseSourceMap(result.sourceMap) : null
    const toSource = (line?: number, column?: number) => {
      if (line === undefined) return {}
      if (sourceMap) return originalPosition(sourceMap, line, column ?? 1) ?? {}
      // The demo transpiler keeps lines 1:1
      return result.isDemo ? { line, column } : {}
    }

    setState(prev => ({
      ...prev,
      jsOutput: result.code!,
      consoleLogs: [],
      errors: [],
      notice: '',
      compileTime,
      status: 'running',
      statusText: 'Running...',
    }))

    // Console output can arrive in bursts: flush it once per frame.
    let pending: LogEntry[] = []
    let frame = 0
    const flush = () => {
      frame = 0
      if (runId !== runIdRef.current || pending.length === 0) return
      const batch = pending
      pending = []
      setState(prev => ({ ...prev, consoleLogs: [...prev.consoleLogs, ...batch] }))
    }

    const { stop } = runInWorker(result.code, {
      onLog: entry => {
        pending.push(entry)
        if (!frame) frame = requestAnimationFrame(flush)
      },
      onTruncated: limit => {
        if (runId !== runIdRef.current) return
        setState(prev => ({ ...prev, notice: `Console output truncated after ${limit} messages.` }))
      },
      onError: err => {
        if (runId !== runIdRef.current) return
        const pos = toSource(err.line, err.column)
        setState(prev => ({
          ...prev,
          errors: [...prev.errors, { kind: 'runtime', name: err.name, message: err.message, ...pos }],
        }))
      },
      onEnd: (reason: RunEnd) => {
        if (frame) cancelAnimationFrame(frame)
        flush()
        if (runId !== runIdRef.current) return
        stopRef.current = null
        setState(prev => {
          const errors = [...prev.errors]
          let notice = prev.notice
          if (reason === 'sync-timeout') {
            errors.push({
              kind: 'timeout',
              message: `The code ran for more than ${SYNC_TIME_LIMIT_MS / 1000}s without finishing, so it was stopped. Is there an infinite loop?`,
            })
          } else if (reason === 'async-timeout') {
            notice = `Stopped after ${TOTAL_TIME_LIMIT_MS / 1000}s: timers or promises were still pending.`
          } else if (reason === 'stopped') {
            notice = 'Execution stopped.'
          }
          const failed = errors.length > 0
          return {
            ...prev,
            errors,
            notice,
            status: failed ? 'error' : 'ready',
            statusText: failed
              ? (errors.some(e => e.kind === 'timeout') ? 'Execution timed out' : 'Runtime error')
              : reason === 'completed'
                ? (result.isDemo ? 'Demo - compiled' : 'Done')
                : 'Stopped',
          }
        })
      },
    })
    stopRef.current = stop
  }, [])

  const stop = useCallback(() => {
    stopRef.current?.()
  }, [])

  return {
    ...state,
    runCode,
    stop,
    isReady: !state.isLoading && isCompilerLoaded() || !state.isLoading,
  }
}
