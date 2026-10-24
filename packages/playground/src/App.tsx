import { useState, useEffect, useCallback, useMemo, useRef } from 'react'
import { AlignLeft } from 'lucide-react'
import { Header } from '@/components/Header'
import { Editor, type EditorHandle, type EditorDiagnostic } from '@/components/Editor'
import { OutputPanel } from '@/components/OutputPanel'
import { StatusBar } from '@/components/StatusBar'
import { LoadingOverlay } from '@/components/LoadingOverlay'
import { NativeSelect } from '@/components/ui/select'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Tooltip, TooltipTrigger, TooltipPopup } from '@/components/ui/tooltip'
import { ToastProvider, toastManager } from '@/components/ui/toast'
import { useCompiler } from '@/hooks/useCompiler'
import { formatSource } from '@/lib/compiler'
import { examples, exampleList, defaultCode } from '@/lib/examples'
import { SyntaxHelp } from '@/components/SyntaxHelp'
import { MobileTabs, type MobileTab } from '@/components/MobileTabs'
import { useMediaQuery } from '@/hooks/useMediaQuery'
import { cn } from '@/lib/utils'
import { buildShareUrl, readSharedCode, loadSavedCode, saveCode } from '@/lib/share'
import { lessons } from '@/lib/lessons'
import { LessonPanel } from '@/components/LessonPanel'
import type { LogValue } from '@/lib/runner'
import { useI18n } from '@/lib/i18n'

const EXAMPLE_TITLES: Record<string, string> = Object.fromEntries(
  exampleList.map(example => [example.id, `${example.id}.fratm`]),
)

const AUTO_RUN_KEY = 'fratm:autoRun'
const LESSON_CODE_KEY = 'fratm:scola:'
const LESSONS_DONE_KEY = 'fratm:scola:fatte'

/** Lesson from the URL hash (`#scola=3`, 1-based), or null outside the tour. */
function lessonFromHash(): number | null {
  const value = new URLSearchParams(window.location.hash.slice(1)).get('scola')
  if (value === null) return null
  const n = Number.parseInt(value, 10)
  return Number.isFinite(n) ? Math.min(Math.max(n, 1), lessons.length) - 1 : 0
}

function readStored(key: string): string | null {
  try {
    return localStorage.getItem(key)
  } catch {
    return null
  }
}

function writeStored(key: string, value: string) {
  try {
    localStorage.setItem(key, value)
  } catch {
    // Storage can be unavailable (private mode): progress lasts for this visit
  }
}

/** A console line as text, like the browser console prints it. */
function logLine(args: LogValue[]): string {
  return args
    .map(arg => (typeof arg === 'string' ? arg : arg && typeof arg === 'object' && '__fratm' in arg ? String(arg.text) : JSON.stringify(arg) ?? String(arg)))
    .join(' ')
}
const AUTO_RUN_DELAY_MS = 700

function readStoredFlag(key: string, fallback: boolean): boolean {
  try {
    const value = localStorage.getItem(key)
    return value === null ? fallback : value === 'true'
  } catch {
    return fallback
  }
}

function writeStoredFlag(key: string, value: boolean) {
  try {
    localStorage.setItem(key, String(value))
  } catch {
    // Storage can be unavailable (private mode): the toggle still works for this visit
  }
}

