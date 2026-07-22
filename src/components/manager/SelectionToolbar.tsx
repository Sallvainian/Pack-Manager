/**
 * SelectionToolbar — the slide-up bar shown while rows are selected (SPEC §4.8,
 * §F5): "N selected · Upgrade selected · Clear (Esc)".
 */
import { Button } from "../primitives/Button";

export interface SelectionToolbarProps {
  count: number;
  onUpgradeSelected: () => void;
  onClear: () => void;
}

export function SelectionToolbar({ count, onUpgradeSelected, onClear }: SelectionToolbarProps) {
  if (count === 0) return null;
  return (
    <div
      role="region"
      aria-label="Selection actions"
      className={[
        "sticky bottom-0 z-10 flex items-center gap-3 rounded-card border border-border-strong bg-bg-overlay px-4 py-2 shadow-lg",
      ].join(" ")}
    >
      <span className="text-[13px] font-medium text-text-primary">{count} selected</span>
      <Button variant="primary" size="sm" onClick={onUpgradeSelected}>
        Upgrade selected
      </Button>
      <Button variant="ghost" size="sm" onClick={onClear}>
        Clear (Esc)
      </Button>
    </div>
  );
}
