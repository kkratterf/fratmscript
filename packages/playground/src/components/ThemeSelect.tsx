import { useEffect, useState } from 'react'
import { useTheme } from 'next-themes'
import { NativeSelect } from '@/components/ui/select'
import { useI18n } from '@/lib/i18n'

const THEMES = ['system', 'light', 'dark'] as const
type ThemeChoice = (typeof THEMES)[number]

/** Picks system, light or dark; next-themes remembers the choice. */
export function ThemeSelect() {
  const { theme, setTheme } = useTheme()
  const { t } = useI18n()
  // next-themes only knows the stored theme after mount
  const [mounted, setMounted] = useState(false)
  useEffect(() => setMounted(true), [])

  const current: ThemeChoice = mounted && THEMES.includes(theme as ThemeChoice) ? (theme as ThemeChoice) : 'system'

  return (
    <NativeSelect
      value={current}
      onChange={e => setTheme(e.target.value)}
      aria-label={t('theme.label')}
      className="w-[6.5rem]"
    >
      {THEMES.map(option => (
        <option key={option} value={option}>{t(`theme.${option}`)}</option>
      ))}
    </NativeSelect>
  )
}
