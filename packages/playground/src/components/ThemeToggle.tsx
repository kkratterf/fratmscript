import { useEffect, useState } from 'react'
import { useTheme } from 'next-themes'
import { Monitor, Moon, Sun } from 'lucide-react'
import { Tooltip, TooltipTrigger, TooltipPopup } from '@/components/ui/tooltip'
import { useI18n } from '@/lib/i18n'

const ORDER = ['system', 'light', 'dark'] as const
type ThemeChoice = (typeof ORDER)[number]

/** Cycles system → light → dark; next-themes remembers the choice. */
export function ThemeToggle() {
  const { theme, setTheme } = useTheme()
  const { t } = useI18n()
  // next-themes only knows the stored theme after mount
  const [mounted, setMounted] = useState(false)
  useEffect(() => setMounted(true), [])

  const current: ThemeChoice = mounted && ORDER.includes(theme as ThemeChoice) ? (theme as ThemeChoice) : 'system'
  const next = ORDER[(ORDER.indexOf(current) + 1) % ORDER.length]
  const Icon = current === 'light' ? Sun : current === 'dark' ? Moon : Monitor

  return (
    <Tooltip>
      <TooltipTrigger
        render={
          <button
            type="button"
            onClick={() => setTheme(next)}
            aria-label={`${t(`theme.${current}`)}. ${t('theme.switch', { theme: t(`theme.name.${next}`) })}`}
            className="hover:bg-accent p-1.5 rounded-md text-muted-foreground hover:text-foreground transition-colors"
          >
            <Icon className="size-4" />
          </button>
        }
      />
      <TooltipPopup>{t(`theme.${current}`)}</TooltipPopup>
    </Tooltip>
  )
}
