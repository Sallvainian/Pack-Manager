export interface SpinnerProps {
  /** Pixel size of the spinner box. */
  size?: number;
  className?: string;
  label?: string;
}

export function Spinner({ size = 14, className = "", label = "Loading" }: SpinnerProps) {
  return (
    <span
      role="status"
      aria-label={label}
      className={["inline-block animate-spin rounded-full border-2 border-border-strong border-t-accent", className].join(" ")}
      style={{ width: size, height: size }}
    />
  );
}
