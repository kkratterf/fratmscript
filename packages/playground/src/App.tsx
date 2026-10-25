import { useState, useEffect, useCallback, useMemo, useRef } from 'react'
import { AlignLeft } from 'lucide-react'
import { Header, EmbedHeader } from '@/components/Header'
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
import { buildShareUrl, buildEmbedSnippet, isEmbed, readSharedCode, loadSavedCode, saveCode } from '@/lib/share'
import { lessons } from '@/lib/lessons'
import { LessonPanel } from '@/components/LessonPanel'
import { GuidePage } from '@/components/GuidePage'
import { guide } from '@/lib/guide'
import type { LogValue } from '@/lib/runner'
import { useI18n, type MessageKey } from '@/lib/i18n'

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

/** Guide section from the URL hash (`#guida` or `#guida=cicli`), or null outside the guide. */
function guideFromHash(): number | null {
  const value = new URLSearchParams(window.location.hash.slice(1)).get('guida')
  if (value === null) return null
  return Math.max(guide.findIndex(section => section.id === value), 0)
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
  const [embed] = useState(isEmbed)
  // A share link wins over the autosaved code, which wins over the default
  const [initial] = useState(() => {
    // An embed shows only the code from its link
    if (embed) return { code: readSharedCode() ?? defaultCode, example: '', lesson: null, guide: null }
    const lesson = lessonFromHash()
    if (lesson !== null) {
      const id = lessons[lesson].id
      return { code: readStored(LESSON_CODE_KEY + id) ?? lessons[lesson].starter, example: '', lesson, guide: null }
    }
    const section = guideFromHash()
    const code = readSharedCode() ?? loadSavedCode() ?? defaultCode
    if (code === defaultCode) return { code, example: '', lesson: null, guide: section }
    // Unedited example code restored from autosave keeps its name in the picker
    const match = Object.entries(examples).find(([, source]) => source === code)
    return { code, example: match ? match[0] : 'custom', lesson: null, guide: section }
  })
  const [code, setCode] = useState(initial.code)
  const [selectedExample, setSelectedExample] = useState(initial.example)
  const compiler = useCompiler()
  const { lang, t } = useI18n()
  const editorRef = useRef<EditorHandle>(null)
  const isRunning = compiler.status === 'running'
  // A narrow embed stacks editor and output instead of using tabs
  const isMobile = useMediaQuery('(max-width: 767px)') && !embed
  const [mobileTab, setMobileTab] = useState<MobileTab>('code')
  const [cursorLine, setCursorLine] = useState(1)
  // Source of the last run, to tell whether the output is out of date
  const [lastRunCode, setLastRunCode] = useState<string | null>(null)
  const [autoRun, setAutoRun] = useState(() => readStoredFlag(AUTO_RUN_KEY, true))
  // 'A scola: index of the open lesson, null outside the tour
  const [lessonIndex, setLessonIndex] = useState<number | null>(initial.lesson)
  const lesson = lessonIndex === null ? null : lessons[lessonIndex]
  // 'A guida: index of the open guide section, null outside the guide
  const [guideIndex, setGuideIndex] = useState<number | null>(initial.guide)
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
  // An embed never autosaves, so it can't overwrite the visitor's own code.
  useEffect(() => {
    if (embed) return
    const timer = setTimeout(() => {
      if (lessonId) writeStored(LESSON_CODE_KEY + lessonId, code)
      else saveCode(code)
    }, 500)
    return () => clearTimeout(timer)
  }, [code, embed, lessonId])

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

  const handleHighlight = useCallback((range: { from: number; to: number } | null) => {
    editorRef.current?.highlightRange(range)
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
    setGuideIndex(null)
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
    if (!lesson || compiler.busy || compiler.status !== 'ready' || lastRunCode !== code || compiler.errors.length > 0) return
    const printed = compiler.consoleLogs.filter(log => log.type === 'log').map(log => logLine(log.args))
    if (printed.join('\n') !== lesson.expected.join('\n') || completed.has(lesson.id)) return
    setCompleted(prev => {
      const next = new Set(prev).add(lesson.id)
      writeStored(LESSONS_DONE_KEY, JSON.stringify([...next]))
      return next
    })
  }, [code, compiler.busy, compiler.consoleLogs, compiler.errors.length, compiler.status, completed, lastRunCode, lesson])

  const openGuide = useCallback((index: number) => {
    if (lessonIndex !== null) exitTour()
    setGuideIndex(index)
    history.replaceState(null, '', `#guida=${guide[index].id}`)
  }, [exitTour, lessonIndex])

  const exitGuide = useCallback(() => {
    setGuideIndex(null)
    history.replaceState(null, '', window.location.pathname + window.location.search)
  }, [])

  const handleGuide = useCallback(() => {
    if (guideIndex !== null) exitGuide()
    else openGuide(0)
  }, [exitGuide, guideIndex, openGuide])

  // A snippet opened from the guide becomes the playground code
  const handleOpenSnippet = useCallback((source: string) => {
    exitGuide()
    setCode(source)
    setSelectedExample('custom')
    setMobileTab('code')
  }, [exitGuide])

  const copy = useCallback((text: string, title: MessageKey, description: MessageKey) => {
    navigator.clipboard.writeText(text).then(() => {
      toastManager.add({ title: t(title), description: t(description), type: 'success' })
    }).catch(() => {
      toastManager.add({ title: t('share.failed'), description: text, type: 'error' })
    })
  }, [t])

  const handleShare = useCallback(() => {
    copy(buildShareUrl(code), 'share.copied', 'share.copiedDesc')
  }, [code, copy])

  const handleEmbed = useCallback(() => {
    copy(buildEmbedSnippet(code), 'share.embedCopied', 'share.embedCopiedDesc')
  }, [code, copy])

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
        {embed ? (
          <EmbedHeader
            onRun={handleRun}
            onStop={stop}
            playgroundUrl={buildShareUrl(code)}
            isRunning={isRunning}
            disabled={compiler.isLoading}
          />
        ) : (
          <Header
            onRun={handleRun}
            onStop={stop}
            onShare={handleShare}
            onEmbed={handleEmbed}
            onTour={handleTour}
            tourActive={lesson !== null}
            onGuide={handleGuide}
            guideActive={guideIndex !== null}
            isRunning={isRunning}
            disabled={compiler.isLoading}
          />
        )}

        {/* One main landmark for the guide, the mobile tabs and the panels */}
        <main className="flex flex-col flex-1 min-h-0 overflow-hidden">
          {guideIndex !== null && (
            <GuidePage index={guideIndex} onSelect={openGuide} ready={compiler.isWasmLoaded} onOpen={handleOpenSnippet} />
          )}

          {isMobile && guideIndex === null && (
            <MobileTabs
              value={mobileTab}
              onChange={setMobileTab}
              errorCount={compiler.errors.length}
              logCount={compiler.consoleLogs.length}
            />
          )}

          {/* Main content area - fills remaining space. Side by side on
              desktop; on phones one full-height tab at a time. */}
          <div className={cn('flex md:grid flex-col flex-1 md:grid-cols-2 min-h-0 overflow-hidden', guideIndex !== null && 'hidden md:hidden')}>
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
              <div className={cn('flex justify-between items-center bg-card px-4 py-2.5 border-border border-b shrink-0', embed && 'hidden')}>
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
              <div className={cn('flex flex-wrap items-center justify-between gap-x-2 gap-y-2 bg-card px-4 py-2.5 border-border border-t shrink-0', embed && 'hidden')}>
                <div className={cn('flex items-center gap-3 min-w-0', lesson && 'invisible max-md:hidden')}>
                  <label htmlFor="example-picker" className="text-muted-foreground text-xs">{t('editor.examples')}</label>
                  <NativeSelect
                    id="example-picker"
                    value={selectedExample}
                    onChange={handleExampleChange}
                    className="w-56"
                  >
                    <option value="">{t('editor.chooseExample')}</option>
                    {selectedExample === 'custom' && <option value="custom" disabled>{t('editor.custom')}</option>}
                    {exampleList.map(example => (
                      <option key={example.id} value={example.id}>{example.title[lang]}</option>
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
                  source={code}
                  onHighlight={handleHighlight}
                />
              </div>
            </div>
          </div>
        </main>

        {/* Sticky StatusBar */}
        {!embed && guideIndex === null && <StatusBar
          status={isStale && !isRunning ? 'stale' : compiler.status}
          statusText={isStale && !isRunning
            ? t(autoRun ? 'status.staleAuto' : 'status.staleManual')
            : t(compiler.statusText.key, compiler.statusText.params)}
          compileTime={compiler.compileTime === null ? '' : t('status.compiledIn', { ms: compiler.compileTime.toFixed(1) })}
        />}
      </div>
    </ToastProvider>
  )
}

export default App
