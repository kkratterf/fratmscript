import { cn } from '@/lib/utils'

interface LoadingOverlayProps {
  isVisible: boolean
}

export function LoadingOverlay({ isVisible }: LoadingOverlayProps) {
  return (
    <div
      className={cn(
        "fixed inset-0 bg-background/95 flex flex-col items-center justify-center gap-4 z-50 transition-opacity",
        isVisible ? "opacity-100" : "opacity-0 pointer-events-none"
      )}
    >
      <div className="w-10 h-10 border-3 border-muted border-t-primary rounded-full animate-spin" />
      <p className="text-muted-foreground">Loading FratmScript...</p>
    </div>
  )
}
