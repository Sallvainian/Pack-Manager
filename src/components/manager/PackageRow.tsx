/**
 * PackageRow — one row of the PackageTable (SPEC §4.8).
 *
 * Columns: checkbox · name (+ kind sub-label; uv tools expand to executable
 * chips) · installed (mono) · latest (VersionDelta) · status · row Upgrade.
 * Selection interaction lives on the checkbox and reads modifier keys off the
 * click event (shift = range, plain/cmd = toggle) — the Checkbox primitive can't
 * surface the event, so a styled native input is used here.
 */
import type { MouseEvent } from "react";
import type { ManagerId, Package } from "../../lib/ipc/types";
import { Spinner } from "../primitives/Spinner";
import { Tooltip } from "../primitives/Tooltip";
import { StatusBadge, type RowOpState } from "./StatusBadge";
import { VersionDelta } from "./VersionDelta";

/** Kind sub-labels shown under the package name (SPEC §4.8). */
const KIND_LABEL: Partial<Record<Package["kind"], string>> = {
  formula: "formula",
  cask: "cask",
  caskGreedy: "self-updating cask",
  toolchain: "toolchain",
};

export interface PackageRowProps {
  pkg: Package;
  managerId: ManagerId;
  selected: boolean;
  /** Outdated, not pinned, not a greedy cask (SPEC §F3/§F5). */
  selectable: boolean;
  opState?: RowOpState;
  expanded?: boolean;
  /** Cross-manager join target ring (SPEC §4.7). */
  highlighted?: boolean;
  onToggleExpand?: () => void;
  onToggleSelect: (packageId: string) => void;
  onRangeSelect: (packageId: string) => void;
  onUpgrade: (pkg: Package) => void;
}

export function PackageRow({
  pkg,
  selected,
  selectable,
  opState,
  expanded = false,
  highlighted = false,
  onToggleExpand,
  onToggleSelect,
  onRangeSelect,
  onUpgrade,
}: PackageRowProps) {
  const executables = pkg.meta?.executables ?? [];
  const canExpand = executables.length > 0;
  const upgrading = opState === "upgrading";
  const showUpgradeButton = pkg.outdated && !pkg.pinned && pkg.kind !== "caskGreedy";

  function onCheckboxClick(e: MouseEvent<HTMLInputElement>) {
    if (!selectable || upgrading) return;
    // Controlled input — drive selection through the store, not the DOM toggle.
    e.preventDefault();
    if (e.shiftKey) onRangeSelect(pkg.id);
    else onToggleSelect(pkg.id);
  }

  const checkboxDisabled = !selectable || upgrading;
  const checkboxTitle = pkg.pinned
    ? `Pinned in Homebrew — run \`brew unpin ${pkg.name}\` to upgrade`
    : !pkg.outdated
      ? "Already up to date"
      : upgrading
        ? "Upgrade in progress"
        : undefined;

  return (
    <div
      role="row"
      data-testid={`row-${pkg.id}`}
      aria-selected={selected}
      className={[
        "flex items-start gap-2 border-b border-border px-3 py-2 text-[13px]",
        selected ? "bg-accent-subtle" : "hover:bg-bg-raised",
        highlighted ? "ring-2 ring-inset ring-accent" : "",
      ].join(" ")}
    >
      <div role="cell" className="flex w-9 shrink-0 justify-center pt-0.5">
        <input
          type="checkbox"
          checked={selected}
          disabled={checkboxDisabled}
          onChange={() => {}}
          onClick={onCheckboxClick}
          title={checkboxTitle}
          aria-label={`Select ${pkg.name}`}
          className={[
            "h-4 w-4 shrink-0 cursor-pointer rounded-[4px] border border-border-strong bg-bg-raised accent-accent",
            "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-1 focus-visible:ring-offset-bg-surface",
            "disabled:cursor-not-allowed disabled:opacity-40",
          ].join(" ")}
        />
      </div>

      <div role="cell" className="min-w-0 flex-1">
        <div className="flex items-center gap-1.5">
          {canExpand && (
            <button
              type="button"
              onClick={onToggleExpand}
              aria-label={expanded ? `Collapse ${pkg.name}` : `Expand ${pkg.name}`}
              aria-expanded={expanded}
              className="text-text-muted hover:text-text-secondary focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent"
            >
              <span aria-hidden="true" className="inline-block w-3 text-[10px]">
                {expanded ? "▼" : "▶"}
              </span>
            </button>
          )}
          <span className="truncate font-medium text-text-primary" title={pkg.name}>
            {pkg.name}
          </span>
        </div>
        {KIND_LABEL[pkg.kind] && (
          <div className="text-[11px] text-text-muted">{KIND_LABEL[pkg.kind]}</div>
        )}
        {expanded && canExpand && (
          <div className="mt-1 flex flex-wrap gap-1">
            {executables.map((exe) => (
              <span
                key={exe}
                className="rounded-control bg-bg-raised px-1.5 py-0.5 font-mono text-[11px] text-text-secondary"
              >
                {exe}
              </span>
            ))}
          </div>
        )}
      </div>

      <div
        role="cell"
        title={pkg.installed ?? undefined}
        className="w-[100px] shrink-0 truncate pt-0.5 font-mono text-[12px] tabular-nums text-text-secondary"
      >
        {pkg.installed ?? "—"}
      </div>

      <div role="cell" className="w-[216px] shrink-0 pt-0.5">
        <VersionDelta installed={pkg.installed} latest={pkg.latest} outdated={pkg.outdated} />
      </div>

      <div role="cell" className="w-[150px] shrink-0 pt-0.5">
        <StatusBadge pkg={pkg} opState={opState} />
      </div>

      <div role="cell" className="flex w-[90px] shrink-0 justify-end pt-0.5">
        {upgrading ? (
          <Spinner size={14} label={`Upgrading ${pkg.name}`} />
        ) : pkg.pinned ? (
          <Tooltip content={`Run \`brew unpin ${pkg.name}\` to upgrade`}>
            <span className="text-[11px] uppercase tracking-wide text-text-muted">pinned</span>
          </Tooltip>
        ) : showUpgradeButton ? (
          <button
            type="button"
            onClick={() => onUpgrade(pkg)}
            className={[
              "inline-flex h-7 items-center rounded-control px-2.5 text-[12px] font-medium",
              "text-text-secondary hover:bg-bg-raised hover:text-text-primary",
              "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent",
            ].join(" ")}
          >
            Upgrade
          </button>
        ) : null}
      </div>
    </div>
  );
}
