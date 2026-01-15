import { cn } from '@/lib/utils'
import type { LogEntry } from '@/lib/compiler'
import type { CompilationOutput } from '@/lib/compiler'

interface OutputPanelProps {
  jsOutput: string
  consoleLogs: LogEntry[]
  error: CompilationOutput | null
}

function escapeHtml(text: string): string {
  const div = document.createElement('div')
  div.textContent = text
  return div.innerHTML
}

function formatArg(arg: unknown): string {
  if (typeof arg === 'object') {
    return JSON.stringify(arg, null, 2)
  }
  return String(arg)
}

export function OutputPanel({ jsOutput, consoleLogs, error }: OutputPanelProps) {
  return (
    <div className="flex flex-col h-full overflow-auto">
      {/* JavaScript Output */}
      <section className="p-4 border-b border-border">
        <h3 className="text-xs text-muted-foreground uppercase tracking-wider mb-2">
          Generated JavaScript
        </h3>
        <pre className="font-mono text-sm leading-relaxed text-emerald-400 whitespace-pre-wrap break-words">
          {jsOutput}
        </pre>
      </section>

      {/* Console Output */}
      <section className="p-4 border-b border-border">
        <h3 className="text-xs text-muted-foreground uppercase tracking-wider mb-2">
          Console
        </h3>
        <pre className="font-mono text-sm leading-relaxed whitespace-pre-wrap break-words">
          {consoleLogs.length > 0 ? (
            consoleLogs.map((log, i) => {
              const text = log.args.map(formatArg).join(' ')
              return (
                <span
                  key={i}
                  className={cn(
                    log.type === 'warn' && 'text-amber-400',
                    log.type === 'error' && 'text-red-400',
                    log.type === 'log' && 'text-amber-300'
                  )}
                >
                  {escapeHtml(text)}
                  {'\n'}
                </span>
              )
            })
          ) : (
            <span className="text-muted-foreground/50">// No output</span>
          )}
        </pre>
      </section>

      {/* Error Output */}
      {error && !error.success && (
        <section className="p-4">
          <h3 className="text-xs text-muted-foreground uppercase tracking-wider mb-2">
            Error
          </h3>
          <div className="bg-red-500/10 border border-red-500 rounded-md p-3">
            <div className="font-semibold text-red-400 mb-1">
              {error.error}
            </div>
            {error.line && (
              <div className="text-sm text-muted-foreground">
                Line {error.line}, Column {error.column || 1}
              </div>
            )}
            {error.suggestion && (
              <div className="mt-2 pt-2 border-t border-white/10 text-amber-400 text-sm">
                {error.suggestion}
              </div>
            )}
          </div>
        </section>
      )}
    </div>
  )
}
