import { useEffect, useRef } from 'react'
import { EditorState, Compartment } from '@codemirror/state'
import { EditorView, keymap, highlightSpecialChars, drawSelection, highlightActiveLine, lineNumbers } from '@codemirror/view'
import { defaultKeymap, indentWithTab } from '@codemirror/commands'
import { HighlightStyle, syntaxHighlighting, StreamLanguage } from '@codemirror/language'
import { tags } from '@lezer/highlight'
import { useTheme } from 'next-themes'
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
}, { dark: false })

interface EditorProps {
  value: string
  onChange: (value: string) => void
  onRun: () => void
  className?: string
}

// Compartments for dynamic theme switching
const themeCompartment = new Compartment()
const highlightCompartment = new Compartment()

export function Editor({ value, onChange, onRun, className }: EditorProps) {
  const containerRef = useRef<HTMLDivElement>(null)
  const viewRef = useRef<EditorView | null>(null)
  const { resolvedTheme } = useTheme()

  // Determine if dark mode
  const isDark = resolvedTheme === 'dark'

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
        highlightCompartment.of(syntaxHighlighting(isDark ? darkHighlighting : lightHighlighting)),
        themeCompartment.of(isDark ? darkTheme : lightTheme),
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

  return (
    <div
      ref={containerRef}
      className={cn('w-full h-full', className)}
    />
  )
}
