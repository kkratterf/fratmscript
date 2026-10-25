import { useEffect, useRef } from 'react'
import { ChevronLeft, ChevronRight } from 'lucide-react'
import { GuideSnippet } from '@/components/GuideSnippet'
import { RichText } from '@/components/LessonPanel'
import { Button } from '@/components/ui/button'
import { NativeSelect } from '@/components/ui/select'
import { guide } from '@/lib/guide'
import { useI18n } from '@/lib/i18n'
import { cn } from '@/lib/utils'

const isMac = typeof navigator !== 'undefined' && /Mac|iPhone|iPad/.test(navigator.platform)
const runShortcut = isMac ? '⌘↵' : 'Ctrl+↵'

interface GuidePageProps {
  /** Index of the open section */
  index: number
  onSelect: (index: number) => void
  /** The compiler is loaded */
  ready: boolean
  /** Continue in the playground with this code */
  onOpen: (code: string) => void
}

/** 'A guida: the language guide, one section at a time, with runnable snippets. */
export function GuidePage({ index, onSelect, ready, onOpen }: GuidePageProps) {
  const { lang, t } = useI18n()
  const section = guide[index]
  const mainRef = useRef<HTMLElement>(null)

  // A new section starts at the top
  useEffect(() => {
    mainRef.current?.scrollTo({ top: 0 })
  }, [index])

  return (
    <div className="flex md:grid flex-col flex-1 md:grid-cols-[15rem_1fr] min-h-0 overflow-hidden">
      <nav aria-label={t('guide.sections')} className="hidden md:block bg-card py-4 border-border border-r overflow-auto">
        <p className="mb-2 px-4 font-semibold text-foreground text-sm">{t('guide.title')}</p>
        <ol className="space-y-0.5 px-2">
          {guide.map((s, i) => (
            <li key={s.id}>
              <a
                href={`#guida=${s.id}`}
                onClick={e => { e.preventDefault(); onSelect(i) }}
                aria-current={i === index ? 'page' : undefined}
                className={cn(
                  'block px-2 py-1.5 rounded-md text-sm transition-colors',
                  i === index ? 'bg-accent font-medium text-foreground' : 'text-muted-foreground hover:bg-accent/60 hover:text-foreground',
                )}
              >
                <span className="inline-block mr-1.5 w-5 tabular-nums text-muted-foreground text-right">{i + 1}.</span>
                {s.title[lang]}
              </a>
            </li>
          ))}
        </ol>
      </nav>

      <main ref={mainRef} className="flex-1 bg-background min-h-0 overflow-auto">
        <article className="space-y-4 mx-auto px-4 sm:px-6 py-6 max-w-3xl">
          <div className="md:hidden">
            <NativeSelect
              aria-label={t('guide.section')}
              value={String(index)}
              onChange={e => onSelect(Number(e.target.value))}
              className="w-full"
            >
              {guide.map((s, i) => (
                <option key={s.id} value={i}>{i + 1}. {s.title[lang]}</option>
              ))}
            </NativeSelect>
          </div>

          <header>
            <p className="text-muted-foreground text-xs uppercase tracking-wider">{t('guide.title')} · {index + 1}/{guide.length}</p>
            <h2 className="mt-1 font-bold text-foreground text-2xl">{section.title[lang]}</h2>
          </header>

          {section.body[lang].map((paragraph, i) => (
            <p key={i} className="text-muted-foreground leading-relaxed"><RichText text={paragraph} /></p>
          ))}

          {index === 0 && (
            <p className="text-muted-foreground text-sm italic">{t('guide.intro', { shortcut: runShortcut })}</p>
          )}

          {section.snippets.map(snippet => (
            // The key resets the snippet's edits when the section changes
            <GuideSnippet key={`${section.id}/${snippet.file}`} initialCode={snippet.code} ready={ready} onOpen={onOpen} />
          ))}

          <div className="flex justify-between items-center gap-2 pt-4 border-border border-t">
            <Button variant="outline" size="sm" disabled={index === 0} onClick={() => onSelect(index - 1)}>
              <ChevronLeft className="size-4" />
              <span className="truncate">{index > 0 ? guide[index - 1].title[lang] : t('guide.previous')}</span>
            </Button>
            {index < guide.length - 1 && (
              <Button size="sm" onClick={() => onSelect(index + 1)}>
                <span className="truncate">{t('guide.next')}: {guide[index + 1].title[lang]}</span>
                <ChevronRight className="size-4" />
              </Button>
            )}
          </div>
        </article>
      </main>
    </div>
  )
}
