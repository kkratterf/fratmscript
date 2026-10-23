// Main-thread side of the code runner: spawns a fresh worker per run,
// streams its console output and kills it on stop or when it runs too long.

export type LogValue =
  | string
  | number
  | boolean
  | null
  | undefined
  | LogValue[]
  | { __fratm: 'special'; text: string }
  | { [key: string]: LogValue }

export interface LogEntry {
  type: 'log' | 'warn' | 'error'
  args: LogValue[]
}

export interface RuntimeErrorInfo {
  name: string
  message: string
  /** 1-based line/column in the generated JavaScript */
  line?: number
  column?: number
}

export type WorkerRequest = { type: 'run'; code: string }

export type WorkerMessage =
  | { type: 'log'; level: LogEntry['type']; args: LogValue[] }
  | { type: 'truncated'; limit: number }
  | { type: 'error'; name: string; message: string; line?: number; column?: number }
  | { type: 'sync-done' }
  | { type: 'done' }

/**
 * Why a run ended: it finished on its own, the user stopped it, the
 * synchronous part never returned (infinite loop), or async work
 * (timers, intervals) was still going when the time limit hit.
 */
export type RunEnd = 'completed' | 'stopped' | 'sync-timeout' | 'async-timeout'

export interface RunHandlers {
  onLog: (entry: LogEntry) => void
  onTruncated: (limit: number) => void
  onError: (error: RuntimeErrorInfo) => void
  onEnd: (reason: RunEnd) => void
}

export const SYNC_TIME_LIMIT_MS = 3000
export const TOTAL_TIME_LIMIT_MS = 10000

export function runInWorker(code: string, handlers: RunHandlers): { stop: () => void } {
  const worker = new Worker(new URL('./runner.worker.ts', import.meta.url), { type: 'module' })
  let ended = false
  let syncDone = false

  const end = (reason: RunEnd) => {
    if (ended) return
    ended = true
    clearTimeout(syncTimer)
    clearTimeout(totalTimer)
    worker.terminate()
    handlers.onEnd(reason)
  }

  const syncTimer = setTimeout(() => {
    if (!syncDone) end('sync-timeout')
  }, SYNC_TIME_LIMIT_MS)
  const totalTimer = setTimeout(() => end('async-timeout'), TOTAL_TIME_LIMIT_MS)

  worker.onmessage = (event: MessageEvent<WorkerMessage>) => {
    if (ended) return
    const msg = event.data
    switch (msg.type) {
      case 'log':
        handlers.onLog({ type: msg.level, args: msg.args })
        break
      case 'truncated':
        handlers.onTruncated(msg.limit)
        break
      case 'error':
        handlers.onError({ name: msg.name, message: msg.message, line: msg.line, column: msg.column })
        break
      case 'sync-done':
        syncDone = true
        break
      case 'done':
        end('completed')
        break
    }
  }

  worker.onerror = (event) => {
    event.preventDefault()
    handlers.onError({ name: 'Error', message: event.message || 'Worker failed to start' })
    end('completed')
  }

  worker.postMessage({ type: 'run', code } satisfies WorkerRequest)

  return { stop: () => end('stopped') }
}
