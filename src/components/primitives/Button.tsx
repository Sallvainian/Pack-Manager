import type { ButtonHTMLAttributes, ReactNode } from "react";

export type ButtonVariant = "primary" | "secondary" | "ghost" | "danger";
export type ButtonSize = "sm" | "md";

const VARIANTS: Record<ButtonVariant, string> = {
  primary: "bg-accent text-on-accent hover:bg-accent-hover disabled:bg-accent/40",
  secondary:
    "bg-bg-raised text-text-primary border border-border-strong hover:bg-bg-overlay disabled:opacity-50",
  ghost: "bg-transparent text-text-secondary hover:bg-bg-raised hover:text-text-primary disabled:opacity-40",
  // --color-on-accent is the palette's dark ink for bright fills; on --color-danger
  // it measures 8.30:1. White measured 2.30:1 here and failed the 4.5:1 floor.
  danger: "bg-danger text-on-accent hover:brightness-110 disabled:opacity-50",
};

const SIZES: Record<ButtonSize, string> = {
  sm: "h-7 px-2.5 text-[12px]",
  md: "h-8 px-3 text-[13px]",
};

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant;
  size?: ButtonSize;
  children: ReactNode;
}

export function Button({
  variant = "secondary",
  size = "md",
  className = "",
  children,
  type = "button",
  ...rest
}: ButtonProps) {
  return (
    <button
      type={type}
      className={[
        "inline-flex items-center justify-center gap-1.5 rounded-control font-medium",
        "transition-colors focus-visible:outline-2 focus-visible:outline-focus-ring focus-visible:outline-offset-2",
        "disabled:cursor-not-allowed",
        VARIANTS[variant],
        SIZES[size],
        className,
      ].join(" ")}
      {...rest}
    >
      {children}
    </button>
  );
}
