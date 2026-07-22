export interface SkeletonRowsProps {
  rows?: number;
  className?: string;
  /** Height of each shimmer row in px. */
  rowHeight?: number;
}

export function SkeletonRows({ rows = 5, className = "", rowHeight = 16 }: SkeletonRowsProps) {
  return (
    <div className={["flex flex-col gap-2", className].join(" ")} aria-hidden="true" data-testid="skeleton">
      {Array.from({ length: rows }).map((_, i) => (
        <div
          key={i}
          className="animate-pulse rounded-control bg-bg-raised"
          style={{ height: rowHeight, width: `${70 + ((i * 7) % 25)}%` }}
        />
      ))}
    </div>
  );
}
