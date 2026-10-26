import { MadeBy } from '@/components/MadeBy'
import { cn } from '@/lib/utils'

interface StatusBarProps {
  status: 'loading' | 'ready' | 'compiling' | 'running' | 'error' | 'stale'
  statusText: string
  compileTime: string
}

export function StatusBar({ status, statusText, compileTime }: StatusBarProps) {
  return (
    <footer className="flex justify-between items-center gap-3 bg-card px-4 py-0.5 border-border border-t text-muted-foreground text-xs">
      <div className="flex items-center gap-2 min-w-0">
        <div
          className={cn(
            "w-2 h-2 rounded-full shrink-0",
            status === 'ready' && "bg-success",
            status === 'loading' && "bg-warning animate-pulse",
            (status === 'compiling' || status === 'running') && "bg-warning animate-pulse",
            status === 'error' && "bg-destructive",
            status === 'stale' && "bg-warning"
          )}
        />
        <span className="truncate">{statusText}</span>
      </div>
      <div className="flex items-center gap-3 shrink-0">
        {compileTime && (
          <>
            <span className="max-sm:hidden">{compileTime}</span>
            <div role="separator" aria-orientation="vertical" className="max-sm:hidden bg-border w-px h-3.5" />
          </>
        )}
        <MadeBy />
      </div>
    </footer>
  )
}
