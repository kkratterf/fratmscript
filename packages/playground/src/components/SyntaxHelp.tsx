import { useMemo, useState } from 'react'
import { Search } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { useI18n, type Lang } from '@/lib/i18n'
import {
  Dialog,
  DialogTrigger,
  DialogPopup,
  DialogHeader,
  DialogPanel,
  DialogTitle,
  DialogDescription,
} from '@/components/ui/dialog'

function HelpIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <circle cx="12" cy="12" r="10" />
      <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" />
      <path d="M12 17h.01" />
    </svg>
  )
}

type Localized = Record<Lang, string>

interface SyntaxItem {
  fratm: string
  js: string
  desc: Localized
  /** Text inserted on click, when it differs from `fratm` */
  insert?: string
}

const syntaxReference: { category: Localized; items: SyntaxItem[] }[] = [
  { category: { en: 'Variables', nap: 'Variabili' }, items: [
    { fratm: 'chist è', js: 'const', desc: { en: 'Constant declaration', nap: 'Costante: nun cagna maje' }, insert: 'chist è ' },
    { fratm: 'tien', js: 'let', desc: { en: 'Variable declaration', nap: 'Variabile ca pò cagnà' } },
    { fratm: 'tien x = 1, y = 2', js: 'let x = 1, y = 2', desc: { en: 'Several declarations', nap: "Cchiù variabili int'na botta" } },
  ]},
  { category: { en: 'Functions', nap: 'Funzioni' }, items: [
    { fratm: 'facc', js: 'function', desc: { en: 'Function declaration', nap: 'Funzione' } },
    { fratm: 'piglie', js: 'return', desc: { en: 'Return statement', nap: "Dà 'o valore e esce d'a funzione" } },
    { fratm: 'mo vir facc', js: 'async function', desc: { en: 'Async function', nap: 'Funzione asincrona' } },
    { fratm: 'facc* f() { caccia 1 }', js: 'function* f() { yield 1 }', desc: { en: 'Generator', nap: 'Generatore' } },
    { fratm: 'caccia* altri()', js: 'yield* altri()', desc: { en: 'Delegate to another generator', nap: "Passa a n'ato generatore" } },
    { fratm: '(a, b) => a + b', js: '(a, b) => a + b', desc: { en: 'Arrow function', nap: 'Funzione a freccia' } },
    { fratm: 'mo vir (x) => …', js: 'async (x) => …', desc: { en: 'Async arrow function', nap: 'Freccia asincrona' }, insert: 'mo vir (x) => ' },
    { fratm: 'facc f(a = 1, ...resto)', js: 'function f(a = 1, ...resto)', desc: { en: 'Default and rest parameters', nap: "Parametri cu valore 'e default e resto" } },
    { fratm: '[...a]  { ...o }  f(...xs)', js: '[...a]  { ...o }  f(...xs)', desc: { en: 'Spread', nap: "Spanne 'e elementi" }, insert: '...' },
    { fratm: 'chist è { a, b: x, ...r } = o', js: 'const { a, b: x, ...r } = o', desc: { en: 'Object destructuring', nap: "Destructuring 'e n'oggetto" } },
    { fratm: 'tien [primo, ...ato] = xs', js: 'let [primo, ...ato] = xs', desc: { en: 'Array destructuring', nap: "Destructuring 'e na lista" } },
    { fratm: '{ nome, [k]: v, saluta() { } }', js: '{ nome, [k]: v, saluta() { } }', desc: { en: 'Object shorthand, computed keys, methods', nap: 'Oggetti: nomme curte, chiave calcolate, metodi' }, insert: '{ }' },
    { fratm: 'aspett', js: 'await', desc: { en: 'Await expression', nap: 'Aspetta na promise' } },
  ]},
  { category: { en: 'Control Flow', nap: 'Cuntrollo' }, items: [
    { fratm: 'si', js: 'if', desc: { en: 'If condition', nap: "Si 'a condizione è vera" } },
    { fratm: 'sinnò', js: 'else', desc: { en: 'Else branch', nap: "Sinnò: quanno nun è vero" } },
    { fratm: 'vedimm (x) { caso 1: … predefinit: … }', js: 'switch (x) { case 1: … default: … }', desc: { en: 'Switch', nap: 'Sceglie tra cchiù casi' }, insert: 'vedimm (x) {\n  caso 1:\n    rompe\n  predefinit:\n}' },
    { fratm: 'mentre che', js: 'while', desc: { en: 'While loop', nap: 'Ripete fino a quanno è vero' } },
    { fratm: 'primm fa { } mentre che (c)', js: 'do { } while (c)', desc: { en: 'Do...while loop', nap: "Fa primma, po' ripete" } },
    { fratm: 'pe', js: 'for', desc: { en: 'For loop', nap: 'Ripete cuntanno' } },
    { fratm: 'pe ogni (chist è x dint xs)', js: 'for (const x of xs)', desc: { en: 'For...of loop', nap: "Passa 'e elementi uno a uno" } },
    { fratm: "pe ogni (tien k 'ncopp o)", js: 'for (let k in o)', desc: { en: 'For...in loop', nap: "Passa 'e chiave 'e n'oggetto" } },
    { fratm: 'pe aspett (… dint …)', js: 'for await (… of …)', desc: { en: 'Async iteration', nap: "Passa 'e elementi aspettanno" }, insert: 'pe aspett (chist è x dint xs) { }' },
    { fratm: 'rompe', js: 'break', desc: { en: 'Break loop', nap: "Esce d'o ciclo" } },
    { fratm: 'salta', js: 'continue', desc: { en: 'Continue loop', nap: "Zompa a 'o giro appriesso" } },
  ]},
  { category: { en: 'Classes', nap: 'Classi' }, items: [
    { fratm: 'na famiglie', js: 'class', desc: { en: 'Class declaration', nap: 'Classe' } },
    { fratm: 'nu bell', js: 'new', desc: { en: 'New instance', nap: 'Oggetto nuovo' } },
    { fratm: 'stu cos', js: 'this', desc: { en: 'This reference', nap: "L'oggetto stesso" } },
    { fratm: 'costruttore', js: 'constructor', desc: { en: 'Constructor method', nap: "Prepara l'oggetto" } },
    { fratm: 'damme x() { }', js: 'get x() { }', desc: { en: 'Getter', nap: 'Getter' } },
    { fratm: 'miette x(v) { }', js: 'set x(v) { }', desc: { en: 'Setter', nap: 'Setter' } },
    { fratm: 'conteggio = 0  #segreto', js: 'conteggio = 0  #segreto', desc: { en: 'Fields and private members', nap: 'Campi e membri privati' } },
  ]},
  { category: { en: 'Values', nap: 'Valori' }, items: [
    { fratm: 'overo', js: 'true', desc: { en: 'Boolean true', nap: 'Vero' } },
    { fratm: 'sfòls', js: 'false', desc: { en: 'Boolean false', nap: 'Falso' } },
    { fratm: 'nisciun', js: 'null', desc: { en: 'Null value', nap: 'Niente' } },
    { fratm: 'boh', js: 'undefined', desc: { en: 'Undefined value', nap: 'Nun definito' } },
    { fratm: '`Uè ${nome}!`', js: '`Uè ${nome}!`', desc: { en: 'Template literal', nap: 'Template' } },
    { fratm: '/ab+/gi', js: '/ab+/gi', desc: { en: 'Regex literal', nap: 'Espressione regolare' } },
  ]},
  { category: { en: 'Operators', nap: 'Operatori' }, items: [
    { fratm: 'e / pure', js: '&&', desc: { en: 'Logical AND', nap: 'E' }, insert: 'e' },
    { fratm: 'o', js: '||', desc: { en: 'Logical OR', nap: 'O' } },
    { fratm: 'no / manco', js: '!', desc: { en: 'Logical NOT', nap: 'No' }, insert: 'no ' },
    { fratm: 'x += 1', js: 'x += 1', desc: { en: 'Compound assignment (also -= *= /= %= **= ??= ||= &&=)', nap: 'Assegnazione composta (pure -= *= /= %= **= ??= ||= &&=)' } },
    { fratm: 'i++ / i--', js: 'i++ / i--', desc: { en: 'Increment / decrement', nap: "Aumenta / scala 'e uno" }, insert: 'i++' },
    { fratm: 'a ?? b', js: 'a ?? b', desc: { en: 'Nullish coalescing', nap: "Valore 'e riserva si è nisciun o boh" } },
    { fratm: 'a?.b  a?.[k]  f?.()', js: 'a?.b  a?.[k]  f?.()', desc: { en: 'Optional chaining', nap: 'Accesso sicuro' } },
  ]},
  { category: { en: 'Console', nap: 'Console' }, items: [
    { fratm: 'stamm a dì', js: 'console.log', desc: { en: 'Log output', nap: 'Stampa' }, insert: 'stamm a dì()' },
    { fratm: 'avvis a dì', js: 'console.warn', desc: { en: 'Warning output', nap: "Stampa n'avviso" } },
    { fratm: 'scrive a dì', js: 'console.error', desc: { en: 'Error output', nap: "Stampa n'errore" } },
  ]},
  { category: { en: 'Error Handling', nap: 'Errori' }, items: [
    { fratm: 'pruvamm', js: 'try', desc: { en: 'Try block', nap: 'Prova' } },
    { fratm: 'e si schiatta', js: 'catch', desc: { en: 'Catch block', nap: "Acchiappa l'errore" } },
    { fratm: 'aropp', js: 'finally', desc: { en: 'Always runs after try/catch', nap: 'Gira sempe doppo try/catch' } },
    { fratm: 'iett', js: 'throw', desc: { en: 'Throw error', nap: "Lancia n'errore" } },
  ]},
  { category: { en: 'Modules', nap: 'Moduli' }, items: [
    { fratm: 'chiamm ... da', js: 'import ... from', desc: { en: 'Import module', nap: 'Chiamma nu modulo' }, insert: 'chiamm {  } da ""' },
    { fratm: 'mann for', js: 'export', desc: { en: 'Export', nap: 'Esporta' } },
  ]},
]

