import { useState } from "react";

export interface CopyableCommandProps {
  command: string;
  /** Optional label announced before the command (e.g. "Install"). */
  label?: string;
  className?: string;
}

/**
 * A mono command string with a click-to-copy affordance. Used for install hints,
 * fix commands, and the stall-dialog handoff (SPEC §4.7, §4.8, §F1).
 */
export function CopyableCommand({ command, label, className = "" }: CopyableCommandProps) {
  const [copied, setCopied] = useState(false);

  async function copy() {
    try {
      await navigator.clipboard?.writeText(command);
      setCopied(true);
      window.setTimeout(() => setCopied(false), 1500);
    } catch {
      // Clipboard denied — the command text is still visible to copy manually.
      setCopied(false);
    }
  }

  return (
    <button
      type="button"
      onClick={copy}
      title="Copy to clipboard"
      className={[
        "group inline-flex items-center gap-2 rounded-control border border-border bg-bg-inset px-2.5 py-1.5",
        "font-mono text-[12px] text-text-secondary hover:border-border-strong",
        "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-1 focus-visible:ring-offset-bg-surface",
        className,
      ].join(" ")}
    >
      {label && <span className="font-sans text-[11px] uppercase tracking-wide text-text-muted">{label}</span>}
      <code className="whitespace-pre">{command}</code>
      <span className="font-sans text-[11px] text-text-muted group-hover:text-accent">
        {copied ? "Copied" : "Copy"}
      </span>
    </button>
  );
}