function App() {
  // A share link wins over the autosaved code, which wins over the default
  const [initial] = useState(() => {
    const lesson = lessonFromHash()
    if (lesson !== null) {
      const id = lessons[lesson].id
      return { code: readStored(LESSON_CODE_KEY + id) ?? lessons[lesson].starter, example: '', lesson }
    }
    const code = readSharedCode() ?? loadSavedCode() ?? defaultCode
    if (code === defaultCode) return { code, example: '', lesson: null }
    // Unedited example code restored from autosave keeps its name in the picker
    const match = Object.entries(examples).find(([, source]) => source === code)
    return { code, example: match ? match[0] : 'custom', lesson: null }
  })
  const [code, setCode] = useState(initial.code)
  const [selectedExample, setSelectedExample] = useState(initial.example)
  const compiler = useCompiler()
  const { t } = useI18n()
  const editorRef = useRef<EditorHandle>(null)
  const isRunning = compiler.status === 'running'
  const isMobile = useMediaQuery('(max-width: 767px)')
  const [mobileTab, setMobileTab] = useState<MobileTab>('code')
  const [cursorLine, setCursorLine] = useState(1)
  // Source of the last run, to tell whether the output is out of date
  const [lastRunCode, setLastRunCode] = useState<string | null>(null)
  const [autoRun, setAutoRun] = useState(() => readStoredFlag(AUTO_RUN_KEY, true))
  // 'A scola: index of the open lesson, null outside the tour
  const [lessonIndex, setLessonIndex] = useState<number | null>(initial.lesson)
  const lesson = lessonIndex === null ? null : lessons[lessonIndex]
  const [completed, setCompleted] = useState<Set<string>>(() => {
    try {
      return new Set(JSON.parse(readStored(LESSONS_DONE_KEY) ?? '[]') as string[])
    } catch {
      return new Set()
    }
  })

  // Autosave, debounced so typing doesn't write on every keystroke.
  // In the tour each lesson keeps its own code, so the playground code is untouched.
  const lessonId = lesson?.id
  useEffect(() => {
    const timer = setTimeout(() => {
      if (lessonId) writeStored(LESSON_CODE_KEY + lessonId, code)
      else saveCode(code)
    }, 500)
    return () => clearTimeout(timer)
  }, [code, lessonId])

  const { runCode, stop } = compiler
  const run = useCallback((source: string) => {
    runCode(source)
    setLastRunCode(source)
  }, [runCode])

  const handleRun = useCallback(() => {
    run(code)
    // On phones the output lives in its own tab: show it
    setMobileTab(tab => (tab === 'code' ? 'console' : tab))
  }, [code, run])

  // Auto-run: rerun shortly after typing stops (and once the compiler loads)
  useEffect(() => {
    if (!autoRun || compiler.isLoading || code === lastRunCode) return
    const timer = setTimeout(() => run(code), AUTO_RUN_DELAY_MS)
    return () => clearTimeout(timer)
  }, [autoRun, code, compiler.isLoading, lastRunCode, run])

  const handleAutoRunChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    setAutoRun(e.target.checked)
    writeStoredFlag(AUTO_RUN_KEY, e.target.checked)
  }, [])

  const isStale = lastRunCode !== null && code !== lastRunCode

  const handleCodeChange = useCallback((value: string) => {
    setCode(value)
    // Editing an example turns it into custom code
    setSelectedExample(prev => (
      prev && prev !== 'custom' && examples[prev] !== value ? 'custom' : prev
    ))
  }, [])

  const handleFormat = useCallback(() => {
    const formatted = formatSource(code)
    if (formatted !== null && formatted !== code) editorRef.current?.replaceAll(formatted)
  }, [code])

  const handleInsert = useCallback((text: string) => {
    setMobileTab('code')
    requestAnimationFrame(() => editorRef.current?.insertText(text))
  }, [])

  const handleErrorClick = useCallback((line: number, column?: number) => {
    setMobileTab('code')
    // Wait a frame so the editor is visible before scrolling to the line
    requestAnimationFrame(() => editorRef.current?.revealLine(line, column))
  }, [])

  const diagnostics = useMemo<EditorDiagnostic[]>(() => compiler.errors.flatMap(error => {
    if ((error.kind !== 'compile' && error.kind !== 'runtime') || error.line === undefined) return []
    const message = error.kind === 'runtime' ? `${error.name}: ${error.message}` : error.message
    return [{ line: error.line, column: error.column, message }]
  }), [compiler.errors])

  const openLesson = useCallback((index: number) => {
    const next = lessons[index]
    setLessonIndex(index)
    setSelectedExample('')
    setCode(readStored(LESSON_CODE_KEY + next.id) ?? next.starter)
    setMobileTab('code')
    history.replaceState(null, '', `#scola=${index + 1}`)
  }, [])

  const exitTour = useCallback(() => {
    setLessonIndex(null)
    setCode(loadSavedCode() ?? defaultCode)
    history.replaceState(null, '', window.location.pathname + window.location.search)
  }, [])

  const handleTour = useCallback(() => {
    if (lessonIndex !== null) {
      exitTour()
      return
    }
    // Resume at the first lesson not done yet
    const next = lessons.findIndex(l => !completed.has(l.id))
    openLesson(next === -1 ? 0 : next)
  }, [completed, exitTour, lessonIndex, openLesson])

  // The exercise is done when the last run of the current code printed the expected lines
  useEffect(() => {
    if (!lesson || compiler.status !== 'ready' || lastRunCode !== code || compiler.errors.length > 0) return
    const printed = compiler.consoleLogs.filter(log => log.type === 'log').map(log => logLine(log.args))
    if (printed.join('\n') !== lesson.expected.join('\n') || completed.has(lesson.id)) return
    setCompleted(prev => {
      const next = new Set(prev).add(lesson.id)
      writeStored(LESSONS_DONE_KEY, JSON.stringify([...next]))
      return next
    })
  }, [code, compiler.consoleLogs, compiler.errors.length, compiler.status, completed, lastRunCode, lesson])

  const handleShare = useCallback(() => {
    const url = buildShareUrl(code)

    navigator.clipboard.writeText(url).then(() => {
      toastManager.add({
        title: t('share.copied'),
        description: t('share.copiedDesc'),
        type: 'success',
      })
    }).catch(() => {
      toastManager.add({
        title: t('share.failed'),
        description: url,
        type: 'error',
      })
    })
  }, [code, t])

  const handleExampleChange = useCallback((e: React.ChangeEvent<HTMLSelectElement>) => {
    const name = e.target.value
    setSelectedExample(e.target.value)
    if (examples[name]) {
      setCode(examples[name])
    }
  }, [])

  return (
    <ToastProvider position="bottom-right">
      <div className="flex flex-col bg-background h-dvh overflow-hidden">
        <LoadingOverlay isVisible={compiler.isLoading} />

        {/* Sticky Header */}
        <Header
          onRun={handleRun}
          onStop={stop}
          onShare={handleShare}
          onTour={handleTour}
          tourActive={lesson !== null}
          isRunning={isRunning}
          disabled={compiler.isLoading}
        />

        {isMobile && (
          <MobileTabs
            value={mobileTab}
            onChange={setMobileTab}
            errorCount={compiler.errors.length}
            logCount={compiler.consoleLogs.length}
          />
        )}

        {/* Main content area - fills remaining space. Side by side on
            desktop; on phones one full-height tab at a time. */}
        <div className="flex md:grid flex-col flex-1 md:grid-cols-2 min-h-0 overflow-hidden">
          {/* Editor Panel (kept mounted when hidden so the editor keeps its state) */}
          <div
            id="panel-code"
            role={isMobile ? 'tabpanel' : undefined}
            className={cn(
              'flex flex-col flex-1 md:border-r border-border min-h-0 overflow-hidden',
              isMobile && mobileTab !== 'code' && 'hidden',
            )}
          >
            {lesson && lessonIndex !== null && (
              <LessonPanel
                lessons={lessons}
                index={lessonIndex}
                completed={completed}
                onSelect={openLesson}
                onExit={exitTour}
                onReset={() => editorRef.current?.replaceAll(lesson.starter)}
                onShowSolution={() => editorRef.current?.replaceAll(lesson.solution)}
              />
            )}
            {/* Sticky panel header */}
            <div className="flex justify-between items-center bg-card px-4 py-2.5 border-border border-b shrink-0">
              <span className="font-medium text-muted-foreground text-sm truncate">
                {lesson ? `scola/${lesson.id}.fratm` : EXAMPLE_TITLES[selectedExample] ?? 'main.fratm'}
                {selectedExample === 'custom' && <span className="ml-1.5 text-xs">{t('editor.edited')}</span>}
              </span>
              <Badge className="px-1 font-mono" variant="outline">{compiler.version}</Badge>
            </div>
            {/* Scrollable editor */}
            <div className="flex-1 bg-background min-h-0 overflow-auto">
              <Editor
                ref={editorRef}
                value={code}
                onChange={handleCodeChange}
                onRun={handleRun}
                onFormat={handleFormat}
                diagnostics={diagnostics}
                onCursorLineChange={setCursorLine}
              />
            </div>
            {/* Sticky examples bar */}
            <div className="flex items-center justify-between gap-2 bg-card px-4 py-2.5 border-border border-t shrink-0">
              <div className={cn('flex items-center gap-3 min-w-0', lesson && 'invisible')}>
                <label className="text-muted-foreground text-xs">{t('editor.examples')}</label>
                <NativeSelect
                  value={selectedExample}
                  onChange={handleExampleChange}
                  className="w-44 sm:w-52"
                >
                  <option value="">{t('editor.chooseExample')}</option>
                  {selectedExample === 'custom' && <option value="custom" disabled>{t('editor.custom')}</option>}
                  {exampleList.map(example => (
                    <option key={example.id} value={example.id}>{example.title}</option>
                  ))}
                </NativeSelect>
              </div>
              <div className="flex items-center gap-3 shrink-0">
                <label className="flex items-center gap-1.5 text-muted-foreground text-xs cursor-pointer select-none">
                  <input
                    type="checkbox"
                    checked={autoRun}
                    onChange={handleAutoRunChange}
                    className="accent-primary size-3.5"
                  />
                  {t('editor.autoRun')}
                </label>
                <Tooltip>
                  <TooltipTrigger
                    render={
                      <Button
                        variant="outline"
                        size="icon-sm"
                        aria-label={t('editor.format')}
                        onClick={handleFormat}
                        disabled={compiler.isLoading}
                      >
                        <AlignLeft className="size-4" />
                      </Button>
                    }
                  />
                  <TooltipPopup>{t('editor.formatTip')}</TooltipPopup>
                </Tooltip>
                <SyntaxHelp onInsert={handleInsert} />
              </div>
            </div>
          </div>

          {/* Output Panel */}
          <div
            id="panel-output"
            role={isMobile ? 'tabpanel' : undefined}
            className={cn(
              'flex flex-col flex-1 min-h-0 overflow-hidden',
              isMobile && mobileTab === 'code' && 'hidden',
            )}
          >
            {/* Sticky output header */}
            <div className="hidden md:flex items-center bg-card px-4 py-2.5 border-border border-b shrink-0">
              <span className="font-medium text-foreground text-sm">{t('output.title')}</span>
            </div>
            {/* Scrollable output content */}
            <div className="flex-1 bg-background min-h-0 overflow-auto">
              <OutputPanel
                jsOutput={compiler.jsOutput}
                consoleLogs={compiler.consoleLogs}
                errors={compiler.errors}
                notice={compiler.notice}
                isRunning={isRunning}
                onErrorClick={handleErrorClick}
                onClearConsole={compiler.clearConsole}
                jsLineMap={compiler.jsLineMap}
                cursorLine={cursorLine}
                show={isMobile && mobileTab !== 'code' ? mobileTab : 'all'}
                stale={isStale && !isRunning}
              />
            </div>
          </div>
        </div>

        {/* Sticky StatusBar */}
        <StatusBar
          status={isStale && !isRunning ? 'stale' : compiler.status}
          statusText={isStale && !isRunning
            ? t(autoRun ? 'status.staleAuto' : 'status.staleManual')
            : t(compiler.statusText.key, compiler.statusText.params)}
          compileTime={compiler.compileTime === null ? '' : t('status.compiledIn', { ms: compiler.compileTime.toFixed(1) })}
        />
      </div>
    </ToastProvider>
  )
}

export default App
