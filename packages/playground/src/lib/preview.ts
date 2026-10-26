// Component preview: code that uses React (JSX or `chiamm … da "react"`) runs
// in a sandboxed iframe instead of the console worker. React comes from a CDN
// through an import map, so it costs nothing until a preview is opened.

import type { LogEntry, LogValue } from './runner'

export const REACT_VERSION = '19.1.1'

const CDN = 'https://esm.sh'

/**
 * react-dom is built with `react` left external, so it imports the same React
 * copy as the user's code through the import map (two copies break hooks).
 */
export const IMPORT_MAP = {
  imports: {
    'react': `${CDN}/react@${REACT_VERSION}`,
    'react/jsx-runtime': `${CDN}/react@${REACT_VERSION}/jsx-runtime`,
    'react/jsx-dev-runtime': `${CDN}/react@${REACT_VERSION}/jsx-dev-runtime`,
    'react-dom': `${CDN}/react-dom@${REACT_VERSION}?external=react`,
    'react-dom/client': `${CDN}/react-dom@${REACT_VERSION}/client?external=react`,
  },
}

/**
 * Tailwind's browser build, loaded only for code that imports Tailwind: it
 * scans the rendered page and writes the CSS of the classes it finds.
 */
export const TAILWIND_VERSION = '4.1.11'
const TAILWIND_SCRIPT = `https://cdn.jsdelivr.net/npm/@tailwindcss/browser@${TAILWIND_VERSION}`

/**
 * `chiamm "tailwindcss/index.css"` (what a Vite project imports) or
 * `chiamm "tailwindcss"` turns Tailwind on in the preview.
 */
export function usesTailwind(code: string): boolean {
  return /^import "tailwindcss(?:\/index\.css)?";?$/m.test(code)
}

/** Compiled code that should run in the preview: it imports React. */
export function usesPreview(code: string): boolean {
  return /^import\b[^\n]*\bfrom "react(?:\/[\w-]+)?";?$/m.test(code)
}

/** File name the preview gives the user's module in stack traces. */
export const PREVIEW_SOURCE_URL = 'fratm-playground.js'

export type PreviewMessage =
  | { type: 'log'; level: LogEntry['type']; args: LogValue[] }
  | { type: 'error'; name: string; message: string; line?: number; column?: number }
  | { type: 'rendered' }
  | { type: 'no-component' }
  | { type: 'load-failed'; message: string }

/** What the iframe posts to the playground: a PreviewMessage tagged with its run. */
export type PreviewEnvelope = PreviewMessage & { __fratmPreview: number }

/**
 * Runs inside the iframe: forwards console calls and errors to the playground,
 * imports the user's module and renders its default export.
 */
function previewMain(id: number, code: string, sourceUrl: string) {
  const send = (msg: Record<string, unknown>) => parent.postMessage({ __fratmPreview: id, ...msg }, '*')

  // Values postMessage can't clone (functions, DOM nodes) travel as text
  const plain = (value: unknown): unknown => {
    if (typeof value === 'function') return { __fratm: 'special', text: `ƒ ${value.name || 'anonymous'}()` }
    if (typeof value === 'symbol' || typeof value === 'bigint') return { __fratm: 'special', text: String(value) + (typeof value === 'bigint' ? 'n' : '') }
    if (value instanceof Error) return { __fratm: 'special', text: `${value.name}: ${value.message}` }
    if (value && typeof value === 'object') {
      try {
        return structuredClone(value)
      } catch {
        return { __fratm: 'special', text: Object.prototype.toString.call(value) }
      }
    }
    return value
  }
  const levels = { log: 'log', info: 'log', debug: 'log', warn: 'warn', error: 'error' } as const
  for (const [method, level] of Object.entries(levels)) {
    const original = (console as unknown as Record<string, (...args: unknown[]) => void>)[method].bind(console)
    ;(console as unknown as Record<string, unknown>)[method] = (...args: unknown[]) => {
      original(...args)
      send({ type: 'log', level, args: args.map(plain) })
    }
  }

  const report = (error: unknown) => {
    const name = error instanceof Error ? error.name : 'Error'
    const message = error instanceof Error ? error.message : String(error)
    const stack = error instanceof Error ? error.stack ?? '' : ''
    // The module is a blob: URL; its line numbers are the generated JS lines
    const match = stack.match(/(?:fratm-playground\.js|blob:[^\s)]+?):(\d+):(\d+)/)
    send({ type: 'error', name, message, line: match ? Number(match[1]) : undefined, column: match ? Number(match[2]) : undefined })
  }
  addEventListener('error', event => {
    event.preventDefault()
    report(event.error ?? event.message)
  })
  addEventListener('unhandledrejection', event => {
    event.preventDefault()
    report(event.reason)
  })

  // A variable specifier keeps the bundler from resolving these imports:
  // the import map does, inside the iframe
  const load = (specifier: string) => import(/* @vite-ignore */ specifier)

  const run = async () => {
    let react: { createElement: (type: unknown) => unknown }
    let client: { createRoot: (el: Element) => { render: (node: unknown) => void } }
    try {
      ;[react, client] = await Promise.all([load('react'), load('react-dom/client')])
    } catch (error) {
      send({ type: 'load-failed', message: error instanceof Error ? error.message : String(error) })
      return
    }
    const url = URL.createObjectURL(new Blob([`${code}\n//# sourceURL=${sourceUrl}`], { type: 'text/javascript' }))
    let component: unknown
    try {
      component = (await load(url)).default
    } catch (error) {
      report(error)
      return
    }
    if (typeof component !== 'function' && !(component && typeof component === 'object')) {
      send({ type: 'no-component' })
      return
    }
    client.createRoot(document.getElementById('root')!).render(react.createElement(component))
    send({ type: 'rendered' })
  }
  run()
}

/** The iframe's HTML: import map, base styles and the code to run. */
export function previewDocument(id: number, code: string, dark: boolean): string {
  // `</script` inside the JSON would end the script tag early
  const json = (value: unknown) => JSON.stringify(value).replace(/</g, '\\u003c')
  const tailwind = usesTailwind(code)
  // The browser build does the work, so the import itself is an empty module
  const importMap = tailwind
    ? { imports: { ...IMPORT_MAP.imports, 'tailwindcss': 'data:text/javascript,', 'tailwindcss/index.css': 'data:text/javascript,' } }
    : IMPORT_MAP
  return `<!doctype html>
<html lang="it"${dark ? ' class="dark"' : ''}>
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<script type="importmap">${json(importMap)}</script>${tailwind ? `\n<script src="${TAILWIND_SCRIPT}"></script>` : ''}
<style>
  :root { color-scheme: ${dark ? 'dark' : 'light'}; font-family: system-ui, -apple-system, "Segoe UI", sans-serif; line-height: 1.5; }
  body { margin: 16px; background: ${dark ? '#0a0a0a' : '#ffffff'}; color: ${dark ? '#e5e7eb' : '#1f2937'}; }
  button { font: inherit; padding: 4px 12px; border-radius: 6px; border: 1px solid ${dark ? '#3f3f46' : '#d4d4d8'}; background: ${dark ? '#27272a' : '#f4f4f5'}; color: inherit; cursor: pointer; }
</style>
</head>
<body>
<div id="root"></div>
<script type="module">(${previewMain.toString()})(${id}, ${json(code)}, ${json(PREVIEW_SOURCE_URL)})</script>
</body>
</html>`
}
