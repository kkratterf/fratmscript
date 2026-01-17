"use client";

import * as React from "react";
import { Dialog as DialogPrimitive } from "@base-ui/react/dialog";

import { cn } from "@/lib/utils";

const Dialog = DialogPrimitive.Root;

const DialogTrigger = DialogPrimitive.Trigger;

const DialogClose = DialogPrimitive.Close;

const DialogPortal = DialogPrimitive.Portal;

function DialogBackdrop({
  className,
  ...props
}: React.ComponentPropsWithoutRef<typeof DialogPrimitive.Backdrop>) {
  return (
    <DialogPrimitive.Backdrop
      className={cn(
        "z-50 fixed inset-0 bg-black/60 data-[ending-style]:opacity-0 data-[starting-style]:opacity-0 transition-all duration-200",
        className
      )}
      {...props}
    />
  );
}

function CloseIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <path d="M18 6 6 18" />
      <path d="m6 6 12 12" />
    </svg>
  );
}

function DialogPopup({
  className,
  children,
  showCloseButton = true,
  ...props
}: React.ComponentPropsWithoutRef<typeof DialogPrimitive.Popup> & {
  showCloseButton?: boolean;
}) {
  return (
    <DialogPortal>
      <DialogBackdrop />
      <DialogPrimitive.Popup
        className={cn(
          "top-1/2 left-1/2 z-50 fixed w-[calc(100%-2rem)] max-w-lg overflow-hidden -translate-x-1/2 -translate-y-1/2",
          "rounded-xl border bg-card shadow-2xl shadow-black/20",
          "transition-all duration-200 ease-out",
          "data-[ending-style]:scale-95 data-[ending-style]:opacity-0",
          "data-[starting-style]:scale-95 data-[starting-style]:opacity-0",
          className
        )}
        {...props}
      >
        {children}
        {showCloseButton && (
          <DialogPrimitive.Close className="top-4 right-4 absolute opacity-70 hover:opacity-100 p-1 rounded-md focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 text-muted-foreground transition-opacity cursor-pointer">
            <CloseIcon className="size-4" />
            <span className="sr-only">Close</span>
          </DialogPrimitive.Close>
        )}
      </DialogPrimitive.Popup>
    </DialogPortal>
  );
}

function DialogHeader({
  className,
  ...props
}: React.HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={cn("flex flex-col gap-1.5 bg-background px-4 pt-5 pb-4 border-border border-b", className)}
      {...props}
    />
  );
}

function DialogPanel({
  className,
  ...props
}: React.HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={cn("flex-1 px-6 py-4 overflow-y-auto", className)}
      {...props}
    />
  );
}

function DialogFooter({
  className,
  variant = "default",
  ...props
}: React.HTMLAttributes<HTMLDivElement> & {
  variant?: "default" | "bare";
}) {
  return (
    <div
      className={cn(
        "flex sm:flex-row flex-col-reverse sm:justify-end gap-2 px-6 pt-4 pb-6",
        variant === "default" && "border-t bg-muted/30",
        className
      )}
      {...props}
    />
  );
}

function DialogTitle({
  className,
  ...props
}: React.ComponentPropsWithoutRef<typeof DialogPrimitive.Title>) {
  return (
    <DialogPrimitive.Title
      className={cn("font-semibold text-lg leading-none tracking-tight", className)}
      {...props}
    />
  );
}

function DialogDescription({
  className,
  ...props
}: React.ComponentPropsWithoutRef<typeof DialogPrimitive.Description>) {
  return (
    <DialogPrimitive.Description
      className={cn("text-muted-foreground text-sm", className)}
      {...props}
    />
  );
}

export {
  Dialog,
  DialogTrigger,
  DialogClose,
  DialogPortal,
  DialogBackdrop,
  DialogPopup,
  DialogHeader,
  DialogPanel,
  DialogFooter,
  DialogTitle,
  DialogDescription,
};
