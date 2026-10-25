// Turns console arguments into plain data that postMessage can carry and the
// output panel knows how to print. Used by the code runner worker and by the
// component preview.

import type { LogValue } from './runner'

const MAX_DEPTH = 4
const MAX_ITEMS = 100

export function special(text: string): LogValue {
  return { __fratm: 'special', text }
}

export function toLogValue(value: unknown, depth = 0, seen = new WeakSet<object>()): LogValue {
  switch (typeof value) {
    case 'string':
    case 'number':
    case 'boolean':
    case 'undefined':
      return value
    case 'bigint':
      return special(`${value}n`)
    case 'symbol':
      return special(value.toString())
    case 'function':
      return special(`ƒ ${value.name || 'anonymous'}()`)
  }
  if (value === null) return null

  const obj = value as object
  if (obj instanceof Error) return special(`${obj.name}: ${obj.message}`)
  if (obj instanceof Date) return special(obj.toISOString())
  if (obj instanceof RegExp) return special(String(obj))
  if (obj instanceof Promise) return special('Promise { <pending> }')
  if (seen.has(obj)) return special('[Circular]')
  if (depth >= MAX_DEPTH) return special(Array.isArray(obj) ? '[…]' : '{…}')
  seen.add(obj)

  if (Array.isArray(obj)) {
    const items = obj.slice(0, MAX_ITEMS).map(v => toLogValue(v, depth + 1, seen))
    if (obj.length > MAX_ITEMS) items.push(special(`… ${obj.length - MAX_ITEMS} more`))
    return items
  }
  if (obj instanceof Map) {
    return special(`Map(${obj.size})`)
  }
  if (obj instanceof Set) {
    return special(`Set(${obj.size})`)
  }

  const out: Record<string, LogValue> = {}
  const keys = Object.keys(obj)
  for (const key of keys.slice(0, MAX_ITEMS)) {
    out[key] = toLogValue((obj as Record<string, unknown>)[key], depth + 1, seen)
  }
  return out
}
