import { Button } from '@/components/ui/button'
import { Play, Share2 } from 'lucide-react'

interface HeaderProps {
  onRun: () => void
  onShare: () => void
  isCompiling: boolean
}

export function Header({ onRun, onShare, isCompiling }: HeaderProps) {
  return (
    <header className="bg-gradient-to-r from-primary to-secondary px-4 py-3 flex justify-between items-center flex-wrap gap-4">
      <div className="flex items-center gap-3">
        <h1 className="text-xl font-bold">FratmScript</h1>
        <p className="text-sm opacity-85">JavaScript, ma comme si deve</p>
      </div>
      <div className="flex items-center gap-3">
        <span className="text-xs text-foreground/70">
          <kbd className="bg-secondary/50 px-1.5 py-0.5 rounded text-xs">Ctrl</kbd>
          +
          <kbd className="bg-secondary/50 px-1.5 py-0.5 rounded text-xs">Enter</kbd>
          {' '}to run
        </span>
        <Button onClick={onRun} disabled={isCompiling}>
          <Play className="w-4 h-4" />
          Run
        </Button>
        <Button variant="secondary" onClick={onShare}>
          <Share2 className="w-4 h-4" />
          Share
        </Button>
      </div>
    </header>
  )
}
