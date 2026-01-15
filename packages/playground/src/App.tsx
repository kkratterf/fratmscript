import { useState, useEffect, useCallback } from 'react'
import { Header } from '@/components/Header'
import { Editor } from '@/components/Editor'
import { OutputPanel } from '@/components/OutputPanel'
import { StatusBar } from '@/components/StatusBar'
import { LoadingOverlay } from '@/components/LoadingOverlay'
import { Select } from '@/components/ui/select'
import { Badge } from '@/components/ui/badge'
import { useCompiler } from '@/hooks/useCompiler'
import { examples, defaultCode } from '@/lib/examples'

function App() {
  const [code, setCode] = useState(defaultCode)
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
      alert('Link copied to clipboard!')
    }).catch(() => {
      prompt('Copy this link:', url)
    })
  }, [code])

  const handleExampleChange = useCallback((e: React.ChangeEvent<HTMLSelectElement>) => {
    const name = e.target.value as keyof typeof examples
    if (examples[name]) {
      setCode(examples[name])
      e.target.value = ''
    }
  }, [])

  return (
    <div className="min-h-screen flex flex-col">
      <LoadingOverlay isVisible={compiler.isLoading} />

      <Header
        onRun={handleRun}
        onShare={handleShare}
        isCompiling={compiler.status === 'compiling'}
      />

      <div className="flex-1 grid grid-cols-1 md:grid-cols-2 min-h-0">
        {/* Editor Panel */}
        <div className="flex flex-col border-r border-border min-h-0">
          <div className="px-4 py-2 bg-secondary flex justify-between items-center border-b border-white/10">
            <span className="font-semibold text-sm">FratmScript</span>
            <Badge variant="secondary">v{compiler.version}</Badge>
          </div>
          <div className="flex-1 min-h-0 bg-card">
            <Editor
              value={code}
              onChange={setCode}
              onRun={handleRun}
            />
          </div>
          <div className="px-4 py-2 bg-black/20 flex items-center gap-3">
            <label className="text-xs text-muted-foreground">Examples:</label>
            <Select onChange={handleExampleChange} className="w-48">
              <option value="">-- Choose an example --</option>
              <option value="hello">Hello World</option>
              <option value="fibonacci">Fibonacci</option>
              <option value="classe">Pizzaiolo Class</option>
              <option value="async">Async/Await</option>
              <option value="operatori">Logical Operators</option>
              <option value="array">Arrays and Objects</option>
            </Select>
          </div>
        </div>

        {/* Output Panel */}
        <div className="flex flex-col min-h-0">
          <div className="px-4 py-2 bg-secondary flex items-center border-b border-white/10">
            <span className="font-semibold text-sm">Output</span>
          </div>
          <div className="flex-1 min-h-0 bg-card overflow-auto">
            <OutputPanel
              jsOutput={compiler.jsOutput}
              consoleLogs={compiler.consoleLogs}
              error={compiler.error}
            />
          </div>
        </div>
      </div>

      <StatusBar
        status={compiler.status}
        statusText={compiler.statusText}
        compileTime={compiler.compileTime}
      />
    </div>
  )
}

export default App
