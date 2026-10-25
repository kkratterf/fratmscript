import { Button } from '@/components/ui/button'
import { Tooltip, TooltipTrigger, TooltipPopup } from '@/components/ui/tooltip'
import { BookOpen, CodeXml, ExternalLink, GraduationCap, Play, Share2, Square } from 'lucide-react'
import { ThemeSelect } from '@/components/ThemeSelect'
import { LanguageSelect } from '@/components/LanguageSelect'
import { useI18n } from '@/lib/i18n'
import { cn } from '@/lib/utils'

interface HeaderProps {
  onRun: () => void
  onStop: () => void
  onShare: () => void
  onEmbed: () => void
  onTour: () => void
  tourActive: boolean
  onGuide: () => void
  guideActive: boolean
  isRunning: boolean
  disabled: boolean
}

const isMac = typeof navigator !== 'undefined' && /Mac|iPhone|iPad/.test(navigator.platform)
const runShortcut = isMac ? '⌘↵' : 'Ctrl+↵'

function GitHubIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="currentColor">
      <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
    </svg>
  )
}

function LinkedInIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="currentColor">
      <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/>
    </svg>
  )
}

function XIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="currentColor">
      <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/>
    </svg>
  )
}

export function Header({ onRun, onStop, onShare, onEmbed, onTour, tourActive, onGuide, guideActive, isRunning, disabled }: HeaderProps) {
  const { t } = useI18n()
  return (
    <header className="flex flex-col gap-2 bg-card px-4 py-3 border-border border-b">
      <div className="flex justify-between items-center gap-3">
        <div className="flex items-center gap-3 min-w-0">
          <h1 className="font-bold text-primary text-lg sm:text-xl truncate">FratmScript<span className="max-[399px]:hidden"> 🤌🏻</span></h1>
        </div>
        <div className="flex items-center gap-1 sm:gap-3 shrink-0">
          <Tooltip>
            <TooltipTrigger
              render={
                <Button
                  variant={tourActive ? 'secondary' : 'ghost'}
                  size="sm"
                  onClick={onTour}
                  aria-pressed={tourActive}
                  aria-label={t('header.tourLabel')}
                >
                  <GraduationCap className="size-4" />
                  <span className="hidden sm:inline">{t('header.tour')}</span>
                </Button>
              }
            />
            <TooltipPopup>{t(tourActive ? 'header.tourExit' : 'header.tourOpen')}</TooltipPopup>
          </Tooltip>
          <Tooltip>
            <TooltipTrigger
              render={
                <Button
                  variant={guideActive ? 'secondary' : 'ghost'}
                  size="sm"
                  onClick={onGuide}
                  aria-pressed={guideActive}
                  aria-label={t('header.guideLabel')}
                >
                  <BookOpen className="size-4" />
                  <span className="hidden sm:inline">{t('header.guide')}</span>
                </Button>
              }
            />
            <TooltipPopup>{t(guideActive ? 'header.guideExit' : 'header.guideOpen')}</TooltipPopup>
          </Tooltip>
          <div className="hidden sm:flex items-center gap-2">
            <LanguageSelect />
            <ThemeSelect />
          </div>
          <div className="hidden sm:flex items-center gap-1">
            <Tooltip>
              <TooltipTrigger
                render={
                  <a
                    href="https://github.com/kkratterf/fratmscript"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="hover:bg-accent p-1.5 rounded-md text-muted-foreground hover:text-foreground transition-colors"
                    aria-label={t('header.github')}
                  >
                    <GitHubIcon className="size-4" />
                  </a>
                }
              />
              <TooltipPopup>{t('header.githubTip')}</TooltipPopup>
            </Tooltip>
            <Tooltip>
              <TooltipTrigger
                render={
                  <a
                    href="https://linkedin.com/in/kkratter"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="hover:bg-accent p-1.5 rounded-md text-muted-foreground hover:text-foreground transition-colors"
                    aria-label="LinkedIn"
                  >
                    <LinkedInIcon className="size-4" />
                  </a>
                }
              />
              <TooltipPopup>{t('header.linkedinTip')}</TooltipPopup>
            </Tooltip>
            <Tooltip>
              <TooltipTrigger
                render={
                  <a
                    href="https://x.com/kkratter"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="hover:bg-accent p-1.5 rounded-md text-muted-foreground hover:text-foreground transition-colors"
                    aria-label="X (Twitter)"
                  >
                    <XIcon className="size-4" />
                  </a>
                }
              />
              <TooltipPopup>{t('header.xTip')}</TooltipPopup>
            </Tooltip>
          </div>
          {/* The guide runs its own snippets: the playground's actions wait until you leave it */}
          <div className={cn('flex items-center gap-1.5', guideActive && 'hidden')}>
            {/* Run and Stop share one grid cell, so swapping them never moves the header.
                The hidden one is visibility: hidden, out of the tab order and a11y tree. */}
            <div className="grid *:col-start-1 *:row-start-1">
              <Tooltip>
                <TooltipTrigger
                  render={
                    <Button onClick={onRun} disabled={disabled} className={cn(isRunning && 'invisible')}>
                      <Play className="size-3.5 fill-current" />
                      {t('header.run')}
                      <kbd className="hidden sm:inline opacity-70 ml-1 font-sans text-xs">{runShortcut}</kbd>
                    </Button>
                  }
                />
                <TooltipPopup>{t('header.runTip', { shortcut: runShortcut })}</TooltipPopup>
              </Tooltip>
              <Button variant="destructive" onClick={onStop} className={cn(!isRunning && 'invisible')}>
                <Square className="size-3.5 fill-current" />
                {t('header.stop')}
              </Button>
            </div>
            {/* Icon only on phones, so the logo is not cut off */}
            <Button variant="outline" onClick={onShare} aria-label={t('header.share')} className="max-sm:px-2.5">
              <Share2 className="size-4 sm:hidden" />
              <span className="hidden sm:inline">{t('header.share')}</span>
            </Button>
            <Tooltip>
              <TooltipTrigger
                render={
                  <Button variant="outline" size="icon" className="hidden sm:inline-flex" onClick={onEmbed} aria-label={t('share.embed')}>
                    <CodeXml className="size-4" />
                  </Button>
                }
              />
              <TooltipPopup>{t('share.embed')}</TooltipPopup>
            </Tooltip>
          </div>
        </div>
      </div>
      {/* On phones the menus get their own row */}
      <div className="sm:hidden flex gap-2 w-full">
        <LanguageSelect />
        <ThemeSelect />
      </div>
    </header>
  )
}

