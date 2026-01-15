"use client"

import * as React from "react"
import { Select as BaseSelect } from "@base-ui/react/select"
import { cn } from "@/lib/utils"
import { ChevronDown } from "lucide-react"

// Native select wrapper - simpler for basic use cases
export interface NativeSelectProps
  extends Omit<React.SelectHTMLAttributes<HTMLSelectElement>, "size"> {
  selectSize?: "sm" | "default" | "lg"
}

const NativeSelect = React.forwardRef<HTMLSelectElement, NativeSelectProps>(
  ({ className, children, selectSize = "default", ...props }, ref) => {
    const sizeClasses = {
      sm: "h-7 text-xs px-2",
      default: "h-8 text-sm px-3",
      lg: "h-10 text-base px-4",
    }

    return (
      <div className="relative">
        <select
          className={cn(
            "flex w-full appearance-none rounded-lg border border-input bg-secondary py-1 pr-8 text-foreground shadow-xs transition-colors",
            "focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring",
            "disabled:cursor-not-allowed disabled:opacity-50",
            sizeClasses[selectSize],
            className
          )}
          ref={ref}
          {...props}
        >
          {children}
        </select>
        <ChevronDown className="absolute right-2 top-1/2 size-4 -translate-y-1/2 opacity-50 pointer-events-none" />
      </div>
    )
  }
)
NativeSelect.displayName = "NativeSelect"

// Re-export Base UI Select primitives for advanced usage
const Select = BaseSelect.Root
const SelectTrigger = BaseSelect.Trigger
const SelectValue = BaseSelect.Value
const SelectIcon = BaseSelect.Icon
const SelectBackdrop = BaseSelect.Backdrop
const SelectPositioner = BaseSelect.Positioner
const SelectPopup = BaseSelect.Popup
const SelectGroup = BaseSelect.Group
const SelectGroupLabel = BaseSelect.GroupLabel
const SelectItem = BaseSelect.Item
const SelectItemIndicator = BaseSelect.ItemIndicator
const SelectItemText = BaseSelect.ItemText
const SelectSeparator = BaseSelect.Separator
const SelectArrow = BaseSelect.Arrow

export {
  NativeSelect,
  Select,
  SelectTrigger,
  SelectValue,
  SelectIcon,
  SelectBackdrop,
  SelectPositioner,
  SelectPopup,
  SelectGroup,
  SelectGroupLabel,
  SelectItem,
  SelectItemIndicator,
  SelectItemText,
  SelectSeparator,
  SelectArrow,
}
