import { useEffect, useRef } from "react";

export interface CheckboxProps {
  checked: boolean;
  /** Renders the tri-state dash; overrides the check when true. */
  indeterminate?: boolean;
  disabled?: boolean;
  onChange?: (checked: boolean) => void;
  "aria-label"?: string;
  title?: string;
}

export function Checkbox({
  checked,
  indeterminate = false,
  disabled = false,
  onChange,
  title,
  ...aria
}: CheckboxProps) {
  const ref = useRef<HTMLInputElement>(null);
  useEffect(() => {
    if (ref.current) ref.current.indeterminate = indeterminate && !checked;
  }, [indeterminate, checked]);

  return (
    <input
      ref={ref}
      type="checkbox"
      checked={checked}
      disabled={disabled}
      title={title}
      aria-label={aria["aria-label"]}
      onChange={(e) => onChange?.(e.target.checked)}
      className={[
        "h-4 w-4 shrink-0 cursor-pointer rounded-[4px] border border-border-strong bg-bg-raised",
        "accent-accent",
        "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-1 focus-visible:ring-offset-bg-surface",
        "disabled:cursor-not-allowed disabled:opacity-40",
      ].join(" ")}
    />
  );
}