function matches(item: SyntaxItem, query: string, lang: Lang) {
  const q = query.trim().toLowerCase()
  return !q || [item.fratm, item.js, item.desc[lang]].some(text => text.toLowerCase().includes(q))
}

export function SyntaxHelp({ onInsert }: { onInsert: (text: string) => void }) {
  const { lang, t } = useI18n()
  const [open, setOpen] = useState(false)
  const [query, setQuery] = useState('')

  const sections = useMemo(() => syntaxReference
    .map(section => ({ ...section, items: section.items.filter(item => matches(item, query, lang)) }))
    .filter(section => section.items.length > 0), [query, lang])

  const insert = (item: SyntaxItem) => {
    setOpen(false)
    onInsert(item.insert ?? item.fratm)
  }

  return (
    <Dialog open={open} onOpenChange={(next) => { setOpen(next); if (!next) setQuery('') }}>
      <DialogTrigger
        render={
          <Button variant="outline" size="icon-sm" aria-label={t('syntax.button')}>
            <HelpIcon className="size-3.5" />
          </Button>
        }
      />
      <DialogPopup className="flex flex-col max-w-2xl max-h-[80vh]">
        <DialogHeader>
          <DialogTitle>{t('syntax.title')}</DialogTitle>
          <DialogDescription>
            {t('syntax.description')}
          </DialogDescription>
          <div className="relative mt-3">
            <Search className="top-1/2 left-2.5 absolute size-4 text-muted-foreground -translate-y-1/2 pointer-events-none" />
            <input
              type="search"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder={t('syntax.search')}
              aria-label={t('syntax.searchLabel')}
              className="bg-secondary py-1.5 pr-3 pl-8 border border-input rounded-lg focus-visible:outline-2 focus-visible:outline-ring focus-visible:outline-offset-2 w-full text-sm"
              autoFocus
            />
          </div>
        </DialogHeader>
        <DialogPanel className="space-y-6">
          {sections.length === 0 && (
            <p className="py-6 text-muted-foreground text-sm text-center">{t('syntax.nothing', { query })}</p>
          )}
          {sections.map((section) => (
            <div key={section.category.en}>
              <h3 className="mb-2 font-semibold text-foreground text-sm">{section.category[lang]}</h3>
              <div className="border rounded-md overflow-hidden">
                <table className="w-full text-sm">
                  <thead className="bg-muted/50">
                    <tr>
                      <th className="px-3 py-2 font-medium text-muted-foreground text-left">FratmScript</th>
                      <th className="px-3 py-2 font-medium text-muted-foreground text-left">JavaScript</th>
                      <th className="hidden sm:table-cell px-3 py-2 font-medium text-muted-foreground text-left">{t('syntax.description.column')}</th>
                    </tr>
                  </thead>
                  <tbody>
                    {section.items.map((item) => (
                      <tr
                        key={item.fratm}
                        className="hover:bg-accent/60 border-border border-t cursor-pointer"
                        onClick={() => insert(item)}
                      >
                        <td className="px-3 py-2 font-mono text-primary">
                          <button
                            type="button"
                            className="font-mono text-left cursor-pointer"
                            onClick={(e) => { e.stopPropagation(); insert(item) }}
                            aria-label={t('syntax.insert', { code: item.fratm })}
                          >
                            {item.fratm}
                          </button>
                        </td>
                        <td className="px-3 py-2 font-mono text-muted-foreground">{item.js}</td>
                        <td className="hidden sm:table-cell px-3 py-2 text-muted-foreground">{item.desc[lang]}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          ))}
        </DialogPanel>
      </DialogPopup>
    </Dialog>
  )
}
