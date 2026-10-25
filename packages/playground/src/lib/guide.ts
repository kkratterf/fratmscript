// The language guide: sections of prose with runnable snippets.
//
// Content lives in src/guida/: guida.json holds the sections in every UI
// language, and each snippet is a .fratm file with the .out file it prints.
// Snippets named *.sbagliato.fratm are meant to fail and have no .out.
// CI checks every snippet against its .out, and the agent reference
// (llms-full.txt) reads the English text from the same files, so keep the
// keys of guida.json (id, title, body, snippets) stable.

import type { Lang } from '@/lib/i18n'
import data from '../guida/guida.json'

const sources = import.meta.glob('../guida/*.fratm', { query: '?raw', import: 'default', eager: true }) as Record<string, string>

export interface GuideSnippet {
  /** File name, e.g. `05_cicli.fratm` */
  file: string
  code: string
}

export interface GuideSection {
  id: string
  title: Record<Lang, string>
  /** Paragraphs; `code` in backticks is shown as code */
  body: Record<Lang, string[]>
  snippets: GuideSnippet[]
}

function snippet(file: string): GuideSnippet {
  const code = sources[`../guida/${file}`]
  if (code === undefined) throw new Error(`guida.json names a missing snippet: ${file}`)
  return { file, code }
}

export const guide: GuideSection[] = data.sections.map(section => ({
  id: section.id,
  title: section.title,
  body: section.body,
  snippets: section.snippets.map(snippet),
}))
