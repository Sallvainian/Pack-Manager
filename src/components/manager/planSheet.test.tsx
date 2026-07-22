import "../../test/setup";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, within } from "@testing-library/react";

vi.mock("../../lib/ipc/bridge", () => import("../../test/fakeIpc"));

import { ManagerPane } from "./ManagerPane";
import { UpgradePlanSheet } from "../dialogs/UpgradePlanSheet";
import type { UpgradePlan } from "../../lib/ipc/types";
import { resetStores, useManagersStore, usePackagesStore, useUiStore } from "../../store";
import * as fakeIpc from "../../test/fakeIpc";
import { defaultSettings, detectionReport, npmSnapshot } from "../../test/fixtures";

beforeEach(() => {
  fakeIpc.reset();
  resetStores();
  useManagersStore.getState().setDetection(detectionReport);
  useUiStore.getState().setSettings(defaultSettings);
});

const basePlan: UpgradePlan = {
  planId: "plan-1",
  groups: [
    {
      subject: "brew",
      executor: "brew",
      locks: ["brew"],
      commands: [{ argvPreview: "brew upgrade dolt", label: "Upgrade formulae" }],
      packageIds: ["formula:dolt"],
      selfUpdate: false,
    },
    {
      subject: "npm",
      executor: "npm",
      locks: ["npm", "mise"],
      commands: [{ argvPreview: "npm install -g typescript@latest", label: "Upgrade globals" }],
      packageIds: ["globalPackage:typescript"],
      selfUpdate: true,
    },
  ],
  excluded: [
    { managerId: "brew", packageId: "formula:deno", reason: "pinned" },
    { managerId: "brew", packageId: "caskGreedy:openusage", reason: "greedyCask" },
    { managerId: "mise", packageId: "tool:rust", reason: "rustDedup" },
  ],
  notes: ["rust toolchains are handled by rustup in this plan"],
  warnings: ["Homebrew list may be stale"],
};

describe("plan_sheet_renders_exact_command_previews_and_excluded_reasons", () => {
  it("shows every command verbatim, the excluded reasons, warnings and notes", () => {
    render(<UpgradePlanSheet plan={basePlan} />);

    expect(screen.getByText("Upgrade 2 packages")).toBeInTheDocument();
    expect(screen.getByText("brew upgrade dolt")).toBeInTheDocument();
    expect(screen.getByText("npm install -g typescript@latest")).toBeInTheDocument();

    const dialog = screen.getByRole("dialog");
    expect(dialog.textContent).toContain("pinned");
    expect(dialog.textContent).toContain("self-updating cask");
    expect(dialog.textContent).toContain("handled by rustup");
    expect(dialog.textContent).toContain("Homebrew list may be stale");
    expect(dialog.textContent).toContain("rust toolchains are handled by rustup in this plan");
    expect(dialog.textContent).toContain("each manager runs one command at a time");
  });
});

describe("greedy_toggle_off_by_default", () => {
  it("defaults self-updates on and self-updating casks off", () => {
    render(<UpgradePlanSheet plan={basePlan} />);
    expect((screen.getByLabelText("Include manager self-updates") as HTMLInputElement).checked).toBe(true);
    expect((screen.getByLabelText("Include self-updating casks") as HTMLInputElement).checked).toBe(false);
  });
});

describe("confirm_calls_execute_plan_with_toggled_plan", () => {
  it("rebuilds on toggle and executes the plan currently shown", async () => {
    const toggledPlan: UpgradePlan = {
      planId: "plan-2",
      groups: [
        {
          subject: "brew",
          executor: "brew",
          locks: ["brew"],
          commands: [
            { argvPreview: "brew upgrade dolt", label: "Upgrade formulae" },
            { argvPreview: "brew upgrade --cask --greedy openusage", label: "Upgrade greedy casks" },
          ],
          packageIds: ["formula:dolt", "caskGreedy:openusage"],
          selfUpdate: false,
        },
      ],
      excluded: [],
      notes: [],
      warnings: [],
    };
    fakeIpc.respond("build_upgrade_plan", () => toggledPlan);
    fakeIpc.respond("execute_plan", () => ({ opIds: ["op-1"] }));

    render(<UpgradePlanSheet plan={basePlan} />);

    fireEvent.click(screen.getByLabelText("Include self-updating casks"));
    await vi.waitFor(() =>
      expect(screen.getByText("brew upgrade --cask --greedy openusage")).toBeInTheDocument(),
    );

    fireEvent.click(screen.getByRole("button", { name: "Upgrade" }));
    await vi.waitFor(() => expect(fakeIpc.called("execute_plan")).toBe(true));

    const call = fakeIpc.callsFor("execute_plan")[0].args as { args: { plan: UpgradePlan } };
    expect(call.args.plan).toEqual(toggledPlan);
  });
});

describe("upgrade_selected_dispatches_exact_ids_then_clears_on_success", () => {
  function Harness() {
    const dialog = useUiStore((s) => s.dialog);
    return (
      <>
        <ManagerPane managerId="npm" />
        {dialog.kind === "upgradePlan" && <UpgradePlanSheet plan={dialog.plan} />}
      </>
    );
  }

  it("builds a plan with exactly the checked ids, executes it, and clears selection", async () => {
    const cannedPlan: UpgradePlan = {
      planId: "plan-sel",
      groups: [
        {
          subject: "npm",
          executor: "npm",
          locks: ["npm", "mise"],
          commands: [
            {
              argvPreview: "npm install -g @google/gemini-cli@latest typescript@latest",
              label: "Upgrade globals",
            },
          ],
          packageIds: ["globalPackage:@google/gemini-cli", "globalPackage:typescript"],
          selfUpdate: false,
        },
      ],
      excluded: [],
      notes: [],
      warnings: [],
    };
    fakeIpc.respond("build_upgrade_plan", () => cannedPlan);
    fakeIpc.respond("execute_plan", () => ({ opIds: ["op-1"] }));

    usePackagesStore.getState().setSnapshot("npm", npmSnapshot);
    render(<Harness />);

    fireEvent.click(screen.getByLabelText("Select @google/gemini-cli"));
    fireEvent.click(screen.getByLabelText("Select typescript"));

    fireEvent.click(screen.getByRole("button", { name: "Upgrade selected (2)" }));
    await vi.waitFor(() => expect(screen.getByRole("dialog")).toBeInTheDocument());

    const build = fakeIpc.callsFor("build_upgrade_plan")[0].args as {
      args: { selection: { managerId: string; packageId: string }[] };
    };
    const ids = build.args.selection.map((s) => s.packageId).sort();
    expect(ids).toEqual(
      ["globalPackage:@google/gemini-cli", "globalPackage:typescript"].sort(),
    );
    build.args.selection.forEach((s) => expect(s.managerId).toBe("npm"));

    fireEvent.click(within(screen.getByRole("dialog")).getByRole("button", { name: "Upgrade" }));
    await vi.waitFor(() => expect(fakeIpc.called("execute_plan")).toBe(true));

    const exec = fakeIpc.callsFor("execute_plan")[0].args as { args: { plan: UpgradePlan } };
    expect(exec.args.plan).toEqual(cannedPlan);
    await vi.waitFor(() =>
      expect(usePackagesStore.getState().selection["npm"]?.size ?? 0).toBe(0),
    );
  });
});
