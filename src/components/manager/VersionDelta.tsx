/**
 * VersionDelta — the visual signature (SPEC §4.6). Renders `installed → latest`
 * in mono, highlighting only the changed suffix of `latest` in a severity color,
 * with a text-labelled severity chip. Non-comparable pairs (`stable`, hashes) and
 * a missing `latest` show "update available" rather than a fabricated delta.
 *
 * Purely presentational: the `outdated` flag comes from the manager.
 */
import { versionDelta, type DeltaSeverity } from "../../lib/versionDelta";
import { Chip, type ChipTone } from "../primitives/Chip";

const SEV_TONE: Record<DeltaSeverity, ChipTone> = {
  major: "danger",
  minor: "warning",
  patch: "success",
};

const SEV_TEXT: Record<DeltaSeverity, string> = {
  major: "text-sev-major",
  minor: "text-sev-minor",
  patch: "text-sev-patch",
};

export interface VersionDeltaProps {
  installed: string | null;
  latest: string | null;
  outdated: boolean;
}

export function VersionDelta({ installed, latest, outdated }: VersionDeltaProps) {
  // Outdated with an unknown latest (uv until its format is captured): never a
  // fabricated delta — just the honest "update available".
  if (outdated && latest == null) {
    return <span className="text-[12px] text-text-muted">update available</span>;
  }

  const delta = versionDelta(installed, latest);

  if (outdated && delta.comparable && delta.severity) {
    return (
      <span className="inline-flex items-center gap-2 font-mono text-[12px] tabular-nums">
        <span>
          <span className="text-text-secondary">{installed}</span>
          <span className="px-1 text-text-muted">→</span>
          <span className="text-text-secondary" data-testid="delta-prefix">
            {delta.prefix}
          </span>
          <span
            className={SEV_TEXT[delta.severity]}
            data-testid="delta-changed"
            data-severity={delta.severity}
          >
            {delta.suffix}
          </span>
        </span>
        <Chip tone={SEV_TONE[delta.severity]}>{delta.severity}</Chip>
      </span>
    );
  }

  // Outdated but non-comparable (e.g. `stable`, commit hashes): honest label.
  if (outdated) {
    return <span className="text-[12px] text-text-muted">update available</span>;
  }

  // Up to date: show the current version plainly, no highlight.
  return (
    <span className="font-mono text-[12px] tabular-nums text-text-secondary">
      {latest ?? installed ?? "—"}
    </span>
  );
}
