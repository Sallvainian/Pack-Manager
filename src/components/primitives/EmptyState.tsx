import type { ReactNode } from "react";

export interface EmptyStateProps {
  /** Small glyph/icon shown above the title. */
  icon?: ReactNode;
  title: string;
  description?: ReactNode;
  /** Optional action row (buttons, install hints). */
  action?: ReactNode;
  tone?: "neutral" | "success";
  className?: string;
}

export function EmptyState({
  icon,
  title,
  description,
  action,
  tone = "neutral",
  className = "",
}: EmptyStateProps) {
  return (
    <div
      className={[
        "flex flex-col items-center justify-center gap-2 rounded-card px-6 py-10 text-center",
        className,
      ].join(" ")}
    >
      {icon && (
        <div className={tone === "success" ? "text-success" : "text-text-muted"}>{icon}</div>
      )}
      <div className="text-[15px] font-semibold text-text-primary">{title}</div>
      {description && <div className="max-w-md text-[13px] text-text-secondary">{description}</div>}
      {action && <div className="mt-2 flex items-center gap-2">{action}</div>}
    </div>
  );
}