interface EmbedHeaderProps {
  onRun: () => void
  onStop: () => void
  /** Full playground URL with the current code */
  playgroundUrl: string
  isRunning: boolean
  disabled: boolean
}

/** Slim header for `?embed=1`: a link to the full playground and Run. */
export function EmbedHeader({ onRun, onStop, playgroundUrl, isRunning, disabled }: EmbedHeaderProps) {
  const { t } = useI18n()
  return (
    <header className="flex justify-between items-center gap-2 bg-card px-3 py-1.5 border-border border-b">
      <a
        href={playgroundUrl}
        target="_blank"
        rel="noopener noreferrer"
        className="flex items-center gap-1.5 min-w-0 font-semibold text-primary text-sm hover:underline"
        title={t('embed.open')}
      >
        <span className="truncate">FratmScript 🤌🏻</span>
        <ExternalLink className="size-3.5 shrink-0" aria-hidden />
        <span className="sr-only">{t('embed.open')}</span>
      </a>
      {isRunning ? (
        <Button variant="destructive" size="sm" onClick={onStop}>
          <Square className="size-3 fill-current" />
          {t('header.stop')}
        </Button>
      ) : (
        <Button size="sm" onClick={onRun} disabled={disabled}>
          <Play className="size-3 fill-current" />
          {t('header.run')}
        </Button>
      )}
    </header>
  )
}
