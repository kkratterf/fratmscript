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
  { category: { it: 'Variabili', en: 'Variables', nap: 'Variabili' }, items: [
    { fratm: 'chist è', js: 'const', desc: { it: 'Costante', en: 'Constant declaration', nap: 'Costante: nun cagna maje' }, insert: 'chist è ' },
    { fratm: 'tien', js: 'let', desc: { it: 'Variabile', en: 'Variable declaration', nap: 'Variabile ca pò cagnà' } },
    { fratm: 'tien x = 1, y = 2', js: 'let x = 1, y = 2', desc: { it: 'Più dichiarazioni insieme', en: 'Several declarations', nap: "Cchiù variabili int'na botta" } },
  ]},
  { category: { it: 'Funzioni', en: 'Functions', nap: 'Funzioni' }, items: [
    { fratm: 'facc', js: 'function', desc: { it: 'Funzione', en: 'Function declaration', nap: 'Funzione' } },
    { fratm: 'piglie', js: 'return', desc: { it: 'Restituisce un valore', en: 'Return statement', nap: "Dà 'o valore e esce d'a funzione" } },
    { fratm: 'mo vir facc', js: 'async function', desc: { it: 'Funzione asincrona', en: 'Async function', nap: 'Funzione asincrona' } },
    { fratm: 'facc* f() { caccia 1 }', js: 'function* f() { yield 1 }', desc: { it: 'Generatore', en: 'Generator', nap: 'Generatore' } },
    { fratm: 'caccia* altri()', js: 'yield* altri()', desc: { it: 'Delega a un altro generatore', en: 'Delegate to another generator', nap: "Passa a n'ato generatore" } },
    { fratm: '(a, b) => a + b', js: '(a, b) => a + b', desc: { it: 'Funzione freccia', en: 'Arrow function', nap: 'Funzione a freccia' } },
    { fratm: 'mo vir (x) => …', js: 'async (x) => …', desc: { it: 'Funzione freccia asincrona', en: 'Async arrow function', nap: 'Freccia asincrona' }, insert: 'mo vir (x) => ' },
    { fratm: 'facc f(a = 1, ...resto)', js: 'function f(a = 1, ...resto)', desc: { it: 'Parametri con default e rest', en: 'Default and rest parameters', nap: "Parametri cu valore 'e default e resto" } },
    { fratm: '[...a]  { ...o }  f(...xs)', js: '[...a]  { ...o }  f(...xs)', desc: { it: 'Spread', en: 'Spread', nap: "Spanne 'e elementi" }, insert: '...' },
    { fratm: 'chist è { a, b: x, ...r } = o', js: 'const { a, b: x, ...r } = o', desc: { it: 'Destructuring di oggetti', en: 'Object destructuring', nap: "Destructuring 'e n'oggetto" } },
    { fratm: 'tien [primo, ...ato] = xs', js: 'let [primo, ...ato] = xs', desc: { it: 'Destructuring di array', en: 'Array destructuring', nap: "Destructuring 'e na lista" } },
    { fratm: '{ nome, [k]: v, saluta() { } }', js: '{ nome, [k]: v, saluta() { } }', desc: { it: 'Oggetti: forma breve, chiavi calcolate, metodi', en: 'Object shorthand, computed keys, methods', nap: 'Oggetti: nomme curte, chiave calcolate, metodi' }, insert: '{ }' },
    { fratm: 'aspett', js: 'await', desc: { it: 'Attende una promise', en: 'Await expression', nap: 'Aspetta na promise' } },
  ]},
  { category: { it: 'Controllo del flusso', en: 'Control Flow', nap: 'Cuntrollo' }, items: [
    { fratm: 'si', js: 'if', desc: { it: 'Condizione', en: 'If condition', nap: "Si 'a condizione è vera" } },
    { fratm: 'sinnò', js: 'else', desc: { it: 'Altrimenti', en: 'Else branch', nap: "Sinnò: quanno nun è vero" } },
    { fratm: 'vedimm (x) { caso 1: … predefinit: … }', js: 'switch (x) { case 1: … default: … }', desc: { it: 'Scelta tra più casi', en: 'Switch', nap: 'Sceglie tra cchiù casi' }, insert: 'vedimm (x) {\n  caso 1:\n    rompe\n  predefinit:\n}' },
    { fratm: 'mentre che', js: 'while', desc: { it: 'Ciclo while', en: 'While loop', nap: 'Ripete fino a quanno è vero' } },
    { fratm: 'primm fa { } mentre che (c)', js: 'do { } while (c)', desc: { it: 'Ciclo do...while', en: 'Do...while loop', nap: "Fa primma, po' ripete" } },
    { fratm: 'pe', js: 'for', desc: { it: 'Ciclo for', en: 'For loop', nap: 'Ripete cuntanno' } },
    { fratm: 'pe ogni (chist è x dint xs)', js: 'for (const x of xs)', desc: { it: 'Ciclo for...of', en: 'For...of loop', nap: "Passa 'e elementi uno a uno" } },
    { fratm: "pe ogni (tien k 'ncopp o)", js: 'for (let k in o)', desc: { it: 'Ciclo for...in', en: 'For...in loop', nap: "Passa 'e chiave 'e n'oggetto" } },
    { fratm: 'pe aspett (… dint …)', js: 'for await (… of …)', desc: { it: 'Iterazione asincrona', en: 'Async iteration', nap: "Passa 'e elementi aspettanno" }, insert: 'pe aspett (chist è x dint xs) { }' },
    { fratm: 'rompe', js: 'break', desc: { it: 'Esce dal ciclo', en: 'Break loop', nap: "Esce d'o ciclo" } },
    { fratm: 'salta', js: 'continue', desc: { it: 'Passa al giro successivo', en: 'Continue loop', nap: "Zompa a 'o giro appriesso" } },
  ]},
  { category: { it: 'Classi', en: 'Classes', nap: 'Classi' }, items: [
    { fratm: 'na famiglie', js: 'class', desc: { it: 'Classe', en: 'Class declaration', nap: 'Classe' } },
    { fratm: 'nu bell', js: 'new', desc: { it: 'Nuova istanza', en: 'New instance', nap: 'Oggetto nuovo' } },
    { fratm: 'stu cos', js: 'this', desc: { it: "L'istanza corrente", en: 'This reference', nap: "L'oggetto stesso" } },
    { fratm: 'costruttore', js: 'constructor', desc: { it: 'Costruttore', en: 'Constructor method', nap: "Prepara l'oggetto" } },
    { fratm: 'damme x() { }', js: 'get x() { }', desc: { it: 'Getter', en: 'Getter', nap: 'Getter' } },
    { fratm: 'miette x(v) { }', js: 'set x(v) { }', desc: { it: 'Setter', en: 'Setter', nap: 'Setter' } },
    { fratm: 'conteggio = 0  #segreto', js: 'conteggio = 0  #segreto', desc: { it: 'Campi e membri privati', en: 'Fields and private members', nap: 'Campi e membri privati' } },
  ]},
  { category: { it: 'Valori', en: 'Values', nap: 'Valori' }, items: [
    { fratm: 'overo', js: 'true', desc: { it: 'Booleano vero', en: 'Boolean true', nap: 'Vero' } },
    { fratm: 'sfòls', js: 'false', desc: { it: 'Booleano falso', en: 'Boolean false', nap: 'Falso' } },
    { fratm: 'nisciun', js: 'null', desc: { it: 'Valore nullo', en: 'Null value', nap: 'Niente' } },
    { fratm: 'boh', js: 'undefined', desc: { it: 'Valore non definito', en: 'Undefined value', nap: 'Nun definito' } },
    { fratm: '`Uè ${nome}!`', js: '`Uè ${nome}!`', desc: { it: 'Template literal', en: 'Template literal', nap: 'Template' } },
    { fratm: '/ab+/gi', js: '/ab+/gi', desc: { it: 'Espressione regolare', en: 'Regex literal', nap: 'Espressione regolare' } },
  ]},
  { category: { it: 'Operatori', en: 'Operators', nap: 'Operatori' }, items: [
    { fratm: 'e / pure', js: '&&', desc: { it: 'AND logico', en: 'Logical AND', nap: 'E' }, insert: 'e' },
    { fratm: 'o', js: '||', desc: { it: 'OR logico', en: 'Logical OR', nap: 'O' } },
    { fratm: 'no / manco', js: '!', desc: { it: 'NOT logico', en: 'Logical NOT', nap: 'No' }, insert: 'no ' },
    { fratm: 'x += 1', js: 'x += 1', desc: { it: 'Assegnazione composta (anche -= *= /= %= **= ??= ||= &&=)', en: 'Compound assignment (also -= *= /= %= **= ??= ||= &&=)', nap: 'Assegnazione composta (pure -= *= /= %= **= ??= ||= &&=)' } },
    { fratm: 'i++ / i--', js: 'i++ / i--', desc: { it: 'Incremento / decremento', en: 'Increment / decrement', nap: "Aumenta / scala 'e uno" }, insert: 'i++' },
    { fratm: 'a ?? b', js: 'a ?? b', desc: { it: 'Valore di riserva se null o undefined', en: 'Nullish coalescing', nap: "Valore 'e riserva si è nisciun o boh" } },
    { fratm: 'a?.b  a?.[k]  f?.()', js: 'a?.b  a?.[k]  f?.()', desc: { it: 'Accesso opzionale', en: 'Optional chaining', nap: 'Accesso sicuro' } },
  ]},
  { category: { it: 'Console', en: 'Console', nap: 'Console' }, items: [
    { fratm: 'stamm a dì', js: 'console.log', desc: { it: 'Stampa', en: 'Log output', nap: 'Stampa' }, insert: 'stamm a dì()' },
    { fratm: 'avvis a dì', js: 'console.warn', desc: { it: 'Stampa un avviso', en: 'Warning output', nap: "Stampa n'avviso" } },
    { fratm: 'scrive a dì', js: 'console.error', desc: { it: 'Stampa un errore', en: 'Error output', nap: "Stampa n'errore" } },
  ]},
  { category: { it: 'Gestione degli errori', en: 'Error Handling', nap: 'Errori' }, items: [
    { fratm: 'pruvamm', js: 'try', desc: { it: 'Blocco try', en: 'Try block', nap: 'Prova' } },
    { fratm: 'e si schiatta', js: 'catch', desc: { it: 'Blocco catch', en: 'Catch block', nap: "Acchiappa l'errore" } },
    { fratm: 'aropp', js: 'finally', desc: { it: 'Gira sempre dopo try/catch', en: 'Always runs after try/catch', nap: 'Gira sempe doppo try/catch' } },
    { fratm: 'iett', js: 'throw', desc: { it: 'Lancia un errore', en: 'Throw error', nap: "Lancia n'errore" } },
  ]},
  { category: { it: 'Moduli', en: 'Modules', nap: 'Moduli' }, items: [
    { fratm: 'chiamm { a } da', js: 'import { a } from', desc: { it: 'Importa per nome', en: 'Named import', nap: 'Chiamma pe nomme' }, insert: 'chiamm {  } da ""' },
    { fratm: 'chiamm X da', js: 'import X from', desc: { it: 'Importa il default', en: 'Default import', nap: "Chiamma 'o predefinito" }, insert: 'chiamm  da ""' },
    { fratm: 'chiamm * comme ns da', js: 'import * as ns from', desc: { it: 'Importa tutto il modulo', en: 'Namespace import', nap: "Chiamma tutt'o modulo" }, insert: 'chiamm * comme  da ""' },
    { fratm: 'a comme b', js: 'a as b', desc: { it: 'Rinomina un import o un export', en: 'Rename an import or export', nap: "Dà n'atu nomme" } },
    { fratm: 'mann for', js: 'export', desc: { it: 'Esporta', en: 'Export', nap: 'Esporta' } },
    { fratm: 'mann for { a } da', js: 'export { a } from', desc: { it: 'Riesporta da un altro modulo', en: 'Re-export', nap: "Riesporta 'a n'atu modulo" }, insert: 'mann for {  } da ""' },
    { fratm: 'chiamm("…")', js: 'import("…")', desc: { it: 'Import dinamico (con aspett)', en: 'Dynamic import (with aspett)', nap: 'Import dinamico (cu aspett)' }, insert: 'aspett chiamm("")' },
  ]},
  // Helpers of the fratm library (crates/fratm-core/src/stdlib.rs): a click inserts the import
  { category: { it: 'Libreria fratm', en: 'fratm library', nap: "'A libreria fratm" }, items: [
    { fratm: 'a_caso(min, max)', js: 'Math.random()', desc: { it: 'Intero a caso tra min e max', en: 'Random integer from min to max', nap: 'Nummero a caso tra min e max' }, insert: 'chiamm { a_caso } da "fratm"\n' },
    { fratm: 'uno_a_caso(lista)', js: 'list[random]', desc: { it: 'Elemento a caso di una lista', en: 'Random item of a list', nap: "N'elemento a caso 'e na lista" }, insert: 'chiamm { uno_a_caso } da "fratm"\n' },
    { fratm: 'mischia(lista)', js: 'shuffle', desc: { it: 'Copia mescolata di una lista', en: 'Shuffled copy of a list', nap: 'Na copia mischiata' }, insert: 'chiamm { mischia } da "fratm"\n' },
    { fratm: 'quanti(cosa)', js: '.length / .size', desc: { it: 'Quanti elementi', en: 'How many items', nap: "Quant'elementi ce stanno" }, insert: 'chiamm { quanti } da "fratm"\n' },
    { fratm: 'arrotonda(n, cifre)', js: 'Math.round', desc: { it: 'Arrotonda a tot cifre', en: 'Round to n digits', nap: 'Arrotonda' }, insert: 'chiamm { arrotonda } da "fratm"\n' },
    { fratm: 'ripeti(volte, f)', js: 'for (…) f(i)', desc: { it: 'Ripete una funzione', en: 'Call a function n times', nap: 'Chiamma na funzione tante vote' }, insert: 'chiamm { ripeti } da "fratm"\n' },
    { fratm: 'aspetta(ms)', js: 'setTimeout Promise', desc: { it: 'Aspetta ms millisecondi (con aspett)', en: 'Wait ms milliseconds (with aspett)', nap: 'Aspetta ms millisecondi (cu aspett)' }, insert: 'chiamm { aspetta } da "fratm"\n' },
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
            <div key={section.category.it}>
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
