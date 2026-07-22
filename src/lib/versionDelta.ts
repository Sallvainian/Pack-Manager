/**
 * versionDelta.ts — the pure string segment-diff behind the VersionDelta visual
 * signature (SPEC §4.6, DECISIONS D2).
 *
 * DISPLAY ONLY. This never decides whether a package is outdated — the manager's
 * `outdated` verdict is authoritative (SPEC invariant 1). This function only
 * decides how to *highlight* an `installed → latest` pair the manager already
 * called outdated, and refuses to fabricate a delta from non-numeric tokens
 * (`stable`, git hashes) or a missing `latest`.
 *
 * Rule (SPEC §4.6): split both on `.`/`-`, find the first differing segment,
 * render the common prefix of `latest` muted and the changed suffix in a severity
 * color keyed by the differing segment's index — 0 → major, 1 → minor, ≥2 → patch.
 */

export type DeltaSeverity = "major" | "minor" | "patch";

export interface VersionDelta {
  /** True when installed/latest are a comparable numeric version pair that differs. */
  comparable: boolean;
  /** Severity from the first differing segment index; null when not comparable. */
  severity: DeltaSeverity | null;
  /** Leading portion of `latest` shared with `installed` (segments + separators). */
  prefix: string;
  /** Changed remainder of `latest`, rendered in the severity color. */
  suffix: string;
}

/** Segment delimiter set: dots and dashes (SPEC §4.6). */
const DELIM = /[.-]/;
/** Same delimiters, capturing — keeps separators so the prefix is byte-faithful. */
const DELIM_CAPTURE = /([.-])/;

function isNumericSeg(s: string | undefined): boolean {
  return s !== undefined && s.length > 0 && /^[0-9]+$/.test(s);
}

function severityForIndex(index: number): DeltaSeverity {
  if (index <= 0) return "major";
  if (index === 1) return "minor";
  return "patch";
}

/**
 * Compute the display delta for an `installed → latest` pair. Returns a
 * non-comparable result (no highlight) for equal versions, missing values, or
 * pairs whose first differing segment is not numeric on both sides.
 */
export function versionDelta(installed: string | null, latest: string | null): VersionDelta {
  const plain: VersionDelta = { comparable: false, severity: null, prefix: latest ?? "", suffix: "" };

  if (installed == null || latest == null || installed === latest) return plain;

  const installedSegs = installed.split(DELIM);
  const latestSegs = latest.split(DELIM);

  // First index at which the two segment lists differ.
  let i = 0;
  const min = Math.min(installedSegs.length, latestSegs.length);
  while (i < min && installedSegs[i] === latestSegs[i]) i++;

  // The changed segment must be numeric on the latest side (and, when present,
  // on the installed side) — never fabricate a delta from `stable` or a hash.
  const changedLatest = latestSegs[i];
  const changedInstalled = installedSegs[i];
  const comparable =
    isNumericSeg(changedLatest) && (changedInstalled === undefined || isNumericSeg(changedInstalled));
  if (!comparable) return plain;

  // Rebuild the prefix from `latest`, preserving its original separators. The
  // capturing split interleaves segments (even indices) and separators (odd),
  // so segment j sits at position 2*j.
  const parts = latest.split(DELIM_CAPTURE);
  const cut = i * 2;
  return {
    comparable: true,
    severity: severityForIndex(i),
    prefix: parts.slice(0, cut).join(""),
    suffix: parts.slice(cut).join(""),
  };
}
