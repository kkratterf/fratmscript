import { forwardRef, useEffect, useImperativeHandle, useRef } from 'react'
import { EditorState, Compartment, Prec, StateEffect, StateField } from '@codemirror/state'
import { EditorView, Decoration, type DecorationSet, keymap, highlightSpecialChars, drawSelection, highlightActiveLine, lineNumbers } from '@codemirror/view'
import { defaultKeymap, indentWithTab, history, historyKeymap } from '@codemirror/commands'
import { lintGutter, setDiagnostics, type Diagnostic } from '@codemirror/lint'
import { HighlightStyle, syntaxHighlighting, StreamLanguage, type StringStream } from '@codemirror/language'
import { tags } from '@lezer/highlight'
import { useTheme } from 'next-themes'
import { cn } from '@/lib/utils'
import { keywordHelp, setKeywordLanguage } from '@/lib/keywordHelp'
import { useI18n } from '@/lib/i18n'
import { STILE_TAGS, STILE_VALUES, GOMORRA_CLAUSES, GOMORRA_OPERATORS, GOMORRA_STATEMENTS, GOMORRA_VALUES } from '@/lib/templateWords'

// FratmScript language definition for StreamLanguage.
// JSX and template literals are tracked with a stack of contexts: inside a
// tag (`<a href="…"`), among an element's children, in a `{ … }` or `${ … }`
// expression, or in a template literal (CSS for stile`…`, SQL for gomorra`…`).
type TemplateLanguage = 'stile' | 'gomorra' | null

type JsxContext =
  | { kind: 'tag' }
  | { kind: 'children' }
  | { kind: 'expr'; depth: number }
  | { kind: 'template'; language: TemplateLanguage }

interface FratmState {
  jsx: JsxContext[]
  /** The last token ends a value (name, number, `)`), so `<` compares */
  afterValue: boolean
  /** The tag of the template literal that starts at the next backtick */
  templateTag: TemplateLanguage
}

/** A regex that matches one of `words` as a whole word, longest first */
function wordsPattern(words: string[]): RegExp {
  const escaped = [...words]
    .sort((a, b) => b.length - a.length)
    .map(word => word.replace(/[.*+?^${}()|[\]\\]/g, '\\$&').replace(/ /g, '\\s+'))
  return new RegExp(`^(?:${escaped.join('|')})(?![\\p{L}\\p{N}_$-])`, 'u')
}

const STILE_VALUE = wordsPattern(STILE_VALUES)
// A tag is a selector only when a selector or a block follows it
const STILE_TAG = new RegExp(`${wordsPattern(STILE_TAGS).source}(?=\\s*(?:[{,>+~\\[]|[.#][\\p{L}_-]|::?\\p{L})|\\s+[^:;\`]*\\{)`, 'u')
const GOMORRA_STATEMENT = wordsPattern([...GOMORRA_STATEMENTS, ...GOMORRA_CLAUSES])
const GOMORRA_OPERATOR = wordsPattern(GOMORRA_OPERATORS)
const GOMORRA_VALUE = wordsPattern(GOMORRA_VALUES)
const SQL_KEYWORD = /^(?:select|from|where|insert|into|update|set|delete|create|table|drop|if|exists|join|left|inner|on|order|group|by|limit|offset|values|and|or|not|null|as|like|in|is|asc|desc|begin|commit|rollback|primary|key|integer|text|real|default)(?![\p{L}\p{N}_$-])/iu
const CSS_NAME = /^-{0,2}[\p{L}_][\p{L}\p{N}_-]*/u
// A property is a name followed by `:` and a space (`buttone:passanno` is a selector)
const CSS_PROPERTY = new RegExp(`${CSS_NAME.source}(?=\\s*:(?:\\s|$))`, 'u')

const JSX_NAME = /^[\p{L}_$][\p{L}\p{N}_$.:-]*/u

