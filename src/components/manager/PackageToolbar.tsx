/**
 * PackageToolbar — search, outdated-only toggle, counts, and the bulk-upgrade
 * entry points (SPEC §4.8 row 4).
 *
 * Search is debounced (200ms) into the packages store; the ManagerPane reads the
 * store value to filter rows (name + executables). The outdated-only toggle
 * defaults ON whenever anything is outdated (seeded by the store on snapshot).
 */
import { useEffect, useRef, useState } from "react";
import type { ManagerId } from "../../lib/ipc/types";
import { usePackagesStore } from "../../store/packages";
import { Button } from "../primitives/Button";

const SEARCH_DEBOUNCE_MS = 200;

export interface PackageToolbarProps {
  managerId: ManagerId;
  installedCount: number;
  outdatedCount: number;
  selectionCount: number;
  onUpgradeSelected: () => void;
  onUpgradeAll: () => void;
}

export function PackageToolbar({
  managerId,
  installedCount,
  outdatedCount,
  selectionCount,
  onUpgradeSelected,
  onUpgradeAll,
}: PackageToolbarProps) {
  const storedSearch = usePackagesStore((s) => s.search[managerId] ?? "");
  const setSearch = usePackagesStore((s) => s.setSearch);
  const storedOutdatedOnly = usePackagesStore((s) => s.outdatedOnly[managerId]);
  const setOutdatedOnly = usePackagesStore((s) => s.setOutdatedOnly);

  const outdatedOnly = storedOutdatedOnly ?? outdatedCount > 0;

  const [draft, setDraft] = useState(storedSearch);
  const timer = useRef<ReturnType<typeof setTimeout> | null>(null);

  // Debounce the draft into the store; the pane filters off the store value.
  useEffect(() => {
    if (timer.current) clearTimeout(timer.current);
    timer.current = setTimeout(() => setSearch(managerId, draft), SEARCH_DEBOUNCE_MS);
    return () => {
      if (timer.current) clearTimeout(timer.current);
    };
  }, [draft, managerId, setSearch]);

  return (
    <div className="flex flex-wrap items-center gap-3 py-2">
      <input
        type="search"
        value={draft}
        onChange={(e) => setDraft(e.target.value)}
        placeholder="Search packages"
        aria-label="Search packages"
        className={[
          "h-8 w-[240px] rounded-control border border-border bg-bg-raised px-2.5 text-[13px] text-text-primary",
          "placeholder:text-text-muted focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent",
        ].join(" ")}
      />

      <label className="flex cursor-pointer items-center gap-1.5 text-[13px] text-text-secondary">
        <input
          type="checkbox"
          checked={outdatedOnly}
          onChange={(e) => setOutdatedOnly(managerId, e.target.checked)}
          aria-label="Outdated only"
          className="h-4 w-4 rounded-[4px] border border-border-strong bg-bg-raised accent-accent"
        />
        Outdated only
      </label>

      <div className="ml-auto flex items-center gap-3">
        <span className="text-[11px] uppercase tracking-wide text-text-muted">
          {installedCount} installed · {outdatedCount} outdated
        </span>
        {selectionCount > 0 && (
          <Button variant="secondary" size="sm" onClick={onUpgradeSelected}>
            Upgrade selected ({selectionCount})
          </Button>
        )}
        {outdatedCount > 0 && (
          <Button variant="primary" size="sm" onClick={onUpgradeAll}>
            Upgrade all ({outdatedCount})
          </Button>
        )}
      </div>
    </div>
  );
}
