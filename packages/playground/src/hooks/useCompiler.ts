import { useState, useEffect, useCallback, useRef } from 'react'
import { loadWasm, compile, isCompilerLoaded } from '@/lib/compiler'
import { runInWorker, SYNC_TIME_LIMIT_MS, TOTAL_TIME_LIMIT_MS, type LogEntry, type RunEnd } from '@/lib/runner'
import { parseSourceMap, originalPosition, sourceLinePerGeneratedLine } from '@/lib/sourcemap'
import type { Message } from '@/lib/i18n'

export type PlaygroundError =
  | { kind: 'compile'; message: string; line?: number; column?: number; suggestion?: string }
  | { kind: 'runtime'; name: string; message: string; line?: number; column?: number }
  | { kind: 'timeout'; seconds: number }
  | { kind: 'load' }

interface CompilerState {
  isLoading: boolean
  isWasmLoaded: boolean
  version: string
  jsOutput: string
  /** Source line for each line of jsOutput (null when unmapped) */
  jsLineMap: (number | null)[]
  consoleLogs: LogEntry[]
  errors: PlaygroundError[]
  /** Muted note shown under the console (stopped, truncated, …) */
  notice: Message | null
  /** Compile time in milliseconds of the last run */
  compileTime: number | null
  status: 'loading' | 'ready' | 'compiling' | 'running' | 'error'
  statusText: Message
}

export function useCompiler() {
  const [state, setState] = useState<CompilerState>({
    isLoading: true,
    isWasmLoaded: false,
    version: '0.1.0',
    jsOutput: '',
    jsLineMap: [],
    consoleLogs: [],
    errors: [],
    notice: null,
    compileTime: null,
    status: 'loading',
    statusText: { key: 'status.loading' },
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
        status: loaded ? 'ready' : 'error',
        statusText: { key: loaded ? 'status.ready' : 'status.loadFailed' },
        errors: loaded ? [] : [{ kind: 'load' }],
      }))
    })
    return () => stopRef.current?.()
  }, [])

  const runCode = useCallback((source: string) => {
    // Without the compiler there is nothing to run; keep the load error visible
    if (!isCompilerLoaded()) return
    stopRef.current?.()
    const runId = ++runIdRef.current

    const result = compile(source)

    if (!result.success || !result.code) {
      setState(prev => ({
        ...prev,
        jsOutput: '',
        jsLineMap: [],
        consoleLogs: [],
        errors: [{
          kind: 'compile',
          // The position is shown separately, drop the "Riga N, colonna M:" prefix
          message: (result.error || 'Unknown error').replace(/^Riga \d+, colonna \d+:\s*/, ''),
          line: result.line,
          column: result.column,
          suggestion: result.suggestion,
        }],
        notice: null,
        compileTime: result.compileTime,
        status: 'error',
        statusText: { key: 'status.compileError' },
      }))
      return
    }

    const sourceMap = result.sourceMap ? parseSourceMap(result.sourceMap) : null
    const toSource = (line?: number, column?: number) => {
      if (line === undefined) return {}
      return sourceMap ? originalPosition(sourceMap, line, column ?? 1) ?? {} : {}
    }

    setState(prev => ({
      ...prev,
      jsOutput: result.code!,
      jsLineMap: sourceMap ? sourceLinePerGeneratedLine(sourceMap) : [],
      consoleLogs: [],
      errors: [],
      notice: null,
      compileTime: result.compileTime,
      status: 'running',
      statusText: { key: 'status.running' },
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
        setState(prev => ({ ...prev, notice: { key: 'notice.truncated', params: { limit } } }))
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
            errors.push({ kind: 'timeout', seconds: SYNC_TIME_LIMIT_MS / 1000 })
          } else if (reason === 'async-timeout') {
            notice = { key: 'notice.asyncTimeout', params: { seconds: TOTAL_TIME_LIMIT_MS / 1000 } }
          } else if (reason === 'stopped') {
            notice = { key: 'notice.stopped' }
          }
          const failed = errors.length > 0
          return {
            ...prev,
            errors,
            notice,
            status: failed ? 'error' : 'ready',
            statusText: {
              key: failed
                ? (errors.some(e => e.kind === 'timeout') ? 'status.timeout' : 'status.runtimeError')
                : reason === 'completed'
                  ? 'status.done'
                  : 'status.stopped',
            },
          }
        })
      },
    })
    stopRef.current = stop
  }, [])

  const stop = useCallback(() => {
    stopRef.current?.()
  }, [])

  const clearConsole = useCallback(() => {
    setState(prev => ({ ...prev, consoleLogs: [], notice: null }))
  }, [])

  return {
    ...state,
    runCode,
    stop,
    clearConsole,
  }
}
