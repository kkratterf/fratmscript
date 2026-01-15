import { useEffect, useRef } from 'react'
import { EditorState } from '@codemirror/state'
import { EditorView, keymap, highlightSpecialChars, drawSelection, highlightActiveLine, lineNumbers } from '@codemirror/view'
import { defaultKeymap, indentWithTab } from '@codemirror/commands'
import { HighlightStyle, syntaxHighlighting, StreamLanguage } from '@codemirror/language'
import { tags } from '@lezer/highlight'
import { cn } from '@/lib/utils'

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
    if (stream.match(/\b(chist|è|tien|facc|piglie|si|sinnò|pe|ogni|mentre|che)\b/)) return 'keyword'
    if (stream.match(/\b(overo|sfòls|nisciun|boh)\b/)) return 'atom'
    if (stream.match(/\b(stamm|dì|avvis|scrive)\b/)) return 'builtin'
    if (stream.match(/\b(mo|vir|aspett)\b/)) return 'keyword'
    if (stream.match(/\b(pruvamm|schiatta|iett)\b/)) return 'keyword'
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

// Custom theme colors matching COSS UI dark theme
const fratmScriptHighlighting = HighlightStyle.define([
  { tag: tags.keyword, color: '#f472b6' }, // pink for keywords
  { tag: tags.atom, color: '#c084fc' }, // purple for booleans/null
  { tag: tags.number, color: '#fbbf24' }, // amber for numbers
  { tag: tags.string, color: '#4ade80' }, // green for strings
  { tag: tags.comment, color: '#6b7280', fontStyle: 'italic' }, // gray italic for comments
  { tag: tags.variableName, color: '#e2e8f0' }, // light gray for variables
  { tag: tags.definition(tags.variableName), color: '#60a5fa' }, // blue for definitions
  { tag: tags.operator, color: '#94a3b8' }, // slate for operators
  { tag: tags.punctuation, color: '#64748b' }, // darker slate for punctuation
  { tag: tags.function(tags.variableName), color: '#60a5fa' }, // blue for functions
  { tag: tags.propertyName, color: '#38bdf8' }, // cyan for properties
  { tag: tags.typeName, color: '#c084fc' }, // purple for types
  { tag: tags.className, color: '#fbbf24' }, // amber for class names
  { tag: tags.meta, color: '#f472b6' }, // pink for meta
  { tag: tags.name, color: '#e2e8f0' }, // default
])

// Dark theme for CodeMirror
const darkTheme = EditorView.theme({
  '&': {
    backgroundColor: 'transparent',
    color: '#e2e8f0',
    height: '100%',
  },
  '.cm-content': {
    fontFamily: '"Fira Code", "SF Mono", "Monaco", "Inconsolata", monospace',
    fontSize: '14px',
    lineHeight: '1.6',
    padding: '16px',
    caretColor: '#f472b6',
  },
  '.cm-cursor': {
    borderLeftColor: '#f472b6',
    borderLeftWidth: '2px',
  },
  '.cm-activeLine': {
    backgroundColor: 'rgba(255, 255, 255, 0.03)',
  },
  '.cm-selectionBackground, ::selection': {
    backgroundColor: 'rgba(244, 114, 182, 0.2) !important',
  },
  '.cm-focused .cm-selectionBackground': {
    backgroundColor: 'rgba(244, 114, 182, 0.3) !important',
  },
  '.cm-gutters': {
    backgroundColor: 'transparent',
    borderRight: '1px solid rgba(255, 255, 255, 0.1)',
    color: '#64748b',
  },
  '.cm-lineNumbers .cm-gutterElement': {
    padding: '0 12px 0 8px',
    minWidth: '40px',
  },
  '.cm-activeLineGutter': {
    backgroundColor: 'rgba(255, 255, 255, 0.03)',
  },
  '.cm-scroller': {
    overflow: 'auto',
  },
}, { dark: true })

interface EditorProps {
  value: string
  onChange: (value: string) => void
  onRun: () => void
  className?: string
}

export function Editor({ value, onChange, onRun, className }: EditorProps) {
  const containerRef = useRef<HTMLDivElement>(null)
  const viewRef = useRef<EditorView | null>(null)

  // Create run keybinding
  const runKeymap = keymap.of([{
    key: 'Mod-Enter',
    run: () => {
      onRun()
      return true
    }
  }])

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
        keymap.of([...defaultKeymap, indentWithTab]),
        runKeymap,
        fratmScriptLanguage,
        syntaxHighlighting(fratmScriptHighlighting),
        darkTheme,
        EditorView.updateListener.of((update) => {
          if (update.docChanged) {
            onChange(update.state.doc.toString())
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

  return (
    <div
      ref={containerRef}
      className={cn('w-full h-full', className)}
    />
  )
}
