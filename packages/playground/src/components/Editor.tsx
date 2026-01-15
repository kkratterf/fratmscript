import { forwardRef, useCallback } from 'react'
import { cn } from '@/lib/utils'

interface EditorProps {
  value: string
  onChange: (value: string) => void
  onRun: () => void
  className?: string
}

export const Editor = forwardRef<HTMLTextAreaElement, EditorProps>(
  ({ value, onChange, onRun, className }, ref) => {
    const handleKeyDown = useCallback(
      (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
        // Ctrl+Enter or Cmd+Enter to run
        if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
          e.preventDefault()
          onRun()
        }

        // Tab for indentation
        if (e.key === 'Tab') {
          e.preventDefault()
          const target = e.target as HTMLTextAreaElement
          const start = target.selectionStart
          const end = target.selectionEnd
          const newValue = value.substring(0, start) + '  ' + value.substring(end)
          onChange(newValue)
          // Set cursor position after React updates
          setTimeout(() => {
            target.selectionStart = target.selectionEnd = start + 2
          }, 0)
        }
      },
      [value, onChange, onRun]
    )

    return (
      <textarea
        ref={ref}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        onKeyDown={handleKeyDown}
        spellCheck={false}
        placeholder="Write your FratmScript code..."
        className={cn(
          "w-full h-full p-4 font-mono text-sm leading-relaxed",
          "bg-transparent text-foreground resize-none",
          "focus:outline-none",
          "placeholder:text-muted-foreground/50",
          className
        )}
        style={{ tabSize: 2 }}
      />
    )
  }
)

Editor.displayName = 'Editor'
