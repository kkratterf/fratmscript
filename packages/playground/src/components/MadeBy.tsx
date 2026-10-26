import { Button } from '@/components/ui/button'
import { Tooltip, TooltipTrigger, TooltipPopup } from '@/components/ui/tooltip'
import { useI18n } from '@/lib/i18n'
import photo from '@/assets/federico.jpg'

const MAKER = {
  name: 'Federico',
  fullName: 'Federico Kratter Thaler',
  href: 'https://www.linkedin.com/in/kkratter/',
}

/** "Made with love by" and a link to the maker, as in the M63 footers. */
export function MadeBy() {
  const { t } = useI18n()
  return (
    <p className="flex items-center gap-0.5 whitespace-nowrap">
      <span>{t('footer.madeBy')}</span>
      <Tooltip>
        <TooltipTrigger
          render={
            <Button
              variant="ghost"
              size="xs"
              // The avatar already reads as a margin at the start
              className="ps-[calc(--spacing(2)-3px)] pe-[calc(--spacing(2)-2px)] text-foreground text-xs"
              render={<a href={MAKER.href} target="_blank" rel="noopener noreferrer" />}
            />
          }
        >
          <img src={photo} alt="" className="rounded-full size-4 object-cover" />
          {MAKER.name}
        </TooltipTrigger>
        <TooltipPopup>{MAKER.fullName}</TooltipPopup>
      </Tooltip>
    </p>
  )
}
