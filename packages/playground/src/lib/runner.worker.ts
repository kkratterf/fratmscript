/// <reference lib="webworker" />
// Runs compiled FratmScript code off the main thread, so an infinite loop
// can be killed (worker.terminate()) instead of freezing the tab.
// Console calls are streamed back as they happen, including async ones.

import type { WorkerRequest, WorkerMessage } from './runner'
import { toLogValue } from './logValue'
import { installSqlite } from './sqlite'

declare const self: DedicatedWorkerGlobalScope

const SOURCE_URL = 'fratm-playground.js'
const MAX_LOGS = 1000

const post = (msg: WorkerMessage) => self.postMessage(msg)

// --- console capture ------------------------------------------------------

let logCount = 0

function capture(level: 'log' | 'warn' | 'error') {
  return (...args: unknown[]) => {
    logCount++
    if (logCount > MAX_LOGS) {
      if (logCount === MAX_LOGS + 1) post({ type: 'truncated', limit: MAX_LOGS })
      return
    }
    post({ type: 'log', level, args: args.map(a => toLogValue(a)) })
  }
}

const userConsole = console as unknown as Record<string, unknown>
userConsole.log = capture('log')
userConsole.info = capture('log')
userConsole.debug = capture('log')
userConsole.warn = capture('warn')
userConsole.error = capture('error')

// --- uncaught errors -------------------------------------------------------

function reportError(err: unknown) {
  let name = 'Error'
  let message = String(err)
  let line: number | undefined
  let column: number | undefined
  if (err instanceof Error) {
    name = err.name
    message = err.message
    const match = err.stack?.match(/fratm-playground\.js:(\d+):(\d+)/)
    if (match) {
      line = Number(match[1])
      column = Number(match[2])
    }
  }
  post({ type: 'error', name, message, line, column })
}

self.addEventListener('error', event => {
  event.preventDefault()
  reportError(event.error ?? event.message)
  scheduleIdleCheck()
})

self.addEventListener('unhandledrejection', event => {
  event.preventDefault()
  reportError(event.reason)
  scheduleIdleCheck()
})

// --- pending async work tracking --------------------------------------------
// We call the run "done" once no timers or fetches are pending and the
// microtask queue has drained. A setInterval that is never cleared keeps the
// run alive until the main thread's time limit stops it.

const realSetTimeout = self.setTimeout.bind(self)
const realClearTimeout = self.clearTimeout.bind(self)
const realSetInterval = self.setInterval.bind(self)
const realClearInterval = self.clearInterval.bind(self)
const realFetch = self.fetch?.bind(self)

const timeouts = new Set<number>()
const intervals = new Set<number>()
let pendingFetches = 0
let idleCheckQueued = false
let finished = false
// False while sql.js loads: its own fetch must not end the run
let started = false

function isIdle() {
  return timeouts.size === 0 && intervals.size === 0 && pendingFetches === 0
}

function scheduleIdleCheck() {
  if (idleCheckQueued || finished || !started) return
  idleCheckQueued = true
  // A macrotask hop lets pending promise reactions run first.
  realSetTimeout(() => {
    idleCheckQueued = false
    if (!finished && isIdle()) {
      finished = true
      post({ type: 'done' })
    }
  }, 0)
}

const g = self as unknown as Record<string, unknown>

g.setTimeout = (handler: TimerHandler, ms?: number, ...args: unknown[]) => {
  const id: number = realSetTimeout(() => {
    timeouts.delete(id)
    try {
      if (typeof handler === 'function') handler(...args)
    } finally {
      scheduleIdleCheck()
    }
  }, ms)
  timeouts.add(id)
  return id
}

g.clearTimeout = (id?: number) => {
  if (id === undefined) return
  realClearTimeout(id)
  timeouts.delete(id)
  scheduleIdleCheck()
}

g.setInterval = (handler: TimerHandler, ms?: number, ...args: unknown[]) => {
  const id: number = realSetInterval(() => {
    if (typeof handler === 'function') handler(...args)
  }, ms)
  intervals.add(id)
  return id
}

g.clearInterval = (id?: number) => {
  if (id === undefined) return
  realClearInterval(id)
  intervals.delete(id)
  scheduleIdleCheck()
}

if (realFetch) {
  g.fetch = (...args: Parameters<typeof fetch>) => {
    pendingFetches++
    return realFetch(...args).finally(() => {
      pendingFetches--
      scheduleIdleCheck()
    })
  }
}

// --- run ---------------------------------------------------------------------

// apri_db asks for node:sqlite; only then is sql.js (about 650 kB) loaded.
const NEEDS_SQLITE = 'node:sqlite'

self.addEventListener('message', async (event: MessageEvent<WorkerRequest>) => {
  if (event.data.type !== 'run') return
  const { code } = event.data
  if (code.includes(NEEDS_SQLITE)) {
    try {
      await installSqlite(g)
    } catch (err) {
      // apri_db then throws its own "nun se pò aprì" error
      console.warn('SQLite nun s\'è caricato:', err instanceof Error ? err.message : String(err))
    }
  }
  // The main thread starts the time limits from here, after any loading
  started = true
  post({ type: 'started' })
  try {
    // Indirect eval runs in global scope; sourceURL gives stack traces a
    // stable file name so we can find the failing line in every browser.
    ;(0, eval)(`${code}\n//# sourceURL=${SOURCE_URL}`)
  } catch (err) {
    reportError(err)
  }
  post({ type: 'sync-done' })
  scheduleIdleCheck()
})
