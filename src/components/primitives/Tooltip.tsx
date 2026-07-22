import type { ReactNode } from "react";

export interface TooltipProps {
  content: ReactNode;
  children: ReactNode;
  /** Preferred side; defaults to top. */
  side?: "top" | "bottom";
  className?: string;
}

/**
 * Lightweight hover/focus tooltip. The content is always present in the DOM
 * (visually hidden until hover/focus via CSS), so it is both accessible
 * (`role="tooltip"`) and queryable in tests.
 */
export function Tooltip({ content, children, side = "top", className = "" }: TooltipProps) {
  return (
    <span className={["group relative inline-flex", className].join(" ")}>
      {children}
      <span
        role="tooltip"
        className={[
          "pointer-events-none absolute left-1/2 z-50 w-max max-w-xs -translate-x-1/2 rounded-control",
          "border border-border-strong bg-bg-overlay px-2 py-1 text-[11px] leading-snug text-text-primary shadow-lg",
          "opacity-0 transition-opacity group-hover:opacity-100 group-focus-within:opacity-100",
          side === "top" ? "bottom-full mb-1.5" : "top-full mt-1.5",
        ].join(" ")}
      >
        {content}
      </span>
    </span>
  );
}
