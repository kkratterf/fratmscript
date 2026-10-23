import { useState } from 'react'
import { ChevronRight } from 'lucide-react'
import { cn } from '@/lib/utils'
import type { LogValue } from '@/lib/runner'

type Special = { __fratm: 'special'; text: string }
type Composite = LogValue[] | { [key: string]: LogValue }

function isSpecial(value: unknown): value is Special {
  return typeof value === 'object' && value !== null && (value as { __fratm?: unknown }).__fratm === 'special'
}

function isComposite(value: LogValue): value is Composite {
  return typeof value === 'object' && value !== null && !isSpecial(value)
}

function entriesOf(value: Composite): [string, LogValue][] {
  return Array.isArray(value) ? value.map((v, i) => [String(i), v]) : Object.entries(value)
}

/** Primitive (or special) value, colored like DevTools. */
function Primitive({ value, quoted }: { value: LogValue; quoted: boolean }) {
  if (isSpecial(value)) return <span className="text-muted-foreground italic">{value.text}</span>
  if (value === null) return <span className="text-purple-600 dark:text-purple-400">null</span>
  if (value === undefined) return <span className="text-muted-foreground">undefined</span>
  if (typeof value === 'string') {
    return quoted ? <span className="text-emerald-700 dark:text-emerald-400">"{value}"</span> : <span>{value}</span>
  }
  if (typeof value === 'number') return <span className="text-amber-700 dark:text-amber-400">{String(value)}</span>
  return <span className="text-purple-600 dark:text-purple-400">{String(value)}</span>
}

/** One-line summary: `[1, 2, {…}]` / `{a: 1, b: […]}` */
function Preview({ value }: { value: Composite }) {
  const entries = entriesOf(value)
  const isArray = Array.isArray(value)
  const shown = entries.slice(0, 5)
  return (
    <span className="text-foreground">
      {isArray && <span className="text-muted-foreground">({entries.length}) </span>}
      {isArray ? '[' : '{'}
      {shown.map(([key, v], i) => (
        <span key={key}>
          {i > 0 && ', '}
          {!isArray && <span className="text-cyan-700 dark:text-cyan-400">{key}: </span>}
          {isComposite(v) ? (Array.isArray(v) ? '[…]' : '{…}') : <Primitive value={v} quoted />}
        </span>
      ))}
      {entries.length > shown.length && ', …'}
      {isArray ? ']' : '}'}
    </span>
  )
}

/**
 * A console value. Objects and arrays render as a collapsed summary that
 * expands into one row per property.
 */
export function ValueTree({ value, quoted = true, label }: { value: LogValue; quoted?: boolean; label?: string }) {
  const [open, setOpen] = useState(false)
  const keyLabel = label !== undefined && <span className="text-cyan-700 dark:text-cyan-400">{label}: </span>

  if (!isComposite(value)) {
    return <span>{keyLabel}<Primitive value={value} quoted={quoted} /></span>
  }

  const entries = entriesOf(value)
  return (
    <span className={cn('inline-block align-top', open && 'w-full')}>
      <button
        type="button"
        onClick={() => setOpen(o => !o)}
        aria-expanded={open}
        className="inline-flex items-start gap-0.5 hover:bg-accent/50 -ml-0.5 px-0.5 rounded text-left cursor-pointer"
      >
        <ChevronRight className={cn('mt-[3px] size-3 text-muted-foreground transition-transform shrink-0', open && 'rotate-90')} />
        <span>{keyLabel}<Preview value={value} /></span>
      </button>
      {open && (
        <span className="block ml-1.5 pl-3 border-border border-l">
          {entries.length === 0 && <span className="block text-muted-foreground italic">empty</span>}
          {entries.map(([key, v]) => (
            <span key={key} className="block">
              <ValueTree value={v} label={key} />
            </span>
          ))}
        </span>
      )}
    </span>
  )
}
