import { useState, useEffect, useCallback, useMemo, useRef } from 'react'
import { Header } from '@/components/Header'
import { Editor, type EditorHandle, type EditorDiagnostic } from '@/components/Editor'
import { OutputPanel } from '@/components/OutputPanel'
import { StatusBar } from '@/components/StatusBar'
import { LoadingOverlay } from '@/components/LoadingOverlay'
import { NativeSelect } from '@/components/ui/select'
import { Badge } from '@/components/ui/badge'
import { ToastProvider, toastManager } from '@/components/ui/toast'
import { useCompiler } from '@/hooks/useCompiler'
import { examples, defaultCode } from '@/lib/examples'
import { SyntaxHelp } from '@/components/SyntaxHelp'
import { MobileTabs, type MobileTab } from '@/components/MobileTabs'
import { useMediaQuery } from '@/hooks/useMediaQuery'
import { cn } from '@/lib/utils'

const AUTO_RUN_KEY = 'fratm:autoRun'
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
  const [code, setCode] = useState(defaultCode)
  const [selectedExample, setSelectedExample] = useState('')
  const compiler = useCompiler()
  const editorRef = useRef<EditorHandle>(null)
  const isRunning = compiler.status === 'running'
  const isMobile = useMediaQuery('(max-width: 767px)')
  const [mobileTab, setMobileTab] = useState<MobileTab>('code')
  // Source of the last run, to tell whether the output is out of date
  const [lastRunCode, setLastRunCode] = useState<string | null>(null)
  const [autoRun, setAutoRun] = useState(() => readStoredFlag(AUTO_RUN_KEY, true))

  // Load code from URL on mount
  useEffect(() => {
    const params = new URLSearchParams(window.location.search)
    const urlCode = params.get('code')
    if (urlCode) {
      try {
        setCode(decodeURIComponent(atob(urlCode)))
        setSelectedExample('custom')
      } catch (e) {
        console.error('Failed to load code from URL:', e)
      }
    }
  }, [])

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
      prev && prev !== 'custom' && examples[prev as keyof typeof examples] !== value ? 'custom' : prev
    ))
  }, [])

  const handleErrorClick = useCallback((line: number, column?: number) => {
    setMobileTab('code')
    // Wait a frame so the editor is visible before scrolling to the line
    requestAnimationFrame(() => editorRef.current?.revealLine(line, column))
  }, [])

  const diagnostics = useMemo<EditorDiagnostic[]>(() => compiler.errors.flatMap(error => {
    if (error.kind === 'timeout' || error.line === undefined) return []
    const message = error.kind === 'runtime' ? `${error.name}: ${error.message}` : error.message
    return [{ line: error.line, column: error.column, message }]
  }), [compiler.errors])

  const handleShare = useCallback(() => {
    const encoded = btoa(encodeURIComponent(code))
    const url = window.location.origin + window.location.pathname + '?code=' + encoded

    navigator.clipboard.writeText(url).then(() => {
      toastManager.add({
        title: 'Link copied!',
        description: 'Share this link with others to share your code.',
        type: 'success',
      })
    }).catch(() => {
      toastManager.add({
        title: 'Copy failed',
        description: url,
        type: 'error',
      })
    })
  }, [code])

  const handleExampleChange = useCallback((e: React.ChangeEvent<HTMLSelectElement>) => {
    const name = e.target.value as keyof typeof examples
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
            {/* Sticky panel header */}
            <div className="flex justify-between items-center bg-card px-4 py-2.5 border-border border-b shrink-0">
              <span className="font-medium text-muted-foreground text-sm">JavaScript ma comme si deve</span>
              <Badge className="px-1 font-mono" variant="outline">{compiler.version}</Badge>
            </div>
            {/* Scrollable editor */}
            <div className="flex-1 bg-background min-h-0 overflow-auto">
              <Editor
                ref={editorRef}
                value={code}
                onChange={handleCodeChange}
                onRun={handleRun}
                diagnostics={diagnostics}
              />
            </div>
            {/* Sticky examples bar */}
            <div className="flex items-center justify-between gap-2 bg-card px-4 py-2.5 border-border border-t shrink-0">
              <div className="flex items-center gap-3 min-w-0">
                <label className="text-muted-foreground text-xs">Examples:</label>
                <NativeSelect
                  value={selectedExample}
                  onChange={handleExampleChange}
                  className="w-44 sm:w-52"
                >
                  <option value="">-- Choose an example --</option>
                  {selectedExample === 'custom' && <option value="custom" disabled>Custom code</option>}
                  <option value="hello">Hello World</option>
                  <option value="fibonacci">Fibonacci</option>
                  <option value="classe">Pizzaiolo Class</option>
                  <option value="oop_avanzato">Advanced OOP</option>
                  <option value="async">Async/Await</option>
                  <option value="operatori">Logical Operators</option>
                  <option value="array">Arrays and Objects</option>
                  <option value="trycatch">Try/Catch</option>
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
                  Auto-run
                </label>
                <SyntaxHelp />
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
              <span className="font-medium text-foreground text-sm">Output</span>
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
            ? (autoRun ? 'Code changed, updating…' : 'Code changed: output is out of date, press Run')
            : compiler.statusText}
          compileTime={compiler.compileTime}
        />
      </div>
    </ToastProvider>
  )
}

export default App
