import { Button } from '@/components/ui/button'

interface HeaderProps {
  onRun: () => void
  onShare: () => void
  isCompiling: boolean
}

export function Header({ onRun, onShare, isCompiling }: HeaderProps) {
  return (
    <header className="flex flex-wrap justify-between items-center gap-4 bg-card px-4 py-3 border-border border-b">
      <div className="flex items-center gap-3">
        <h1 className="font-bold text-primary text-xl">fratmscript</h1>
      </div>
      <div className="flex items-center gap-1.5">
        <Button onClick={onRun} disabled={isCompiling}>
          Run
        </Button>
        <Button variant="outline" onClick={onShare}>
          Share
        </Button>
      </div>
    </header>
  )
}
