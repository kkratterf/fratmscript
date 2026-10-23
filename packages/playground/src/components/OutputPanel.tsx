import { cn } from '@/lib/utils'
import type { LogEntry } from '@/lib/runner'
import type { PlaygroundError } from '@/hooks/useCompiler'
import { CircleAlert, TriangleAlert, ChevronRight, Timer, Ban, Copy, Check } from 'lucide-react'
import { useState } from 'react'
import { ValueTree } from '@/components/ValueTree'

interface OutputPanelProps {
  jsOutput: string
  consoleLogs: LogEntry[]
  errors: PlaygroundError[]
  notice: string
  isRunning: boolean
  onErrorClick: (line: number, column?: number) => void
  onClearConsole: () => void
  /** Which sections to render: everything (desktop) or one mobile tab */
  show?: 'all' | 'console' | 'js'
  /** The code changed since this output was produced */
  stale?: boolean
}

const ERROR_TITLES: Record<PlaygroundError['kind'], string> = {
  compile: 'Compilation error',
  runtime: 'Runtime error',
  timeout: 'Execution timed out',
}

function SectionAction({ onClick, disabled, icon, label }: {
  onClick: () => void
  disabled?: boolean
  icon: React.ReactNode
  label: string
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      disabled={disabled}
      className="flex items-center gap-1 hover:bg-accent disabled:opacity-40 px-1.5 py-0.5 rounded text-muted-foreground hover:text-foreground text-xs transition-colors disabled:pointer-events-none"
    >
      {icon}
      {label}
    </button>
  )
}

function CopyButton({ text }: { text: string }) {
  const [copied, setCopied] = useState(false)
  const copy = () => {
    navigator.clipboard.writeText(text).then(() => {
      setCopied(true)
      setTimeout(() => setCopied(false), 1500)
    }).catch(() => {})
  }
  return (
    <SectionAction
      onClick={copy}
      disabled={!text || text.startsWith('// ')}
      icon={copied ? <Check className="size-3" /> : <Copy className="size-3" />}
      label={copied ? 'Copied' : 'Copy JS'}
    />
  )
}

function ErrorCard({ error, onErrorClick }: { error: PlaygroundError; onErrorClick: OutputPanelProps['onErrorClick'] }) {
  const line = error.kind !== 'timeout' ? error.line : undefined
  const column = error.kind !== 'timeout' ? error.column : undefined
  const message = error.kind === 'runtime' ? `${error.name}: ${error.message}` : error.message
  const Icon = error.kind === 'timeout' ? Timer : CircleAlert

  return (
    <div className="bg-destructive/10 border border-destructive/50 rounded-md overflow-hidden">
      <div className="flex items-start gap-3 px-4 py-3">
        <Icon className="mt-0.5 w-5 h-5 text-destructive shrink-0" />
        <div className="flex-1 min-w-0">
          <div className="mb-1 font-medium text-destructive/80 text-xs uppercase tracking-wider">
            {ERROR_TITLES[error.kind]}
          </div>
          <div className="font-semibold text-destructive break-words">
            {message}
          </div>
          {line !== undefined && (
            <button
              type="button"
              onClick={() => onErrorClick(line, column)}
              className="mt-1 text-muted-foreground hover:text-foreground text-sm underline underline-offset-2 decoration-dotted cursor-pointer"
            >
              Line {line}{column !== undefined && `, column ${column}`}
            </button>
          )}
        </div>
      </div>
      {error.kind === 'compile' && error.suggestion && (
        <div className="bg-warning/5 px-4 py-3 border-destructive/20 border-t">
          <div className="flex items-start gap-2 text-sm">
            <TriangleAlert className="mt-0.5 w-4 h-4 text-warning shrink-0" />
            <span className="text-warning">{error.suggestion}</span>
          </div>
        </div>
      )}
    </div>
  )
}

export function OutputPanel({ jsOutput, consoleLogs, errors, notice, isRunning, onErrorClick, onClearConsole, show = 'all', stale = false }: OutputPanelProps) {
  const showConsole = show !== 'js'
  const showJs = show !== 'console'
  return (
    <div
      className={cn('flex flex-col h-full transition-opacity duration-200', stale && 'opacity-60')}
      title={stale ? 'The code changed since this output was produced' : undefined}
    >
      {/* Errors first: they are what you need to see */}
      {showConsole && errors.length > 0 && (
        <section className="flex flex-col gap-2 p-4 border-border border-b" aria-live="polite">
          {errors.map((error, i) => (
            <ErrorCard key={i} error={error} onErrorClick={onErrorClick} />
          ))}
        </section>
      )}

      {/* Console Output - Chrome DevTools style */}
      {showConsole && (
        <section className={cn('p-4 border-border', showJs && 'border-b')}>
          <div className="flex justify-between items-center mb-3">
            <h3 className="flex items-center gap-2 text-muted-foreground text-xs uppercase tracking-wider">
              <span className={cn('bg-info rounded-full w-2 h-2', isRunning && 'animate-pulse')} />
              Console
              {isRunning && <span className="normal-case tracking-normal">running…</span>}
            </h3>
            <SectionAction
              onClick={onClearConsole}
              disabled={consoleLogs.length === 0 && !notice}
              icon={<Ban className="size-3" />}
              label="Clear"
            />
          </div>
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
                    'flex-1 break-all whitespace-pre-wrap',
                    log.type === 'warn' && 'text-warning',
                    log.type === 'error' && 'text-destructive',
                    log.type === 'log' && 'text-foreground'
                  )}>
                    {log.args.map((arg, j) => (
                      <span key={j}>
                        {j > 0 && ' '}
                        {/* Top-level strings print raw, like console.log does */}
                      <ValueTree value={arg} quoted={false} />
                      </span>
                    ))}
                  </span>
                </div>
              ))
            ) : (
              <div className="px-3 py-4 text-muted-foreground/50 text-center">
                {isRunning ? 'Waiting for output…' : 'No console output'}
              </div>
            )}
            {notice && (
              <div className="px-3 py-1.5 border-border/50 border-t text-muted-foreground text-xs italic">
                {notice}
              </div>
            )}
          </div>
        </section>
      )}

      {/* JavaScript Output */}
      {showJs && (
        <section className="flex-1 p-4">
          <div className="flex justify-between items-center mb-3">
            <h3 className="flex items-center gap-2 text-muted-foreground text-xs uppercase tracking-wider">
              <span className="bg-success rounded-full w-2 h-2" />
              Generated JavaScript
            </h3>
            <CopyButton text={jsOutput} />
          </div>
          <pre className="bg-card/50 p-3 border border-border rounded-md font-mono text-foreground/90 text-sm break-words leading-relaxed whitespace-pre-wrap">
            {jsOutput || <span className="text-muted-foreground/50">// Run code to see output</span>}
          </pre>
        </section>
      )}
    </div>
  )
}
