import { useEffect, useMemo, useRef } from 'react'
import { useTheme } from 'next-themes'
import { previewDocument, type PreviewEnvelope } from '@/lib/preview'
import { useI18n } from '@/lib/i18n'

interface PreviewFrameProps {
  /** Run that produced the code; messages from older runs are dropped */
  id: number
  code: string
  onMessage: (message: PreviewEnvelope) => void
}

/**
 * Renders the user's component in a sandboxed iframe. `allow-scripts` without
 * `allow-same-origin` gives it an opaque origin: it can't touch the playground,
 * its storage or cookies, and only talks back through postMessage.
 */
export function PreviewFrame({ id, code, onMessage }: PreviewFrameProps) {
  const { t } = useI18n()
  const { resolvedTheme } = useTheme()
  const frameRef = useRef<HTMLIFrameElement>(null)
  const dark = resolvedTheme === 'dark'
  const srcDoc = useMemo(() => previewDocument(id, code, dark), [id, code, dark])

  const onMessageRef = useRef(onMessage)
  onMessageRef.current = onMessage
  useEffect(() => {
    const listener = (event: MessageEvent) => {
      if (event.source !== frameRef.current?.contentWindow) return
      const data = event.data as PreviewEnvelope | null
      if (data && typeof data === 'object' && data.__fratmPreview === id) onMessageRef.current(data)
    }
    window.addEventListener('message', listener)
    return () => window.removeEventListener('message', listener)
  }, [id])

  return (
    <iframe
      ref={frameRef}
      title={t('preview.title')}
      sandbox="allow-scripts allow-modals allow-forms"
      srcDoc={srcDoc}
      className="bg-background border border-border rounded-md w-full h-72 resize-y"
    />
  )
}
