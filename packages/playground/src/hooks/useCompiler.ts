import { useState, useEffect, useCallback, useRef } from 'react'
import { loadWasm, compile, isCompilerLoaded } from '@/lib/compiler'
import { runInWorker, SYNC_TIME_LIMIT_MS, TOTAL_TIME_LIMIT_MS, type LogEntry, type RunEnd } from '@/lib/runner'
import { parseSourceMap, originalPosition, sourceLinePerGeneratedLine } from '@/lib/sourcemap'
import { usesPreview, type PreviewEnvelope } from '@/lib/preview'
import { toLogValue } from '@/lib/logValue'

type SourceMap = NonNullable<ReturnType<typeof parseSourceMap>>
import type { Message } from '@/lib/i18n'

export type PlaygroundError =
  | { kind: 'compile'; message: string; line?: number; column?: number; suggestion?: string }
  | { kind: 'runtime'; name: string; message: string; line?: number; column?: number }
  | { kind: 'timeout'; seconds: number }
  | { kind: 'load' }
  | { kind: 'previewLoad'; message: string }

/**
 * How long a run may take before the UI switches to "running" (Stop button,
 * empty console). Most runs finish well within it, so the previous output is
 * swapped for the new one in a single update instead of flashing empty.
 */
export const RUNNING_UI_DELAY_MS = 250

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
  /** Status to display: 'running' only once a run outlasts RUNNING_UI_DELAY_MS */
  status: 'loading' | 'ready' | 'compiling' | 'running' | 'error'
  statusText: Message
  /** A run is in progress, even while the UI still shows the previous result */
  busy: boolean
  /** Code that imports React renders here instead of running in the console */
  preview: { id: number; code: string } | null
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
    busy: false,
    preview: null,
  })

  const stopRef = useRef<(() => void) | null>(null)
  const runIdRef = useRef(0)
  // Source map of the code in the preview, to point its errors at the .fratm lines
  const previewMapRef = useRef<SourceMap | null>(null)

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
    // Bump the id first so the run being replaced doesn't report "stopped"
    const runId = ++runIdRef.current
    stopRef.current?.()

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
        busy: false,
        preview: null,
      }))
      return
    }

    const sourceMap = result.sourceMap ? parseSourceMap(result.sourceMap) : null
    const toSource = (line?: number, column?: number) => {
      if (line === undefined) return {}
      return sourceMap ? originalPosition(sourceMap, line, column ?? 1) ?? {} : {}
    }

    if (usesPreview(result.code)) {
      // The iframe reports back through handlePreviewMessage
      previewMapRef.current = sourceMap
      stopRef.current = () => {
        runIdRef.current++
        setState(prev => ({ ...prev, preview: null, busy: false, status: 'ready', statusText: { key: 'status.stopped' }, notice: { key: 'notice.stopped' } }))
      }
      setState(prev => ({
        ...prev,
        jsOutput: result.code!,
        jsLineMap: sourceMap ? sourceLinePerGeneratedLine(sourceMap) : [],
        compileTime: result.compileTime,
        consoleLogs: [],
        errors: [],
        notice: null,
        preview: { id: runId, code: result.code! },
        busy: true,
        status: 'running',
        statusText: { key: 'status.previewLoading' },
      }))
      return
    }

    // The previous console, errors and status stay on screen until this run
    // reports something or outlasts RUNNING_UI_DELAY_MS, whichever comes first.
    setState(prev => ({
      ...prev,
      jsOutput: result.code!,
      jsLineMap: sourceMap ? sourceLinePerGeneratedLine(sourceMap) : [],
      compileTime: result.compileTime,
      busy: true,
      preview: null,
    }))
    let fresh = true
    /** Returns an updater that drops the previous run's output, only the first time. */
    const takeFresh = () => {
      const reset = fresh
      fresh = false
      return (prev: CompilerState): CompilerState => (
        reset ? { ...prev, consoleLogs: [], errors: [], notice: null } : prev
      )
    }
    const slowTimer = setTimeout(() => {
      if (runId !== runIdRef.current) return
      const base = takeFresh()
      setState(prev => ({ ...base(prev), status: 'running', statusText: { key: 'status.running' } }))
    }, RUNNING_UI_DELAY_MS)

    // Console output can arrive in bursts: flush it once per frame.
    let pending: LogEntry[] = []
    let frame = 0
    const flush = () => {
      frame = 0
      if (runId !== runIdRef.current || pending.length === 0) return
      const batch = pending
      pending = []
      const base = takeFresh()
      setState(prev => {
        const next = base(prev)
        return { ...next, consoleLogs: [...next.consoleLogs, ...batch] }
      })
    }

    const { stop } = runInWorker(result.code, {
      onLog: entry => {
        pending.push(entry)
        if (!frame) frame = requestAnimationFrame(flush)
      },
      onTruncated: limit => {
        if (runId !== runIdRef.current) return
        const base = takeFresh()
        setState(prev => ({ ...base(prev), notice: { key: 'notice.truncated', params: { limit } } }))
      },
      onError: err => {
        if (runId !== runIdRef.current) return
        const pos = toSource(err.line, err.column)
        const base = takeFresh()
        setState(prev => {
          const next = base(prev)
          return { ...next, errors: [...next.errors, { kind: 'runtime', name: err.name, message: err.message, ...pos }] }
        })
      },
      onEnd: (reason: RunEnd) => {
        clearTimeout(slowTimer)
        if (frame) cancelAnimationFrame(frame)
        flush()
        if (runId !== runIdRef.current) return
        stopRef.current = null
        const base = takeFresh()
        setState(state => {
          const prev = base(state)
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
            busy: false,
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

  const handlePreviewMessage = useCallback((msg: PreviewEnvelope) => {
    if (msg.__fratmPreview !== runIdRef.current) return
    const finish = (prev: CompilerState): CompilerState => {
      if (!prev.busy) return prev
      stopRef.current = null
      const failed = prev.errors.length > 0
      return {
        ...prev,
        busy: false,
        status: failed ? 'error' : 'ready',
        statusText: { key: failed ? 'status.runtimeError' : 'status.previewReady' },
      }
    }
    switch (msg.type) {
      case 'log': {
        const entry: LogEntry = { type: msg.level, args: msg.args.map(arg => toLogValue(arg)) }
        setState(prev => ({ ...prev, consoleLogs: [...prev.consoleLogs, entry] }))
        break
      }
      case 'error': {
        const map = previewMapRef.current
        const pos = msg.line !== undefined && map ? originalPosition(map, msg.line, msg.column ?? 1) ?? {} : {}
        setState(prev => {
          const next: CompilerState = { ...prev, errors: [...prev.errors, { kind: 'runtime', name: msg.name, message: msg.message, ...pos }] }
          // Errors after the first render (in an event handler) also land here
          return prev.busy ? finish(next) : { ...next, status: 'error', statusText: { key: 'status.runtimeError' } }
        })
        break
      }
      case 'rendered':
        setState(finish)
        break
      case 'no-component':
        setState(prev => finish({ ...prev, notice: { key: 'notice.noComponent' } }))
        break
      case 'load-failed':
        setState(prev => finish({ ...prev, preview: null, errors: [...prev.errors, { kind: 'previewLoad', message: msg.message }] }))
        break
    }
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
    handlePreviewMessage,
  }
}
