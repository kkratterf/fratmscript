import { cn } from '@/lib/utils'

export type MobileTab = 'code' | 'console' | 'js'

interface MobileTabsProps {
  value: MobileTab
  onChange: (tab: MobileTab) => void
  errorCount: number
  logCount: number
}

const TABS: { id: MobileTab; label: string; panel: string }[] = [
  { id: 'code', label: 'Code', panel: 'panel-code' },
  { id: 'console', label: 'Console', panel: 'panel-output' },
  { id: 'js', label: 'JS', panel: 'panel-output' },
]

export function MobileTabs({ value, onChange, errorCount, logCount }: MobileTabsProps) {
  return (
    <div role="tablist" aria-label="Playground panels" className="flex bg-card border-border border-b shrink-0">
      {TABS.map(tab => {
        const selected = tab.id === value
        return (
          <button
            key={tab.id}
            type="button"
            role="tab"
            aria-selected={selected}
            aria-controls={tab.panel}
            onClick={() => onChange(tab.id)}
            className={cn(
              'flex flex-1 justify-center items-center gap-1.5 py-2.5 border-b-2 font-medium text-sm transition-colors',
              selected
                ? 'border-primary text-foreground'
                : 'border-transparent text-muted-foreground hover:text-foreground',
            )}
          >
            {tab.label}
            {tab.id === 'console' && errorCount > 0 && (
              <span className="bg-destructive px-1.5 rounded-full min-w-5 text-white text-xs leading-5" aria-label={`${errorCount} errors`}>
                {errorCount}
              </span>
            )}
            {tab.id === 'console' && errorCount === 0 && logCount > 0 && (
              <span className="bg-muted px-1.5 rounded-full min-w-5 text-muted-foreground text-xs leading-5" aria-label={`${logCount} messages`}>
                {logCount > 99 ? '99+' : logCount}
              </span>
            )}
          </button>
        )
      })}
    </div>
  )
}
