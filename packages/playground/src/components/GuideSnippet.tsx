import { useCallback, useEffect, useRef, useState } from 'react'
import { ChevronRight, CircleAlert, ExternalLink, Play, RotateCcw, Square, TriangleAlert } from 'lucide-react'
import { Editor, type EditorDiagnostic, type EditorHandle } from '@/components/Editor'
import { ErrorCard } from '@/components/OutputPanel'
import { ValueTree } from '@/components/ValueTree'
import { Button } from '@/components/ui/button'
import { compile } from '@/lib/compiler'
import { runInWorker, SYNC_TIME_LIMIT_MS, type LogEntry } from '@/lib/runner'
import { originalPosition, parseSourceMap } from '@/lib/sourcemap'
import type { PlaygroundError } from '@/hooks/useCompiler'
import { useI18n } from '@/lib/i18n'
import { cn } from '@/lib/utils'

interface RunResult {
  logs: LogEntry[]
  errors: PlaygroundError[]
  js: string
}

const EMPTY: RunResult = { logs: [], errors: [], js: '' }

/** Compiles and runs one snippet in its own worker, like the playground does. */
function useSnippetRunner() {
  const [result, setResult] = useState<RunResult>(EMPTY)
  const [running, setRunning] = useState(false)
  const stopRef = useRef<(() => void) | null>(null)

  useEffect(() => () => stopRef.current?.(), [])

  const run = useCallback((source: string) => {
    stopRef.current?.()
    const compiled = compile(source)
    if (!compiled.success || !compiled.code) {
      setRunning(false)
      setResult({
        logs: [],
        js: '',
        errors: [{
          kind: 'compile',
          message: compiled.error ?? '',
          line: compiled.line,
          column: compiled.column,
          suggestion: compiled.suggestion,
        }],
      })
      return
    }
    const sourceMap = compiled.sourceMap ? parseSourceMap(compiled.sourceMap) : null
    const logs: LogEntry[] = []
    const errors: PlaygroundError[] = []
    const js = compiled.code
    setRunning(true)
    const { stop } = runInWorker(js, {
      onLog: entry => logs.push(entry),
      onTruncated: () => {},
      onError: err => {
        const pos = err.line !== undefined && sourceMap ? originalPosition(sourceMap, err.line, err.column ?? 1) ?? {} : {}
        errors.push({ kind: 'runtime', name: err.name, message: err.message, ...pos })
      },
      onEnd: reason => {
        if (reason === 'sync-timeout') errors.push({ kind: 'timeout', seconds: SYNC_TIME_LIMIT_MS / 1000 })
        stopRef.current = null
        setRunning(false)
        // Snippets print a few lines: show them all at once when the run ends
        setResult({ logs, errors, js })
      },
    })
    stopRef.current = stop
  }, [])

  const stop = useCallback(() => stopRef.current?.(), [])

  return { result, running, run, stop }
}

interface GuideSnippetProps {
  initialCode: string
  /** The compiler is loaded: the snippet runs once as soon as it is */
  ready: boolean
  onOpen: (code: string) => void
}

export function GuideSnippet({ initialCode, ready, onOpen }: GuideSnippetProps) {
  const { t } = useI18n()
  const [code, setCode] = useState(initialCode)
  const [showJs, setShowJs] = useState(false)
  const editorRef = useRef<EditorHandle>(null)
  const { result, running, run, stop } = useSnippetRunner()

  // Show the output of the original code without a click
  useEffect(() => {
    if (ready) run(initialCode)
  }, [ready, initialCode, run])

  const handleRun = useCallback(() => run(code), [code, run])

  const diagnostics: EditorDiagnostic[] = result.errors.flatMap(error => (
    (error.kind === 'compile' || error.kind === 'runtime') && error.line !== undefined
      ? [{ line: error.line, column: error.column, message: error.kind === 'runtime' ? `${error.name}: ${error.message}` : error.message }]
      : []
  ))

  return (
    <figure className="bg-card border border-border rounded-lg overflow-hidden">
      <div className="max-h-96 overflow-auto">
        <Editor
          ref={editorRef}
          value={code}
          onChange={setCode}
          onRun={handleRun}
          diagnostics={diagnostics}
        />
      </div>

      <div className="flex flex-wrap justify-between items-center gap-2 bg-muted/30 px-3 py-2 border-border border-t">
        <div className="flex items-center gap-1">
          {running ? (
            <Button size="sm" variant="destructive" onClick={stop}>
              <Square className="size-3 fill-current" /> {t('guide.stop')}
            </Button>
          ) : (
            <Button size="sm" onClick={handleRun} disabled={!ready} aria-label={t('guide.runLabel')}>
              <Play className="size-3 fill-current" /> {t('guide.run')}
            </Button>
          )}
          <Button
            size="sm"
            variant="ghost"
            onClick={() => editorRef.current?.replaceAll(initialCode)}
            disabled={code === initialCode}
            title={t('guide.resetTip')}
          >
            <RotateCcw className="size-3.5" /> <span className="hidden sm:inline">{t('guide.reset')}</span>
          </Button>
        </div>
        <div className="flex items-center gap-1">
          <Button
            size="sm"
            variant={showJs ? 'secondary' : 'ghost'}
            aria-pressed={showJs}
            onClick={() => setShowJs(on => !on)}
            title={t('guide.showJsTip')}
          >
            {t('guide.showJs')}
          </Button>
          <Button size="sm" variant="outline" onClick={() => onOpen(code)} title={t('guide.openTip')}>
            <ExternalLink className="size-3.5" /> {t('guide.open')}
          </Button>
        </div>
      </div>

      <figcaption className="space-y-2 p-3 border-border border-t" aria-live="polite">
        {result.errors.map((error, i) => (
          <ErrorCard key={i} error={error} onErrorClick={(line, column) => editorRef.current?.revealLine(line, column)} />
        ))}
        <div className={cn('bg-background border border-border rounded-md font-mono text-sm', running && 'opacity-60')}>
          <div className="px-3 pt-1.5 font-sans text-muted-foreground text-xs uppercase tracking-wider">{t('guide.console')}</div>
          {result.logs.length > 0 ? (
            result.logs.map((log, i) => (
              <div key={i} className={cn('flex items-start gap-2 px-3 py-1', log.type === 'warn' && 'text-warning-foreground', log.type === 'error' && 'text-destructive-foreground')}>
                <span className="mt-0.5 shrink-0">
                  {log.type === 'warn' && <TriangleAlert className="size-3.5" />}
                  {log.type === 'error' && <CircleAlert className="size-3.5" />}
                  {log.type === 'log' && <ChevronRight className="size-3.5 text-muted-foreground" />}
                </span>
                <span className="flex-1 break-all whitespace-pre-wrap">
                  {log.args.map((arg, j) => (
                    <span key={j}>{j > 0 && ' '}<ValueTree value={arg} quoted={false} /></span>
                  ))}
                </span>
              </div>
            ))
          ) : (
            <div className="px-3 py-1.5 text-muted-foreground/60">{t('guide.empty')}</div>
          )}
        </div>
        {showJs && result.js && (
          <pre className="bg-background p-3 border border-border rounded-md font-mono text-foreground/90 text-sm break-words whitespace-pre-wrap">
            {result.js}
          </pre>
        )}
      </figcaption>
    </figure>
  )
}
