import { useState, useEffect, useCallback } from 'react'
import { Header } from '@/components/Header'
import { Editor } from '@/components/Editor'
import { OutputPanel } from '@/components/OutputPanel'
import { StatusBar } from '@/components/StatusBar'
import { LoadingOverlay } from '@/components/LoadingOverlay'
import { NativeSelect } from '@/components/ui/select'
import { Badge } from '@/components/ui/badge'
import { ToastProvider, toastManager } from '@/components/ui/toast'
import { useCompiler } from '@/hooks/useCompiler'
import { examples, defaultCode } from '@/lib/examples'
import { SyntaxHelp } from '@/components/SyntaxHelp'

function App() {
  const [code, setCode] = useState(defaultCode)
  const [selectedExample, setSelectedExample] = useState('')
  const compiler = useCompiler()

  // Load code from URL on mount
  useEffect(() => {
    const params = new URLSearchParams(window.location.search)
    const urlCode = params.get('code')
    if (urlCode) {
      try {
        setCode(decodeURIComponent(atob(urlCode)))
      } catch (e) {
        console.error('Failed to load code from URL:', e)
      }
    }
  }, [])

  const handleRun = useCallback(() => {
    compiler.runCode(code)
  }, [code, compiler])

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
      <div className="flex flex-col bg-background h-screen overflow-hidden">
        <LoadingOverlay isVisible={compiler.isLoading} />

        {/* Sticky Header */}
        <Header
          onRun={handleRun}
          onShare={handleShare}
          isCompiling={compiler.status === 'compiling'}
        />

        {/* Main content area - fills remaining space */}
        <div className="flex-1 grid grid-cols-1 md:grid-cols-2 min-h-0 overflow-hidden">
          {/* Editor Panel */}
          <div className="flex flex-col border-border border-r min-h-0 overflow-hidden">
            {/* Sticky panel header */}
            <div className="flex justify-between items-center bg-card px-4 py-2.5 border-border border-b shrink-0">
              <span className="font-medium text-muted-foreground text-sm">JavaScript ma comme si deve</span>
              <Badge className="px-1 font-mono" variant="outline">{compiler.version}</Badge>
            </div>
            {/* Scrollable editor */}
            <div className="flex-1 bg-background min-h-0 overflow-auto">
              <Editor
                value={code}
                onChange={setCode}
                onRun={handleRun}
              />
            </div>
            {/* Sticky examples bar */}
            <div className="flex items-center justify-between bg-card px-4 py-2.5 border-border border-t shrink-0">
              <div className="flex items-center gap-3">
                <label className="text-muted-foreground text-xs">Examples:</label>
                <NativeSelect
                  value={selectedExample}
                  onChange={handleExampleChange}
                  className="w-52"
                >
                  <option value="">-- Choose an example --</option>
                  <option value="hello">Hello World</option>
                  <option value="fibonacci">Fibonacci</option>
                  <option value="classe">Pizzaiolo Class</option>
                  <option value="async">Async/Await</option>
                  <option value="operatori">Logical Operators</option>
                  <option value="array">Arrays and Objects</option>
                </NativeSelect>
              </div>
              <SyntaxHelp />
            </div>
          </div>

          {/* Output Panel */}
          <div className="flex flex-col min-h-0 overflow-hidden">
            {/* Sticky output header */}
            <div className="flex items-center bg-card px-4 py-2.5 border-border border-b shrink-0">
              <span className="font-medium text-foreground text-sm">Output</span>
            </div>
            {/* Scrollable output content */}
            <div className="flex-1 bg-background min-h-0 overflow-auto">
              <OutputPanel
                jsOutput={compiler.jsOutput}
                consoleLogs={compiler.consoleLogs}
                error={compiler.error}
              />
            </div>
          </div>
        </div>

        {/* Sticky StatusBar */}
        <StatusBar
          status={compiler.status}
          statusText={compiler.statusText}
          compileTime={compiler.compileTime}
        />
      </div>
    </ToastProvider>
  )
}

export default App
