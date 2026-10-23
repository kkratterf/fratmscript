import { forwardRef, useEffect, useImperativeHandle, useRef } from 'react'
import { EditorState, Compartment, Prec, StateEffect, StateField } from '@codemirror/state'
import { EditorView, Decoration, type DecorationSet, keymap, highlightSpecialChars, drawSelection, highlightActiveLine, lineNumbers } from '@codemirror/view'
import { defaultKeymap, indentWithTab } from '@codemirror/commands'
import { lintGutter, setDiagnostics, type Diagnostic } from '@codemirror/lint'
import { HighlightStyle, syntaxHighlighting, StreamLanguage } from '@codemirror/language'
import { tags } from '@lezer/highlight'
import { useTheme } from 'next-themes'
import { cn } from '@/lib/utils'
import { keywordHelp } from '@/lib/keywordHelp'

// FratmScript language definition for StreamLanguage
const fratmScriptLanguage = StreamLanguage.define({
  token(stream) {
    // Skip whitespace
    if (stream.eatSpace()) return null

    // Comments
    if (stream.match('//')) {
      stream.skipToEnd()
      return 'comment'
    }

    // Strings
    if (stream.match(/"([^"\\]|\\.)*"/)) return 'string'
    if (stream.match(/'([^'\\]|\\.)*'/)) return 'string'

    // Numbers
    if (stream.match(/\d+\.?\d*/)) return 'number'

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
  },
})

// Dark theme colors using COSS palette
const darkHighlighting = HighlightStyle.define([
  { tag: tags.keyword, color: '#a78bfa' }, // purple-400
  { tag: tags.atom, color: '#60a5fa' }, // blue-400
  { tag: tags.number, color: '#fbbf24' }, // amber-400
  { tag: tags.string, color: '#34d399' }, // emerald-400
  { tag: tags.comment, color: '#6b7280', fontStyle: 'italic' }, // neutral-500
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
  { tag: tags.keyword, color: '#7c3aed' }, // purple-600
  { tag: tags.atom, color: '#2563eb' }, // blue-600
  { tag: tags.number, color: '#d97706' }, // amber-600
  { tag: tags.string, color: '#059669' }, // emerald-600
  { tag: tags.comment, color: '#6b7280', fontStyle: 'italic' }, // neutral-500
  { tag: tags.variableName, color: '#374151' }, // neutral-700
  { tag: tags.definition(tags.variableName), color: '#2563eb' }, // blue-600
  { tag: tags.operator, color: '#0891b2' }, // cyan-600
  { tag: tags.punctuation, color: '#6b7280' }, // neutral-500
  { tag: tags.function(tags.variableName), color: '#2563eb' }, // blue-600
  { tag: tags.propertyName, color: '#dc2626' }, // red-600
  { tag: tags.typeName, color: '#d97706' }, // amber-600
  { tag: tags.className, color: '#d97706' }, // amber-600
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
    color: '#6b7280', // neutral-500
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
    color: '#9ca3af', // neutral-400
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
}

interface EditorProps {
  value: string
  onChange: (value: string) => void
  onRun: () => void
  diagnostics?: EditorDiagnostic[]
  /** Called with the 1-based line of the cursor when it moves */
  onCursorLineChange?: (line: number) => void
  className?: string
}

// Compartments for dynamic theme switching
const themeCompartment = new Compartment()
const highlightCompartment = new Compartment()

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
  { value, onChange, onRun, diagnostics, onCursorLineChange, className },
  ref,
) {
  const containerRef = useRef<HTMLDivElement>(null)
  const viewRef = useRef<EditorView | null>(null)
  const { resolvedTheme } = useTheme()

  // Determine if dark mode
  const isDark = resolvedTheme === 'dark'

  // The editor is created once, so read the latest callback through a ref
  const onRunRef = useRef(onRun)
  onRunRef.current = onRun
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
  }), [])

  // Run on Mod-Enter. Highest precedence so defaultKeymap's insertBlankLine
  // binding for the same key never sees it.
  const runKeymap = Prec.highest(keymap.of([{
    key: 'Mod-Enter',
    run: () => {
      onRunRef.current()
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
        keymap.of([...defaultKeymap, indentWithTab]),
        lintGutter(),
        errorLines,
        keywordHelp,
        fratmScriptLanguage,
        highlightCompartment.of(syntaxHighlighting(isDark ? darkHighlighting : lightHighlighting)),
        themeCompartment.of(isDark ? darkTheme : lightTheme),
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
    <div
      ref={containerRef}
      className={cn('w-full h-full', className)}
    />
  )
})
