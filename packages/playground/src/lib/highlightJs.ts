import { parser } from '@lezer/javascript'
import { highlightCode, tagHighlighter, tags } from '@lezer/highlight'

/** A piece of a line of code and the CSS class of its colour ('' for none). */
export interface Token {
  text: string
  cls: string
}

// Same roles as the editor's highlight styles (Editor.tsx); the colours of
// these classes live in index.css, for the light and the dark theme
const highlighter = tagHighlighter([
  { tag: tags.keyword, class: 'tok-keyword' },
  { tag: [tags.atom, tags.bool, tags.null], class: 'tok-atom' },
  { tag: tags.number, class: 'tok-number' },
  { tag: [tags.string, tags.special(tags.string), tags.regexp], class: 'tok-string' },
  { tag: tags.comment, class: 'tok-comment' },
  { tag: [tags.definition(tags.variableName), tags.function(tags.variableName)], class: 'tok-function' },
  { tag: tags.variableName, class: 'tok-variable' },
  { tag: [tags.propertyName, tags.definition(tags.propertyName)], class: 'tok-property' },
  { tag: tags.operator, class: 'tok-operator' },
  { tag: tags.punctuation, class: 'tok-punctuation' },
  { tag: [tags.className, tags.typeName], class: 'tok-type' },
])

/** `code` split into lines of coloured tokens, parsed as a JS module with JSX. */
export function highlightJs(code: string): Token[][] {
  const lines: Token[][] = [[]]
  const tree = parser.configure({ dialect: 'jsx' }).parse(code)
  highlightCode(
    code,
    tree,
    highlighter,
    (text, cls) => lines[lines.length - 1].push({ text, cls }),
    () => lines.push([]),
  )
  return lines
}
