import { Tooltip, TooltipTrigger, TooltipPopup } from '@/components/ui/tooltip'
import { LANGS, useI18n } from '@/lib/i18n'

/** Switches the UI between English and Neapolitan; the choice is remembered. */
export function LanguageToggle() {
  const { lang, setLang, t } = useI18n()
  const next = LANGS[(LANGS.indexOf(lang) + 1) % LANGS.length]

  return (
    <Tooltip>
      <TooltipTrigger
        render={
          <button
            type="button"
            onClick={() => setLang(next)}
            aria-label={t('lang.switch')}
            className="hover:bg-accent px-1.5 py-1 rounded-md font-semibold text-muted-foreground hover:text-foreground text-xs tracking-wide transition-colors"
          >
            {t('lang.short')}
          </button>
        }
      />
      <TooltipPopup>{t('lang.tooltip')}</TooltipPopup>
    </Tooltip>
  )
}
