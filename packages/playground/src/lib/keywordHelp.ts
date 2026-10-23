// Keyword autocomplete and hover for the editor, driven by the same
// keyword table as the VS Code extension's language server.
import { autocompletion, snippetCompletion, type Completion, type CompletionContext } from '@codemirror/autocomplete'
import { hoverTooltip } from '@codemirror/view'
import { KEYWORDS, KEYWORD_MAP, type KeywordInfo } from '@keywords'

const WORD = /[\p{L}_$'][\p{L}\p{N}_$']*/u

// VS Code snippets end with `$0`; CodeMirror wants `${}` for the final cursor
const toCodeMirrorSnippet = (snippet: string) => snippet.replace(/\$0/g, '${}').replace(/\$(\d+)(?!\d|:)/g, '${$1}')

function completionFor(keyword: KeywordInfo): Completion {
  const base: Completion = {
    label: keyword.napoletano,
    detail: keyword.javascript,
    info: keyword.descrizione,
    type: keyword.categoria === 'valore' ? 'constant' : 'keyword',
  }
  return keyword.snippet ? snippetCompletion(toCodeMirrorSnippet(keyword.snippet), base) : base
}

const completions = KEYWORDS.map(completionFor)

function keywordCompletions(context: CompletionContext) {
  const word = context.matchBefore(new RegExp(WORD.source + '$', 'u'))
  if (!word || (word.from === word.to && !context.explicit)) return null
  // Don't pop up inside strings or comments
  const line = context.state.doc.lineAt(context.pos)
  const before = line.text.slice(0, context.pos - line.from)
  const insideQuotes = (quote: string) => (before.split(quote).length - 1) % 2 === 1
  if (before.includes('//') || insideQuotes('"') || insideQuotes('`')) return null
  return { from: word.from, options: completions, validFor: new RegExp('^' + WORD.source + '$', 'u') }
}

const keywordHover = hoverTooltip((view, pos, side) => {
  const line = view.state.doc.lineAt(pos)
  const text = line.text
  let start = pos - line.from
  let end = start
  const isWordChar = (c: string | undefined) => !!c && /[\p{L}\p{N}_$']/u.test(c)
  while (start > 0 && isWordChar(text[start - 1])) start--
  while (end < text.length && isWordChar(text[end])) end++
  if (start === end || (start === pos - line.from && side < 0) || (end === pos - line.from && side > 0)) return null
  const info = KEYWORD_MAP.get(text.slice(start, end))
  if (!info) return null
  return {
    pos: line.from + start,
    end: line.from + end,
    above: true,
    create() {
      const dom = document.createElement('div')
      dom.className = 'cm-keyword-hover'
      const head = dom.appendChild(document.createElement('div'))
      const name = head.appendChild(document.createElement('code'))
      name.textContent = info.napoletano
      head.appendChild(document.createTextNode(' → '))
      const js = head.appendChild(document.createElement('code'))
      js.textContent = info.javascript
      const desc = dom.appendChild(document.createElement('div'))
      desc.className = 'cm-keyword-hover-desc'
      desc.textContent = info.descrizione
      return { dom }
    },
  }
})

export const keywordHelp = [
  autocompletion({ override: [keywordCompletions], icons: false }),
  keywordHover,
]
