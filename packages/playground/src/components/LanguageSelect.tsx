import { NativeSelect } from '@/components/ui/select'
import { LANGS, translate, useI18n, type Lang } from '@/lib/i18n'

/** Picks the UI language; each option is written in its own language. */
export function LanguageSelect() {
  const { lang, setLang, t } = useI18n()

  return (
    <NativeSelect
      value={lang}
      onChange={e => setLang(e.target.value as Lang)}
      aria-label={t('lang.label')}
      className="w-[6.5rem]"
    >
      {LANGS.map(option => (
        <option key={option} value={option}>{translate(option, 'lang.name')}</option>
      ))}
    </NativeSelect>
  )
}
