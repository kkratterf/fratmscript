import { useMemo, useState } from 'react'
import { Search } from 'lucide-react'
import { Button } from '@/components/ui/button'
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

interface SyntaxItem {
  fratm: string
  js: string
  desc: string
  /** Text inserted on click, when it differs from `fratm` */
  insert?: string
}

const syntaxReference: { category: string; items: SyntaxItem[] }[] = [
  { category: 'Variables', items: [
    { fratm: 'chist è', js: 'const', desc: 'Constant declaration', insert: 'chist è ' },
    { fratm: 'tien', js: 'let', desc: 'Variable declaration' },
    { fratm: 'tien x = 1, y = 2', js: 'let x = 1, y = 2', desc: 'Several declarations' },
  ]},
  { category: 'Functions', items: [
    { fratm: 'facc', js: 'function', desc: 'Function declaration' },
    { fratm: 'piglie', js: 'return', desc: 'Return statement' },
    { fratm: 'mo vir facc', js: 'async function', desc: 'Async function' },
    { fratm: 'facc* f() { caccia 1 }', js: 'function* f() { yield 1 }', desc: 'Generator' },
    { fratm: 'caccia* altri()', js: 'yield* altri()', desc: 'Delegate to another generator' },
    { fratm: '(a, b) => a + b', js: '(a, b) => a + b', desc: 'Arrow function' },
    { fratm: 'mo vir (x) => …', js: 'async (x) => …', desc: 'Async arrow function', insert: 'mo vir (x) => ' },
    { fratm: 'facc f(a = 1, ...resto)', js: 'function f(a = 1, ...resto)', desc: 'Default and rest parameters' },
    { fratm: '[...a]  { ...o }  f(...xs)', js: '[...a]  { ...o }  f(...xs)', desc: 'Spread', insert: '...' },
    { fratm: 'chist è { a, b: x, ...r } = o', js: 'const { a, b: x, ...r } = o', desc: 'Object destructuring' },
    { fratm: 'tien [primo, ...ato] = xs', js: 'let [primo, ...ato] = xs', desc: 'Array destructuring' },
    { fratm: '{ nome, [k]: v, saluta() { } }', js: '{ nome, [k]: v, saluta() { } }', desc: 'Object shorthand, computed keys, methods', insert: '{ }' },
    { fratm: 'aspett', js: 'await', desc: 'Await expression' },
  ]},
  { category: 'Control Flow', items: [
    { fratm: 'si', js: 'if', desc: 'If condition' },
    { fratm: 'sinnò', js: 'else', desc: 'Else branch' },
    { fratm: 'vedimm (x) { caso 1: … predefinit: … }', js: 'switch (x) { case 1: … default: … }', desc: 'Switch', insert: 'vedimm (x) {\n  caso 1:\n    rompe\n  predefinit:\n}' },
    { fratm: 'mentre che', js: 'while', desc: 'While loop' },
    { fratm: 'primm fa { } mentre che (c)', js: 'do { } while (c)', desc: 'Do...while loop' },
    { fratm: 'pe', js: 'for', desc: 'For loop' },
    { fratm: 'pe ogni (chist è x dint xs)', js: 'for (const x of xs)', desc: 'For...of loop' },
    { fratm: "pe ogni (tien k 'ncopp o)", js: 'for (let k in o)', desc: 'For...in loop' },
    { fratm: 'pe aspett (… dint …)', js: 'for await (… of …)', desc: 'Async iteration', insert: 'pe aspett (chist è x dint xs) { }' },
    { fratm: 'rompe', js: 'break', desc: 'Break loop' },
    { fratm: 'salta', js: 'continue', desc: 'Continue loop' },
  ]},
  { category: 'Classes', items: [
    { fratm: 'na famiglie', js: 'class', desc: 'Class declaration' },
    { fratm: 'nu bell', js: 'new', desc: 'New instance' },
    { fratm: 'stu cos', js: 'this', desc: 'This reference' },
    { fratm: 'costruttore', js: 'constructor', desc: 'Constructor method' },
    { fratm: 'damme x() { }', js: 'get x() { }', desc: 'Getter' },
    { fratm: 'miette x(v) { }', js: 'set x(v) { }', desc: 'Setter' },
    { fratm: 'conteggio = 0  #segreto', js: 'conteggio = 0  #segreto', desc: 'Fields and private members' },
  ]},
  { category: 'Values', items: [
    { fratm: 'overo', js: 'true', desc: 'Boolean true' },
    { fratm: 'sfòls', js: 'false', desc: 'Boolean false' },
    { fratm: 'nisciun', js: 'null', desc: 'Null value' },
    { fratm: 'boh', js: 'undefined', desc: 'Undefined value' },
    { fratm: '`Uè ${nome}!`', js: '`Uè ${nome}!`', desc: 'Template literal' },
    { fratm: '/ab+/gi', js: '/ab+/gi', desc: 'Regex literal' },
  ]},
  { category: 'Operators', items: [
    { fratm: 'e / pure', js: '&&', desc: 'Logical AND', insert: 'e' },
    { fratm: 'o', js: '||', desc: 'Logical OR' },
    { fratm: 'no / manco', js: '!', desc: 'Logical NOT', insert: 'no ' },
    { fratm: 'x += 1', js: 'x += 1', desc: 'Compound assignment (also -= *= /= %= **= ??= ||= &&=)' },
    { fratm: 'i++ / i--', js: 'i++ / i--', desc: 'Increment / decrement', insert: 'i++' },
    { fratm: 'a ?? b', js: 'a ?? b', desc: 'Nullish coalescing' },
    { fratm: 'a?.b  a?.[k]  f?.()', js: 'a?.b  a?.[k]  f?.()', desc: 'Optional chaining' },
  ]},
  { category: 'Console', items: [
    { fratm: 'stamm a dì', js: 'console.log', desc: 'Log output', insert: 'stamm a dì()' },
    { fratm: 'avvis a dì', js: 'console.warn', desc: 'Warning output' },
    { fratm: 'scrive a dì', js: 'console.error', desc: 'Error output' },
  ]},
  { category: 'Error Handling', items: [
    { fratm: 'pruvamm', js: 'try', desc: 'Try block' },
    { fratm: 'e si schiatta', js: 'catch', desc: 'Catch block' },
    { fratm: 'aropp', js: 'finally', desc: 'Always runs after try/catch' },
    { fratm: 'iett', js: 'throw', desc: 'Throw error' },
  ]},
  { category: 'Modules', items: [
    { fratm: 'chiamm ... da', js: 'import ... from', desc: 'Import module', insert: 'chiamm {  } da ""' },
    { fratm: 'mann for', js: 'export', desc: 'Export' },
  ]},
]