/** Tokens of plain FratmScript code (also inside JSX `{ … }`). */
function codeToken(stream: StringStream, state: FratmState): string | null {
  // Comments
  if (stream.match('//')) {
    stream.skipToEnd()
    return 'comment'
  }

  // Template literals: stile`…` and gomorra`…` get their own colours
  const tagged = stream.match(/^(stile|gomorra)(?=\s*`)/) as RegExpMatchArray | null
  if (tagged) {
    state.templateTag = tagged[1] as TemplateLanguage
    return 'variableName.function'
  }
  if (stream.eat('`')) {
    state.jsx.push({ kind: 'template', language: state.templateTag })
    state.templateTag = null
    return 'string'
  }
  state.templateTag = null

  // Strings
  if (stream.match(/"([^"\\]|\\.)*"/)) return 'string'
  if (stream.match(/'([^'\\]|\\.)*'/)) return 'string'

  // Numbers
  if (stream.match(/\d+\.?\d*/)) return 'number'

  // JSX: `<` followed by a tag name or `>`, where an expression can start
  if (!state.afterValue && stream.match(/^<(?=[\p{L}_$>])/u, false)) {
    stream.next()
    state.jsx.push({ kind: 'tag' })
    return 'angleBracket'
  }

  // Keywords - FratmScript specific
  if (stream.match(/\b(chist|è|tien|facc|piglie|si|sinnò|sinno|pe|ogni|mentre|che)\b/)) return 'keyword'
  if (stream.match(/\b(overo|sfòls|sfols|nisciun|boh)\b/)) return 'atom'
  if (stream.match(/\b(stamm|dì|di|avvis|scrive)\b/)) return 'builtin'
  if (stream.match(/\b(mo|vir|aspett)\b/)) return 'keyword'
  if (stream.match(/\b(pruvamm|schiatta|aropp|iett)\b/)) return 'keyword'
  if (stream.match(/\b(vedimm|caso|primm|fa|dint)\b/)) return 'keyword'
  if (stream.match(/\b(damme|miette|fisso|figlio)\b/)) return 'keyword'
  if (stream.match(/\b(nu|bell|na|famiglie|stu|cos)\b/)) return 'keyword'
  if (stream.match(/\b(chiamm|da|mann|for|predefinit)\b/)) return 'keyword'
  if (stream.match(/\b(rompe|salta|fermete)\b/)) return 'keyword'
  if (stream.match(/\b(e|o|no|manco|pure)\b/)) return 'operator'
  if (stream.match(/\b(leva|caccia)\b/)) return 'keyword'
  if (stream.match(/\b(costruttore)\b/)) return 'def'

  // Operators
  if (stream.match(/[+\-*/%=<>!&|?:]+/)) return 'operator'

  // Punctuation
  if (stream.match(/[{}()\[\],;.]/)) return 'punctuation'

  // Identifiers
  if (stream.match(/[a-zA-Z_àèéìòù][a-zA-Z0-9_àèéìòù]*/)) return 'variable'

  // Skip unknown character
  stream.next()
  return null
}

/** Tokens of a template literal, until the closing backtick. */
function templateToken(stream: StringStream, state: FratmState, language: TemplateLanguage): string | null {
  if (stream.eat('`')) {
    state.jsx.pop()
    state.afterValue = true
    return 'string'
  }
  if (stream.match('${')) {
    state.jsx.push({ kind: 'expr', depth: 0 })
    state.afterValue = false
    return 'brace'
  }
  if (stream.match(/^\\./)) return 'string'
  if (language === null) {
    stream.match(/^(?:[^`$\\]|\$(?!\{))+/) || stream.next()
    return 'string'
  }
  if (stream.eatSpace()) return null
  if (language === 'stile') return stileToken(stream)
  return gomorraToken(stream)
}

/** CSS in Neapolitan inside stile`…` */
function stileToken(stream: StringStream): string | null {
  if (stream.match(/^\/\*(?:[^*`]|\*(?!\/))*(?:\*\/)?/)) return 'comment'
  if (stream.match(/^@[\p{L}_-][\p{L}\p{N}_-]*/u)) return 'keyword'
  if (stream.match(/^"[^"`]*"/) || stream.match(/^'[^'`]*'/)) return 'string'
  if (stream.match(CSS_PROPERTY)) return 'propertyName'
  if (stream.match(/^::?[\p{L}_-][\p{L}\p{N}_-]*/u)) return 'keyword'
  if (stream.match(/^#[0-9a-fA-F]{3,8}(?![\p{L}\p{N}_-])/u)) return 'number'
  if (stream.match(/^[#.][\p{L}_-][\p{L}\p{N}_-]*/u)) return 'className'
  if (stream.match(/^-?(?:\d+\.?\d*|\.\d+)(?:[a-zA-Z]+|%)?/)) return 'number'
  if (stream.match(STILE_TAG)) return 'tagName'
  if (stream.match(STILE_VALUE)) return 'atom'
  if (stream.match(CSS_NAME)) return null
  stream.next()
  return 'punctuation'
}

/** GomorraSQL inside gomorra`…` */
function gomorraToken(stream: StringStream): string | null {
  if (stream.match(/^--(?:[^`$]|\$(?!\{))*/)) return 'comment'
  if (stream.match(/^"[^"`]*"/)) return 'string'
  if (stream.match(GOMORRA_STATEMENT)) return 'keyword'
  if (stream.match(GOMORRA_OPERATOR)) return 'operator'
  if (stream.match(GOMORRA_VALUE)) return 'atom'
  if (stream.match(SQL_KEYWORD)) return 'keyword'
  if (stream.match(/^\d+(?:\.\d+)?/)) return 'number'
  if (stream.match(/^(?:<=|>=|<>|!=|=|<|>)/)) return 'operator'
  if (stream.match(/^[\p{L}_'][\p{L}\p{N}_']*/u)) return null
  stream.next()
  return 'punctuation'
}

/** Tokens inside `<tag …>`: name, attributes, values, until `>` or `/>`. */
function tagToken(stream: StringStream, state: FratmState): string | null {
  if (stream.match('/>')) {
    state.jsx.pop()
    return 'angleBracket'
  }
  if (stream.eat('>')) {
    state.jsx[state.jsx.length - 1] = { kind: 'children' }
    return 'angleBracket'
  }
  if (stream.eat('{')) {
    state.jsx.push({ kind: 'expr', depth: 0 })
    return 'brace'
  }
  if (stream.match(/"[^"]*"/) || stream.match(/'[^']*'/)) return 'string'
  if (stream.eat('=')) return 'operator'
  const name = stream.match(JSX_NAME) as RegExpMatchArray | null
  if (name) {
    // The tag name comes right after `<`; attribute names come later
    const start = stream.string.slice(0, stream.start).trimEnd()
    if (start.endsWith('<')) return /^[A-Z]|\./.test(name[0]) ? 'typeName' : 'tagName'
    return 'attributeName'
  }
  stream.next()
  return null
}

/** Tokens among an element's children: text, `{ … }`, nested and closing tags. */
function childrenToken(stream: StringStream, state: FratmState): string | null {
  if (stream.match(/^<\/[\p{L}\p{N}_$.:-]*\s*>/u)) {
    // The closing tag ends the children and the element
    state.jsx.pop()
    return 'tagName'
  }
  if (stream.match(/^<(?=[\p{L}_$>])/u)) {
    state.jsx.push({ kind: 'tag' })
    return 'angleBracket'
  }
  if (stream.eat('{')) {
    state.jsx.push({ kind: 'expr', depth: 0 })
    return 'brace'
  }
  if (stream.match(/&(?:[a-zA-Z][a-zA-Z0-9]*|#\d+|#x[0-9a-fA-F]+);/)) return 'character'
  stream.match(/^[^<{&]+/) || stream.next()
  return 'content'
}

const fratmScriptLanguage = StreamLanguage.define<FratmState>({
  startState: () => ({ jsx: [], afterValue: false, templateTag: null }),
  copyState: state => ({ jsx: state.jsx.map(context => ({ ...context })), afterValue: state.afterValue, templateTag: state.templateTag }),
  token(stream, state) {
    const context = state.jsx[state.jsx.length - 1]
    if (context?.kind === 'children') return childrenToken(stream, state)
    if (context?.kind === 'template') return templateToken(stream, state, context.language)
    // Skip whitespace
    if (stream.eatSpace()) return null
    if (context?.kind === 'tag') return tagToken(stream, state)
    if (context?.kind === 'expr') {
      if (stream.peek() === '}' && context.depth === 0) {
        stream.next()
        state.jsx.pop()
        state.afterValue = false
        return 'brace'
      }
      if (stream.peek() === '{') context.depth++
      if (stream.peek() === '}') context.depth--
    }
    const style = codeToken(stream, state)
    const text = stream.current()
    if (style !== 'comment' && style !== 'angleBracket' && state.jsx[state.jsx.length - 1]?.kind !== 'template') {
      state.afterValue = style === 'variable' || style === 'number' || style === 'string' || style === 'atom' || /^[)\]]$/.test(text)
    }
    return style
  },
})

// Dark theme colors using COSS palette
const darkHighlighting = HighlightStyle.define([
  { tag: tags.tagName, color: '#f87171' },
  { tag: tags.attributeName, color: '#fbbf24' },
  { tag: tags.angleBracket, color: '#9ca3af' },
  { tag: tags.character, color: '#fbbf24' },
  { tag: tags.keyword, color: '#a78bfa' }, // purple-400
  { tag: tags.atom, color: '#60a5fa' }, // blue-400
  { tag: tags.number, color: '#fbbf24' }, // amber-400
  { tag: tags.string, color: '#34d399' }, // emerald-400
  { tag: tags.comment, color: '#9ca3af', fontStyle: 'italic' }, // neutral-400
  { tag: tags.variableName, color: '#e5e7eb' }, // neutral-200
  { tag: tags.definition(tags.variableName), color: '#60a5fa' }, // blue-400
  { tag: tags.operator, color: '#22d3ee' }, // cyan-400
  { tag: tags.punctuation, color: '#9ca3af' }, // neutral-400
  { tag: tags.function(tags.variableName), color: '#60a5fa' }, // blue-400
  { tag: tags.propertyName, color: '#f87171' }, // red-400
  { tag: tags.typeName, color: '#fbbf24' }, // amber-400
  { tag: tags.className, color: '#fbbf24' }, // amber-400
  { tag: tags.meta, color: '#a78bfa' }, // purple-400
  { tag: tags.name, color: '#e5e7eb' }, // neutral-200
])

// Light theme colors using COSS palette
const lightHighlighting = HighlightStyle.define([
  { tag: tags.tagName, color: '#dc2626' },
  { tag: tags.attributeName, color: '#b45309' },
  { tag: tags.angleBracket, color: '#6b7280' },
  { tag: tags.character, color: '#b45309' },
  { tag: tags.keyword, color: '#7c3aed' }, // purple-600
  { tag: tags.atom, color: '#2563eb' }, // blue-600
  { tag: tags.number, color: '#b45309' }, // amber-700
  { tag: tags.string, color: '#047857' }, // emerald-700
  { tag: tags.comment, color: '#6b7280', fontStyle: 'italic' }, // neutral-500
  { tag: tags.variableName, color: '#374151' }, // neutral-700
  { tag: tags.definition(tags.variableName), color: '#2563eb' }, // blue-600
  { tag: tags.operator, color: '#0e7490' }, // cyan-700
  { tag: tags.punctuation, color: '#6b7280' }, // neutral-500
  { tag: tags.function(tags.variableName), color: '#2563eb' }, // blue-600
  { tag: tags.propertyName, color: '#dc2626' }, // red-600
  { tag: tags.typeName, color: '#b45309' }, // amber-700
  { tag: tags.className, color: '#b45309' }, // amber-700
  { tag: tags.meta, color: '#7c3aed' }, // purple-600
  { tag: tags.name, color: '#374151' }, // neutral-700
])

// Dark theme for CodeMirror using COSS palette
const darkTheme = EditorView.theme({
  '&': {
    backgroundColor: 'transparent',
    color: '#e5e7eb', // neutral-200
    height: '100%',
  },
  '.cm-content': {
    fontFamily: '"Fira Code", "SF Mono", "Monaco", "Inconsolata", monospace',
    fontSize: '14px',
    lineHeight: '1.6',
    padding: '16px',
    caretColor: '#ef4444', // red-500 (primary)
  },
  '.cm-cursor': {
    borderLeftColor: '#ef4444', // red-500
    borderLeftWidth: '2px',
  },
  '.cm-activeLine': {
    backgroundColor: 'rgba(255, 255, 255, 0.04)',
  },
  '.cm-selectionBackground, ::selection': {
    backgroundColor: 'rgba(239, 68, 68, 0.2) !important', // red-500/20
  },
  '.cm-focused .cm-selectionBackground': {
    backgroundColor: 'rgba(239, 68, 68, 0.3) !important', // red-500/30
  },
  '.cm-gutters': {
    backgroundColor: 'transparent',
    borderRight: '1px solid rgba(255, 255, 255, 0.08)',
    color: '#9ca3af', // neutral-400
  },
  '.cm-lineNumbers .cm-gutterElement': {
    padding: '0 12px 0 8px',
    minWidth: '40px',
  },
  '.cm-activeLineGutter': {
    backgroundColor: 'rgba(255, 255, 255, 0.04)',
  },
  '.cm-scroller': {
    overflow: 'auto',
  },
  '.cm-lintRange-error': {
    backgroundImage: 'none',
    textDecoration: 'underline wavy #f87171',
    textUnderlineOffset: '3px',
  },
  '.cm-inspectRange': {
    backgroundColor: 'rgba(250, 204, 21, 0.25)', // yellow-400/25
    borderRadius: '2px',
  },
  '.cm-errorLine': {
    backgroundColor: 'rgba(248, 113, 113, 0.12)',
  },
  '.cm-tooltip': {
    backgroundColor: '#171717',
    border: '1px solid rgba(255, 255, 255, 0.1)',
    borderRadius: '6px',
    color: '#e5e7eb',
  },
  '.cm-tooltip-autocomplete > ul > li[aria-selected]': {
    backgroundColor: 'rgba(239, 68, 68, 0.25)',
    color: '#f5f5f5',
  },
  '.cm-completionDetail': {
    color: '#9ca3af',
    fontStyle: 'normal',
    marginLeft: '0.75em',
  },
  '.cm-keyword-hover': {
    padding: '6px 10px',
    maxWidth: '320px',
    fontSize: '13px',
  },
  '.cm-keyword-hover code': {
    color: '#a78bfa',
  },
  '.cm-keyword-hover-desc': {
    marginTop: '2px',
    color: '#9ca3af',
  },
}, { dark: true })

// Light theme for CodeMirror using COSS palette
const lightTheme = EditorView.theme({
  '&': {
    backgroundColor: 'transparent',
    color: '#374151', // neutral-700
    height: '100%',
  },
  '.cm-content': {
    fontFamily: '"Fira Code", "SF Mono", "Monaco", "Inconsolata", monospace',
    fontSize: '14px',
    lineHeight: '1.6',
    padding: '16px',
    caretColor: '#dc2626', // red-600 (primary)
  },
  '.cm-cursor': {
    borderLeftColor: '#dc2626', // red-600
    borderLeftWidth: '2px',
  },
  '.cm-activeLine': {
    backgroundColor: 'rgba(0, 0, 0, 0.03)',
  },
  '.cm-selectionBackground, ::selection': {
    backgroundColor: 'rgba(220, 38, 38, 0.15) !important', // red-600/15
  },
  '.cm-focused .cm-selectionBackground': {
    backgroundColor: 'rgba(220, 38, 38, 0.25) !important', // red-600/25
  },
  '.cm-gutters': {
    backgroundColor: 'transparent',
    borderRight: '1px solid rgba(0, 0, 0, 0.08)',
    color: '#6b7280', // neutral-500
  },
  '.cm-lineNumbers .cm-gutterElement': {
    padding: '0 12px 0 8px',
    minWidth: '40px',
  },
  '.cm-activeLineGutter': {
    backgroundColor: 'rgba(0, 0, 0, 0.03)',
  },
  '.cm-scroller': {
    overflow: 'auto',
  },
  '.cm-lintRange-error': {
    backgroundImage: 'none',
    textDecoration: 'underline wavy #dc2626',
    textUnderlineOffset: '3px',
  },
  '.cm-inspectRange': {
    backgroundColor: 'rgba(250, 204, 21, 0.25)', // yellow-400/25
    borderRadius: '2px',
  },
  '.cm-errorLine': {
    backgroundColor: 'rgba(220, 38, 38, 0.08)',
  },
  '.cm-tooltip': {
    backgroundColor: '#ffffff',
    border: '1px solid rgba(0, 0, 0, 0.1)',
    borderRadius: '6px',
    color: '#374151',
  },
  '.cm-tooltip-autocomplete > ul > li[aria-selected]': {
    backgroundColor: 'rgba(220, 38, 38, 0.12)',
    color: '#111827',
  },
  '.cm-completionDetail': {
    color: '#6b7280',
    fontStyle: 'normal',
    marginLeft: '0.75em',
  },
  '.cm-keyword-hover': {
    padding: '6px 10px',
    maxWidth: '320px',
    fontSize: '13px',
  },
  '.cm-keyword-hover code': {
    color: '#7c3aed',
  },
  '.cm-keyword-hover-desc': {
    marginTop: '2px',
    color: '#6b7280',
  },
}, { dark: false })

export interface EditorDiagnostic {
  /** 1-based */
  line: number
  /** 1-based; when missing the whole line is marked */
  column?: number
  message: string
}

export interface EditorHandle {
  /** Moves the cursor to a 1-based line/column and focuses the editor */
  revealLine: (line: number, column?: number) => void
  /** Replaces the selection with text and focuses the editor */
  insertText: (text: string) => void
  /** Replaces the whole document as one undoable change */
  replaceAll: (text: string) => void
  /** Highlights a source range (UTF-16 offsets), or clears it with null */
  highlightRange: (range: { from: number; to: number } | null) => void
}

interface EditorProps {
  value: string
  onChange: (value: string) => void
  onRun: () => void
  /** Called on Shift-Alt-F */
  onFormat?: () => void
  diagnostics?: EditorDiagnostic[]
  /** Called with the 1-based line of the cursor when it moves */
  onCursorLineChange?: (line: number) => void
  className?: string
}

// Compartments for dynamic theme switching
const themeCompartment = new Compartment()
const highlightCompartment = new Compartment()
const labelCompartment = new Compartment()

const KEYBOARD_HINT_ID = 'editor-keyboard-hint'

/** Accessible name of the editor, plus a pointer to the hint on leaving it. */
function contentLabel(label: string) {
  return EditorView.contentAttributes.of({ 'aria-label': label, 'aria-describedby': KEYBOARD_HINT_ID })
}

// Line background for lines with an error
const errorLineDecoration = Decoration.line({ class: 'cm-errorLine' })
const setErrorLines = StateEffect.define<number[]>()
const errorLines = StateField.define<DecorationSet>({
  create: () => Decoration.none,
  update(deco, tr) {
    deco = deco.map(tr.changes)
    for (const effect of tr.effects) {
      if (effect.is(setErrorLines)) {
        const lines = [...new Set(effect.value)].sort((a, b) => a - b)
        deco = Decoration.set(lines.map(pos => errorLineDecoration.range(pos)))
      }
    }
    return deco
  },
  provide: field => EditorView.decorations.from(field),
})

// Source range under the pointer in the Token and Tree views
const rangeMark = Decoration.mark({ class: 'cm-inspectRange' })
const setInspectRange = StateEffect.define<{ from: number; to: number } | null>()
const inspectRange = StateField.define<DecorationSet>({
  create: () => Decoration.none,
  update(deco, tr) {
    deco = deco.map(tr.changes)
    for (const effect of tr.effects) {
      if (effect.is(setInspectRange)) {
        const range = effect.value
        const length = tr.state.doc.length
        deco = range && range.to > range.from && range.to <= length
          ? Decoration.set([rangeMark.range(range.from, range.to)])
          : Decoration.none
      }
    }
    return deco
  },
  provide: field => EditorView.decorations.from(field),
})

function toCodeMirror(doc: EditorState['doc'], diagnostics: EditorDiagnostic[]) {
  const result: Diagnostic[] = []
  const lineStarts: number[] = []
  for (const d of diagnostics) {
    if (d.line < 1 || d.line > doc.lines) continue
    const line = doc.line(d.line)
    lineStarts.push(line.from)
    let from = line.from
    let to = line.to
    if (d.column !== undefined) {
      // Underline the word at the column, or the last character when the
      // error points past the end of the line.
      from = Math.min(line.from + Math.max(d.column - 1, 0), line.to)
      const rest = line.text.slice(from - line.from)
      const word = rest.match(/^[\p{L}\p{N}_$]+/u)?.[0] ?? rest.slice(0, 1)
      to = from + word.length
      if (to === from && from > line.from) from -= 1
    }
    result.push({ from, to, severity: 'error', message: d.message })
  }
  return { result, lineStarts }
}

export const Editor = forwardRef<EditorHandle, EditorProps>(function Editor(
  { value, onChange, onRun, onFormat, diagnostics, onCursorLineChange, className },
  ref,
) {
  const containerRef = useRef<HTMLDivElement>(null)
  const viewRef = useRef<EditorView | null>(null)
  const { resolvedTheme } = useTheme()
  const { lang, t } = useI18n()
  setKeywordLanguage(lang)
  const label = t('editor.label')

  // Determine if dark mode
  const isDark = resolvedTheme === 'dark'

  // The editor is created once, so read the latest callback through a ref
  const onRunRef = useRef(onRun)
  onRunRef.current = onRun
  const onFormatRef = useRef(onFormat)
  onFormatRef.current = onFormat
  const onCursorLineRef = useRef(onCursorLineChange)
  onCursorLineRef.current = onCursorLineChange

  useImperativeHandle(ref, () => ({
    revealLine(lineNumber, column) {
      const view = viewRef.current
      if (!view) return
      const doc = view.state.doc
      const line = doc.line(Math.min(Math.max(lineNumber, 1), doc.lines))
      const pos = Math.min(line.from + Math.max((column ?? 1) - 1, 0), line.to)
      view.dispatch({
        selection: { anchor: pos },
        effects: EditorView.scrollIntoView(pos, { y: 'center' }),
      })
      view.focus()
    },
    insertText(text) {
      const view = viewRef.current
      if (!view) return
      view.dispatch(view.state.replaceSelection(text), { scrollIntoView: true })
      view.focus()
    },
    highlightRange(range) {
      const view = viewRef.current
      if (!view) return
      const effects: StateEffect<unknown>[] = [setInspectRange.of(range)]
      if (range && range.to <= view.state.doc.length) effects.push(EditorView.scrollIntoView(range.from, { y: 'nearest' }))
      view.dispatch({ effects })
    },
    replaceAll(text) {
      const view = viewRef.current
      if (!view) return
      const doc = view.state.doc
      // Keep the cursor on the same line when the line count is unchanged
      const head = view.state.selection.main.head
      const line = doc.lineAt(head).number
      view.dispatch({ changes: { from: 0, to: doc.length, insert: text } })
      const next = view.state.doc
      const target = next.line(Math.min(line, next.lines))
      view.dispatch({ selection: { anchor: Math.min(target.from + (head - doc.line(line).from), target.to) } })
      view.focus()
    },
  }), [])

  // Run on Mod-Enter. Highest precedence so defaultKeymap's insertBlankLine
  // binding for the same key never sees it.
  const runKeymap = Prec.highest(keymap.of([{
    key: 'Mod-Enter',
    run: () => {
      onRunRef.current()
      return true
    },
  }, {
    key: 'Shift-Alt-f',
    run: () => {
      onFormatRef.current?.()
      return true
    },
  }]))
  useEffect(() => {
    if (!containerRef.current) return

    const state = EditorState.create({
      doc: value,
      extensions: [
        lineNumbers(),
        highlightActiveLine(),
        highlightSpecialChars(),
        drawSelection(),
        EditorView.lineWrapping,
        runKeymap,
        history(),
        keymap.of([...defaultKeymap, ...historyKeymap, indentWithTab]),
        lintGutter(),
        errorLines,
        inspectRange,
        keywordHelp,
        fratmScriptLanguage,
        highlightCompartment.of(syntaxHighlighting(isDark ? darkHighlighting : lightHighlighting)),
        themeCompartment.of(isDark ? darkTheme : lightTheme),
        labelCompartment.of(contentLabel(label)),
        EditorView.updateListener.of((update) => {
          if (update.docChanged) {
            onChange(update.state.doc.toString())
          }
          if (update.selectionSet || update.docChanged) {
            const state = update.state
            onCursorLineRef.current?.(state.doc.lineAt(state.selection.main.head).number)
          }
        }),
      ],
    })

    const view = new EditorView({
      state,
      parent: containerRef.current,
    })

    viewRef.current = view

    return () => {
      view.destroy()
    }
  }, []) // Only run once on mount

  // Update theme when it changes
  useEffect(() => {
    if (viewRef.current) {
      viewRef.current.dispatch({
        effects: [
          themeCompartment.reconfigure(isDark ? darkTheme : lightTheme),
          highlightCompartment.reconfigure(syntaxHighlighting(isDark ? darkHighlighting : lightHighlighting)),
        ],
      })
    }
  }, [isDark])

  // Follow the UI language
  useEffect(() => {
    viewRef.current?.dispatch({ effects: labelCompartment.reconfigure(contentLabel(label)) })
  }, [label])

  // Update editor content when value changes externally (e.g., loading examples)
  useEffect(() => {
    if (viewRef.current) {
      const currentValue = viewRef.current.state.doc.toString()
      if (currentValue !== value) {
        viewRef.current.dispatch({
          changes: {
            from: 0,
            to: currentValue.length,
            insert: value,
          },
        })
      }
    }
  }, [value])

  // Show errors inline: underline, gutter marker, hover message, line tint
  useEffect(() => {
    const view = viewRef.current
    if (!view) return
    const { result, lineStarts } = toCodeMirror(view.state.doc, diagnostics ?? [])
    view.dispatch(setDiagnostics(view.state, result))
    view.dispatch({ effects: setErrorLines.of(lineStarts) })
  }, [diagnostics])

  return (
    <>
      <div
        ref={containerRef}
        className={cn('w-full h-full', className)}
      />
      {/* Tab indents inside the editor: tell keyboard users how to leave it */}
      <span id={KEYBOARD_HINT_ID} className="sr-only">{t('editor.keyboardHint')}</span>
    </>
  )
})
