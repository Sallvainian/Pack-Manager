import type { ReactNode } from "react";

export type ChipTone = "neutral" | "accent" | "success" | "warning" | "danger" | "info";

const TONES: Record<ChipTone, string> = {
  neutral: "bg-bg-raised text-text-secondary border-border",
  accent: "bg-accent-subtle text-accent border-accent/30",
  success: "bg-success/12 text-success border-success/30",
  warning: "bg-warning/12 text-warning border-warning/30",
  danger: "bg-danger/12 text-danger border-danger/30",
  info: "bg-info/12 text-info border-info/30",
};

export interface ChipProps {
  tone?: ChipTone;
  children: ReactNode;
  /** When set, the chip is a button. */
  onClick?: () => void;
  title?: string;
  className?: string;
}

export function Chip({ tone = "neutral", children, onClick, title, className = "" }: ChipProps) {
  const base = [
    "inline-flex shrink-0 items-center gap-1 whitespace-nowrap rounded-full border px-2 py-0.5",
    "text-[11px] font-medium uppercase tracking-wide",
    TONES[tone],
    className,
  ].join(" ");

  if (onClick) {
    return (
      <button
        type="button"
        onClick={onClick}
        title={title}
        className={[
          base,
          "cursor-pointer transition-colors hover:brightness-125",
          "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-1 focus-visible:ring-offset-bg-surface",
        ].join(" ")}
      >
        {children}
      </button>
    );
  }
  return (
    <span className={base} title={title}>
      {children}
    </span>
  );
}
