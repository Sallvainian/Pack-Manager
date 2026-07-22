/**
 * StatusBadge — the per-row status pill (SPEC §4.8). All states carry a text
 * label, not only a color (accessibility requirement, §4.11).
 *
 * Live op state (Upgrading… / Queued) takes precedence over the static package
 * status; the ManagerPane derives `opState` from the operations store.
 */
import type { Package } from "../../lib/ipc/types";
import { Chip, type ChipTone } from "../primitives/Chip";
import { Spinner } from "../primitives/Spinner";

export type RowOpState = "upgrading" | "queued";

export interface StatusBadgeProps {
  pkg: Package;
  opState?: RowOpState;
}

export function StatusBadge({ pkg, opState }: StatusBadgeProps) {
  if (opState === "upgrading") {
    return (
      <Chip tone="accent">
        <Spinner size={10} /> Upgrading…
      </Chip>
    );
  }
  if (opState === "queued") {
    return <Chip tone="neutral">Queued</Chip>;
  }
  if (pkg.pinned) {
    return <Chip tone="neutral">Pinned</Chip>;
  }
  if (pkg.kind === "caskGreedy") {
    return <Chip tone="info">Self-updating</Chip>;
  }
  if (pkg.outdated) {
    return <Chip tone="warning">Update available</Chip>;
  }
  return <Chip tone="success">Up to date</Chip>;
}

/** Small helper: tone for a status word, exported for reuse if needed. */
export const STATUS_TONES: Record<string, ChipTone> = {
  pinned: "neutral",
  selfUpdating: "info",
  updateAvailable: "warning",
  upToDate: "success",
  upgrading: "accent",
  queued: "neutral",
};
