import { useState } from 'react'
import { ChevronRight } from 'lucide-react'
import { cn } from '@/lib/utils'
import type { InspectNode, InspectToken, Inspection } from '@/lib/compiler'
import { useI18n } from '@/lib/i18n'

export type SourceRange = { from: number; to: number }

interface InspectViewProps {
  inspection: Inspection
  view: 'tokens' | 'tree'
  /** Highlights the hovered item's source range in the editor (null clears) */
  onHighlight: (range: SourceRange | null) => void
}

/**
 * Token list and syntax tree of the program, as the compiler sees it.
 * Hovering an item highlights where it comes from in the editor.
 */
export function InspectView({ inspection, view, onHighlight }: InspectViewProps) {
  const { t } = useI18n()
  return (
    <div className="flex flex-col gap-2" onMouseLeave={() => onHighlight(null)}>
      <p className="text-muted-foreground text-xs">{t('output.inspectHint')}</p>
      {view === 'tokens' ? (
        <TokenList tokens={inspection.tokens} onHighlight={onHighlight} />
      ) : inspection.tree ? (
        <div className="bg-card/50 py-2 border border-border rounded-md overflow-x-auto font-mono text-sm leading-relaxed">
          <TreeRow node={inspection.tree} depth={0} onHighlight={onHighlight} />
        </div>
      ) : (
        <p className="bg-card/50 p-3 border border-border rounded-md text-muted-foreground text-sm">
          {t('output.treeError')}
          {inspection.error && <span className="block mt-1 font-mono text-destructive-foreground text-xs">{inspection.error}</span>}
        </p>
      )}
    </div>
  )
}

function TokenList({ tokens, onHighlight }: { tokens: InspectToken[]; onHighlight: InspectViewProps['onHighlight'] }) {
  const { t } = useI18n()
  if (tokens.length === 0) {
    return <p className="bg-card/50 p-3 border border-border rounded-md text-muted-foreground text-sm text-center">{t('output.noTokens')}</p>
  }
  return (
    <ol className="flex flex-wrap gap-1.5 bg-card/50 p-3 border border-border rounded-md font-mono text-xs">
      {tokens.map((token, i) => (
        <li
          key={i}
          onMouseEnter={() => onHighlight(token)}
          title={`${token.kind} · ${token.line}:${token.column}`}
          className="flex items-baseline gap-1 hover:bg-accent px-1.5 py-0.5 border border-border rounded cursor-default"
        >
          <span className="text-foreground">{token.text}</span>
          <span className={cn('text-[0.7rem]', CATEGORY_COLORS[token.category])}>{token.kind}</span>
        </li>
      ))}
    </ol>
  )
}

/** Keywords, literals and names in different colors, so the list scans easily. */
const CATEGORY_COLORS: Record<InspectToken['category'], string> = {
  keyword: 'text-purple-600 dark:text-purple-400',
  name: 'text-cyan-700 dark:text-cyan-400',
  literal: 'text-amber-700 dark:text-amber-400',
  punct: 'text-muted-foreground',
  invalid: 'text-destructive-foreground',
}

function TreeRow({ node, depth, onHighlight }: { node: InspectNode; depth: number; onHighlight: InspectViewProps['onHighlight'] }) {
  // The first levels start open: program, statements and their direct parts
  const [open, setOpen] = useState(depth < 3)
  const children = node.children ?? []
  const hasRange = node.from !== undefined && node.to !== undefined
  const key = node.key !== undefined && <span className="text-cyan-700 dark:text-cyan-400">{node.key}: </span>

  if (node.value !== undefined) {
    return (
      <div className="px-3 whitespace-nowrap" style={{ paddingLeft: `${depth * 14 + 26}px` }}>
        {key}
        <span className="text-amber-700 dark:text-amber-400">{node.value}</span>
      </div>
    )
  }
  return (
    <>
      <button
        type="button"
        onClick={() => setOpen(o => !o)}
        onMouseEnter={() => onHighlight(hasRange ? { from: node.from!, to: node.to! } : null)}
        aria-expanded={children.length > 0 ? open : undefined}
        className="flex items-center gap-1 hover:bg-accent px-3 w-full text-left whitespace-nowrap"
        style={{ paddingLeft: `${depth * 14 + 12}px` }}
      >
        <ChevronRight className={cn('size-3.5 text-muted-foreground transition-transform shrink-0', open && 'rotate-90', children.length === 0 && 'invisible')} />
        <span>
          {key}
          <span className={node.label.startsWith('[') ? 'text-muted-foreground' : 'text-purple-600 dark:text-purple-400'}>
            {node.label || '{…}'}
          </span>
        </span>
      </button>
      {open && children.map((child, i) => (
        <TreeRow key={i} node={child} depth={depth + 1} onHighlight={onHighlight} />
      ))}
    </>
  )
}
