// The example picker lists the programs in the repo's examples/ folder, so the
// playground always shows the same code that CI compiles and golden-tests.

import type { Lang } from '@/lib/i18n'

const sources = import.meta.glob('../../../../examples/*.fratm', {
  query: '?raw',
  import: 'default',
  eager: true,
}) as Record<string, string>

/**
 * Examples that cannot run in the playground console: 09 imports Node modules
 * (node:path, node:os), 13 reads the terminal with readline and 16 is a React
 * component (it needs the component preview, #170).
 */
const NODE_ONLY = new Set(['09_moduli', '13_pomodor_e_muzzarel', '16_pizzeria_jsx'])

/** Picker names in each UI language; an example missing here uses its file's title comment. */
const TITLES: Record<string, Record<Lang, string>> = {
  '01_salutatore': { it: 'Saluto napoletano', en: 'Neapolitan greeter', nap: "'O saluto" },
  '02_variabili_matematica': { it: 'Variabili e matematica', en: 'Variables and math', nap: 'Variabili e cunte' },
  '03_condizionali': { it: 'Condizioni', en: 'Conditionals', nap: 'Si e sinnò' },
  '04_loop': { it: 'Cicli', en: 'Loops', nap: 'Cicli' },
  '05_array_oggetti': { it: 'Array e oggetti', en: 'Arrays and objects', nap: 'Liste e oggetti' },
  '06_funzioni': { it: 'Funzioni avanzate', en: 'Advanced functions', nap: 'Funzioni cchiù toste' },
  '07_async': { it: 'Async/await e try/catch', en: 'Async/await and try/catch', nap: 'Mo vir, aspett e pruvamm' },
  '08_classi': { it: 'Classi', en: 'Classes', nap: 'Famiglie' },
  '10_nuove_feature': { it: 'Novità di FratmScript', en: 'New FratmScript features', nap: "'E novità 'e FratmScript" },
  '11_oop_avanzato': { it: 'OOP avanzata', en: 'Advanced OOP', nap: 'Famiglie cchiù toste' },
  '12_trycatch_validazione': { it: 'Try/catch e validazione', en: 'Try/catch and validation', nap: 'Pruvamm e cuntrolle' },
  '14_sintassi_moderna': { it: 'Sintassi moderna', en: 'Modern syntax', nap: 'Sintassi moderna' },
}

export interface Example {
  /** File name without extension, e.g. `01_salutatore` */
  id: string
  /** Picker label in each UI language */
  title: Record<Lang, string>
  code: string
}

/** Title from the header comment (`// Example 1: Neapolitan Greeter`). */
function exampleTitle(id: string, code: string): string {
  const number = id.split('_')[0]
  const header = code
    .split('\n')
    .map(line => line.replace(/^\/\/\s*/, '').trim())
    .find(line => /^Example \d+:/.test(line))
  const name = header ? header.replace(/^Example \d+:\s*/, '') : id.slice(number.length + 1).replace(/_/g, ' ')
  return `${number} · ${name}`
}

export const exampleList: Example[] = Object.entries(sources)
  .map(([path, code]) => {
    const id = path.split('/').pop()!.replace(/\.fratm$/, '')
    const fallback = exampleTitle(id, code)
    const number = id.split('_')[0]
    const names = TITLES[id]
    const title = names
      ? { it: `${number} · ${names.it}`, en: `${number} · ${names.en}`, nap: `${number} · ${names.nap}` }
      : { it: fallback, en: fallback, nap: fallback }
    return { id, title, code }
  })
  .filter(example => !NODE_ONLY.has(example.id))
  .sort((a, b) => a.id.localeCompare(b.id))

export const examples: Record<string, string> = Object.fromEntries(
  exampleList.map(example => [example.id, example.code]),
)

export const defaultCode = `// Uè! Prova FratmScript

chist è nome = "Gennaro"

facc saluta(chi) {
    si (chi == nisciun) {
        piglie "E chi si tu?"
    }
    piglie "Ue " + chi + ", comme staje?"
}

stamm a dì(saluta(nome))
stamm a dì(saluta("Ciro"))
stamm a dì(saluta(nisciun))`
