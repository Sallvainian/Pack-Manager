import "../../test/setup";
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";

import { versionDelta } from "../../lib/versionDelta";
import { VersionDelta } from "./VersionDelta";

// --- pure function table (SPEC §7.5 "version_delta unit table") --------------
describe("versionDelta", () => {
  it("patch: last segment differs → patch, prefix + changed suffix", () => {
    const d = versionDelta("2.2.1", "2.2.2");
    expect(d.comparable).toBe(true);
    expect(d.severity).toBe("patch");
    expect(d.prefix).toBe("2.2.");
    expect(d.suffix).toBe("2");
  });

  it("major: first segment differs → major, whole latest is the suffix", () => {
    const d = versionDelta("6.0.3", "7.0.2");
    expect(d.comparable).toBe(true);
    expect(d.severity).toBe("major");
    expect(d.prefix).toBe("");
    expect(d.suffix).toBe("7.0.2");
  });

  it("minor: second segment differs → minor, separators preserved", () => {
    const d = versionDelta("2.0.14-1", "2.1.2-1");
    expect(d.comparable).toBe(true);
    expect(d.severity).toBe("minor");
    expect(d.prefix).toBe("2.");
    expect(d.suffix).toBe("1.2-1");
  });

  it("shorter installed than latest → patch on the added segment", () => {
    const d = versionDelta("1.2", "1.2.3");
    expect(d.comparable).toBe(true);
    expect(d.severity).toBe("patch");
    expect(d.prefix).toBe("1.2.");
    expect(d.suffix).toBe("3");
  });

  it("plain: identical versions are not a delta", () => {
    expect(versionDelta("stable", "stable").comparable).toBe(false);
  });

  it("plain: non-numeric changed segment never fabricates a delta", () => {
    expect(versionDelta("stable", "nightly").comparable).toBe(false);
    expect(versionDelta("8bab26f4f", "a1b2c3d4e").comparable).toBe(false);
  });

  it("unknown: missing latest is not a delta", () => {
    expect(versionDelta("0.15.20", null).comparable).toBe(false);
  });
});

// --- component (SPEC §7.5 "version_delta_highlights_only_changed_segments") --
describe("version_delta_highlights_only_changed_segments", () => {
  it("highlights only the changed patch segment", () => {
    render(<VersionDelta installed="2.2.1" latest="2.2.2" outdated />);
    const changed = screen.getByTestId("delta-changed");
    expect(changed).toHaveTextContent("2");
    expect(changed).toHaveAttribute("data-severity", "patch");
    expect(screen.getByTestId("delta-prefix")).toHaveTextContent("2.2.");
    expect(screen.getByText("patch")).toBeInTheDocument();
  });

  it("highlights the whole latest for a major bump", () => {
    render(<VersionDelta installed="6.0.3" latest="7.0.2" outdated />);
    const changed = screen.getByTestId("delta-changed");
    expect(changed).toHaveTextContent("7.0.2");
    expect(changed).toHaveAttribute("data-severity", "major");
    expect(screen.getByText("major")).toBeInTheDocument();
  });

  it("renders plain (no highlight) for an up-to-date, non-comparable pair", () => {
    render(<VersionDelta installed="stable" latest="stable" outdated={false} />);
    expect(screen.queryByTestId("delta-changed")).not.toBeInTheDocument();
    expect(screen.getByText("stable")).toBeInTheDocument();
  });

  it("shows 'update available' instead of a fabricated delta when latest is unknown", () => {
    render(<VersionDelta installed="0.15.20" latest={null} outdated />);
    expect(screen.getByText("update available")).toBeInTheDocument();
    expect(screen.queryByTestId("delta-changed")).not.toBeInTheDocument();
  });
});
