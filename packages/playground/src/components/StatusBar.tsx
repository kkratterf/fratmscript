import { cn } from '@/lib/utils'

interface StatusBarProps {
  status: 'loading' | 'ready' | 'compiling' | 'error'
  statusText: string
  compileTime: string
}

export function StatusBar({ status, statusText, compileTime }: StatusBarProps) {
  return (
    <div className="px-4 py-1.5 bg-black/30 text-xs text-muted-foreground flex justify-between">
      <div className="flex items-center gap-2">
        <div
          className={cn(
            "w-2 h-2 rounded-full",
            status === 'ready' && "bg-emerald-400",
            status === 'loading' && "bg-amber-400 animate-pulse",
            status === 'compiling' && "bg-amber-400 animate-pulse",
            status === 'error' && "bg-red-400"
          )}
        />
        <span>{statusText}</span>
      </div>
      {compileTime && <span>{compileTime}</span>}
    </div>
  )
}
