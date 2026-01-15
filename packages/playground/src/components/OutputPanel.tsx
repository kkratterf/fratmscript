import { cn } from '@/lib/utils'
import type { LogEntry } from '@/lib/compiler'
import type { CompilationOutput } from '@/lib/compiler'
import { CircleAlert, TriangleAlert, ChevronRight } from 'lucide-react'

interface OutputPanelProps {
  jsOutput: string
  consoleLogs: LogEntry[]
  error: CompilationOutput | null
}

function formatValue(arg: unknown): React.ReactNode {
  if (arg === null) return <span className="text-purple-400">null</span>
  if (arg === undefined) return <span className="text-muted-foreground">undefined</span>
  if (typeof arg === 'string') return <span className="text-green-400">"{arg}"</span>
  if (typeof arg === 'number') return <span className="text-amber-400">{arg}</span>
  if (typeof arg === 'boolean') return <span className="text-purple-400">{String(arg)}</span>
  if (Array.isArray(arg)) {
    return (
      <span className="text-foreground">
        {'['}{arg.map((item, i) => (
          <span key={i}>
            {i > 0 && ', '}
            {formatValue(item)}
          </span>
        ))}{']'}
      </span>
    )
  }
  if (typeof arg === 'object') {
    try {
      const entries = Object.entries(arg as Record<string, unknown>)
      return (
        <span className="text-foreground">
          {'{ '}
          {entries.map(([key, val], i) => (
            <span key={key}>
              {i > 0 && ', '}
              <span className="text-cyan-400">{key}</span>: {formatValue(val)}
            </span>
          ))}
          {' }'}
        </span>
      )
    } catch {
      return <span className="text-foreground">{String(arg)}</span>
    }
  }
  return <span className="text-foreground">{String(arg)}</span>
}

export function OutputPanel({ jsOutput, consoleLogs, error }: OutputPanelProps) {
  return (
    <div className="flex flex-col h-full">
      {/* JavaScript Output */}
      <section className="p-4 border-border border-b">
        <h3 className="flex items-center gap-2 mb-3 text-muted-foreground text-xs uppercase tracking-wider">
          <span className="bg-success rounded-full w-2 h-2" />
          Generated JavaScript
        </h3>
        <pre className="bg-card/50 p-3 border border-border rounded-md font-mono text-foreground/90 text-sm break-words leading-relaxed whitespace-pre-wrap">
          {jsOutput || <span className="text-muted-foreground/50">// Run code to see output</span>}
        </pre>
      </section>

      {/* Console Output - Chrome DevTools style */}
      <section className="flex-1 p-4 border-border">
        <h3 className="flex items-center gap-2 mb-3 text-muted-foreground text-xs uppercase tracking-wider">
          <span className="bg-info rounded-full w-2 h-2" />
          Console
        </h3>
        <div className="bg-card/50 border border-border rounded-md overflow-hidden font-mono text-sm">
          {consoleLogs.length > 0 ? (
            consoleLogs.map((log, i) => (
              <div
                key={i}
                className={cn(
                  'flex items-start gap-2 px-3 py-1.5 border-border/50 last:border-0 border-b',
                  log.type === 'warn' && 'bg-warning/5',
                  log.type === 'error' && 'bg-destructive/5'
                )}
              >
                {/* Log type icon */}
                <span className="mt-0.5 shrink-0">
                  {log.type === 'warn' && <TriangleAlert className="w-3.5 h-3.5 text-warning" />}
                  {log.type === 'error' && <CircleAlert className="w-3.5 h-3.5 text-destructive" />}
                  {log.type === 'log' && <ChevronRight className="w-3.5 h-3.5 text-muted-foreground" />}
                </span>
                {/* Log content */}
                <span className={cn(
                  'flex-1 break-all',
                  log.type === 'warn' && 'text-warning',
                  log.type === 'error' && 'text-destructive',
                  log.type === 'log' && 'text-foreground'
                )}>
                  {log.args.map((arg, j) => (
                    <span key={j}>
                      {j > 0 && ' '}
                      {typeof arg === 'string' ? arg : formatValue(arg)}
                    </span>
                  ))}
                </span>
              </div>
            ))
          ) : (
            <div className="px-3 py-4 text-muted-foreground/50 text-center">
              No console output
            </div>
          )}
        </div>
      </section>

      {/* Error Output */}
      {error && !error.success && (
        <section className="p-4 border-border border-t">
          <h3 className="flex items-center gap-2 mb-3 text-muted-foreground text-xs uppercase tracking-wider">
            <span className="bg-destructive rounded-full w-2 h-2 animate-pulse" />
            Compilation Error
          </h3>
          <div className="bg-destructive/10 border border-destructive/50 rounded-md overflow-hidden">
            <div className="flex items-start gap-3 px-4 py-3">
              <CircleAlert className="mt-0.5 w-5 h-5 text-destructive shrink-0" />
              <div className="flex-1">
                <div className="font-semibold text-destructive">
                  {error.error}
                </div>
                {error.line && (
                  <div className="mt-1 text-muted-foreground text-sm">
                    at line {error.line}, column {error.column || 1}
                  </div>
                )}
              </div>
            </div>
            {error.suggestion && (
              <div className="bg-warning/5 px-4 py-3 border-destructive/20 border-t">
                <div className="flex items-start gap-2 text-sm">
                  <TriangleAlert className="mt-0.5 w-4 h-4 text-warning shrink-0" />
                  <span className="text-warning">{error.suggestion}</span>
                </div>
              </div>
            )}
          </div>
        </section>
      )}
    </div>
  )
}