function matches(item: SyntaxItem, query: string) {
  const q = query.trim().toLowerCase()
  return !q || [item.fratm, item.js, item.desc].some(text => text.toLowerCase().includes(q))
}

export function SyntaxHelp({ onInsert }: { onInsert: (text: string) => void }) {
  const [open, setOpen] = useState(false)
  const [query, setQuery] = useState('')

  const sections = useMemo(() => syntaxReference
    .map(section => ({ ...section, items: section.items.filter(item => matches(item, query)) }))
    .filter(section => section.items.length > 0), [query])

  const insert = (item: SyntaxItem) => {
    setOpen(false)
    onInsert(item.insert ?? item.fratm)
  }

  return (
    <Dialog open={open} onOpenChange={(next) => { setOpen(next); if (!next) setQuery('') }}>
      <DialogTrigger
        render={
          <Button variant="outline" size="icon-sm" aria-label="Syntax Help">
            <HelpIcon className="size-3.5" />
          </Button>
        }
      />
      <DialogPopup className="flex flex-col max-w-2xl max-h-[80vh]">
        <DialogHeader>
          <DialogTitle>FratmScript syntax reference</DialogTitle>
          <DialogDescription>
            Neapolitan dialect to JavaScript translation. Click a row to insert it in the editor.
          </DialogDescription>
          <div className="relative mt-3">
            <Search className="top-1/2 left-2.5 absolute size-4 text-muted-foreground -translate-y-1/2 pointer-events-none" />
            <input
              type="search"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Search: if, stamm, console…"
              aria-label="Search the syntax reference"
              className="bg-secondary py-1.5 pr-3 pl-8 border border-input rounded-lg focus-visible:outline-2 focus-visible:outline-ring focus-visible:outline-offset-2 w-full text-sm"
              autoFocus
            />
          </div>
        </DialogHeader>
        <DialogPanel className="space-y-6">
          {sections.length === 0 && (
            <p className="py-6 text-muted-foreground text-sm text-center">Nothing matches “{query}”.</p>
          )}
          {sections.map((section) => (
            <div key={section.category}>
              <h3 className="mb-2 font-semibold text-foreground text-sm">{section.category}</h3>
              <div className="border rounded-md overflow-hidden">
                <table className="w-full text-sm">
                  <thead className="bg-muted/50">
                    <tr>
                      <th className="px-3 py-2 font-medium text-muted-foreground text-left">FratmScript</th>
                      <th className="px-3 py-2 font-medium text-muted-foreground text-left">JavaScript</th>
                      <th className="hidden sm:table-cell px-3 py-2 font-medium text-muted-foreground text-left">Description</th>
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
                            aria-label={`Insert ${item.fratm}`}
                          >
                            {item.fratm}
                          </button>
                        </td>
                        <td className="px-3 py-2 font-mono text-muted-foreground">{item.js}</td>
                        <td className="hidden sm:table-cell px-3 py-2 text-muted-foreground">{item.desc}</td>
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
