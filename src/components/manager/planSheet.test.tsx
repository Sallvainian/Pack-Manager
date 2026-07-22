import "../../test/setup";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { act, fireEvent, render, screen, within } from "@testing-library/react";

vi.mock("../../lib/ipc/bridge", () => import("../../test/fakeIpc"));

import { ManagerPane } from "./ManagerPane";
import { UpgradePlanSheet } from "../dialogs/UpgradePlanSheet";
import type { UpgradePlan } from "../../lib/ipc/types";
import { resetStores, useManagersStore, usePackagesStore, useUiStore } from "../../store";
import * as fakeIpc from "../../test/fakeIpc";
import { defaultSettings, detectionReport, npmSnapshot } from "../../test/fixtures";

function deferred<T>() {
  let resolve!: (value: T) => void;
  let reject!: (reason?: unknown) => void;
  const promise = new Promise<T>((resolvePromise, rejectPromise) => {
    resolve = resolvePromise;
    reject = rejectPromise;
  });
  return { promise, resolve, reject };
}

beforeEach(() => {
  fakeIpc.reset();
  resetStores();
  useManagersStore.getState().setDetection(detectionReport);
  useUiStore.getState().setSettings(defaultSettings);
});

const basePlan: UpgradePlan = {
  planId: "plan-1",
  request: {
    selection: [
      { managerId: "brew", packageId: "formula:dolt" },
      { managerId: "brew", packageId: "formula:deno" },
      { managerId: "brew", packageId: "caskGreedy:openusage" },
      { managerId: "mise", packageId: "tool:rust" },
      { managerId: "npm", packageId: "globalPackage:typescript" },
    ],
    includeSelfUpdates: true,
    includeGreedyCasks: false,
  },
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
      request: {
        ...basePlan.request,
        includeGreedyCasks: true,
      },
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

describe("stale_plan_refresh_requires_renewed_confirmation", () => {
  it("preserves non-default toggles and executes exactly the refreshed plan only after a second click", async () => {
    const stalePlan: UpgradePlan = {
      ...basePlan,
      planId: "plan-stale",
      request: {
        ...basePlan.request,
        includeSelfUpdates: false,
        includeGreedyCasks: true,
      },
    };
    const refreshedPlan: UpgradePlan = {
      ...stalePlan,
      planId: "plan-refreshed",
      groups: [
        {
          ...basePlan.groups[0],
          commands: [{ argvPreview: "brew upgrade dolt-new", label: "Upgrade formulae" }],
          packageIds: ["formula:dolt-new"],
        },
      ],
      excluded: [],
      notes: [],
      warnings: [],
    };
    fakeIpc.respond("execute_plan", () => {
      throw {
        code: "plan_stale",
        message: "The available updates changed. Review the refreshed plan and confirm again.",
      };
    });
    fakeIpc.respond("build_upgrade_plan", () => refreshedPlan);

    render(<UpgradePlanSheet plan={stalePlan} />);
    fireEvent.click(screen.getByRole("button", { name: "Upgrade" }));

    await vi.waitFor(() =>
      expect(screen.getByText("brew upgrade dolt-new")).toBeInTheDocument(),
    );
    expect(screen.getByRole("alert")).toHaveTextContent("Review the refreshed plan");
    expect(screen.getByRole("dialog")).toBeInTheDocument();
    expect(fakeIpc.callsFor("execute_plan")).toHaveLength(1);
    expect(fakeIpc.callsFor("build_upgrade_plan")[0].args).toEqual({
      args: stalePlan.request,
    });
    expect(screen.getByLabelText("Include manager self-updates")).not.toBeChecked();
    expect(screen.getByLabelText("Include self-updating casks")).toBeChecked();

    fakeIpc.respond("execute_plan", () => ({ opIds: ["op-refreshed"] }));
    fireEvent.click(screen.getByRole("button", { name: "Upgrade" }));
    await vi.waitFor(() => expect(fakeIpc.callsFor("execute_plan")).toHaveLength(2));

    const secondExecute = fakeIpc.callsFor("execute_plan")[1].args as {
      args: { plan: UpgradePlan };
    };
    expect(secondExecute.args.plan).toEqual(refreshedPlan);
  });

  it("does not rebuild or set dialog state when a stale execution returns after Cancel unmounts it", async () => {
    const pending = deferred<{ opIds: string[] }>();
    fakeIpc.respond("execute_plan", () => pending.promise);
    fakeIpc.respond("build_upgrade_plan", () => basePlan);

    function DialogHarness() {
      const dialog = useUiStore((s) => s.dialog);
      return dialog.kind === "upgradePlan" ? <UpgradePlanSheet plan={dialog.plan} /> : null;
    }

    useUiStore.getState().openDialog({ kind: "upgradePlan", plan: basePlan });
    render(<DialogHarness />);

    fireEvent.click(screen.getByRole("button", { name: "Upgrade" }));
    await vi.waitFor(() => expect(fakeIpc.callsFor("execute_plan")).toHaveLength(1));

    fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
    await vi.waitFor(() => expect(screen.queryByRole("dialog")).not.toBeInTheDocument());

    await act(async () => {
      pending.reject({
        code: "plan_stale",
        message: "The available updates changed. Review the refreshed plan and confirm again.",
      });
      await pending.promise.catch(() => undefined);
    });

    expect(fakeIpc.callsFor("build_upgrade_plan")).toHaveLength(0);
    expect(useUiStore.getState().dialog).toEqual({ kind: "none" });
  });
});

describe("plan_rebuild_readiness_gate", () => {
  const greedyPlan: UpgradePlan = {
    ...basePlan,
    planId: "plan-greedy",
    request: {
      ...basePlan.request,
      includeGreedyCasks: true,
    },
    groups: [
      {
        ...basePlan.groups[0],
        commands: [
          {
            argvPreview: "brew upgrade --cask --greedy openusage",
            label: "Upgrade greedy casks",
          },
        ],
        packageIds: ["caskGreedy:openusage"],
      },
    ],
    excluded: [],
    notes: [],
    warnings: [],
  };

  it("disables toggles and Upgrade while a rebuild is pending", async () => {
    const pending = deferred<UpgradePlan>();
    fakeIpc.respond("build_upgrade_plan", () => pending.promise);
    fakeIpc.respond("execute_plan", () => ({ opIds: ["op-should-not-run"] }));

    render(<UpgradePlanSheet plan={basePlan} />);
    fireEvent.click(screen.getByLabelText("Include self-updating casks"));

    expect(screen.getByText("Refreshing plan…")).toBeInTheDocument();
    expect(screen.getByLabelText("Include manager self-updates")).toBeDisabled();
    expect(screen.getByLabelText("Include self-updating casks")).toBeDisabled();
    expect(screen.getByRole("button", { name: "Upgrade" })).toBeDisabled();

    fireEvent.click(screen.getByRole("button", { name: "Upgrade" }));
    expect(fakeIpc.callsFor("execute_plan")).toHaveLength(0);

    pending.resolve(greedyPlan);
    await vi.waitFor(() =>
      expect(screen.getByText("brew upgrade --cask --greedy openusage")).toBeInTheDocument(),
    );
    expect(screen.getByRole("button", { name: "Upgrade" })).toBeEnabled();
  });

  it("keeps execution blocked after failure and makes Refresh plan rebuild only", async () => {
    let buildCount = 0;
    fakeIpc.respond("build_upgrade_plan", () => {
      buildCount += 1;
      if (buildCount === 1) throw new Error("offline");
      return greedyPlan;
    });
    fakeIpc.respond("execute_plan", () => ({ opIds: ["op-after-review"] }));

    render(<UpgradePlanSheet plan={basePlan} />);
    fireEvent.click(screen.getByLabelText("Include self-updating casks"));

    await vi.waitFor(() =>
      expect(screen.getByRole("button", { name: "Refresh plan" })).toBeInTheDocument(),
    );
    expect(screen.getByRole("button", { name: "Upgrade" })).toBeDisabled();
    expect(fakeIpc.callsFor("execute_plan")).toHaveLength(0);

    fireEvent.click(screen.getByRole("button", { name: "Refresh plan" }));
    await vi.waitFor(() =>
      expect(screen.getByText("brew upgrade --cask --greedy openusage")).toBeInTheDocument(),
    );
    expect(fakeIpc.callsFor("execute_plan")).toHaveLength(0);
    expect(screen.getByRole("button", { name: "Upgrade" })).toBeEnabled();

    fireEvent.click(screen.getByRole("button", { name: "Upgrade" }));
    await vi.waitFor(() => expect(fakeIpc.callsFor("execute_plan")).toHaveLength(1));
    const execute = fakeIpc.callsFor("execute_plan")[0].args as {
      args: { plan: UpgradePlan };
    };
    expect(execute.args.plan).toEqual(greedyPlan);
  });

  it("requires a fresh review after a non-stale execute error before allowing another click", async () => {
    const rebuiltPlan: UpgradePlan = {
      ...basePlan,
      planId: "plan-after-execute-error",
      groups: [
        {
          ...basePlan.groups[0],
          commands: [{ argvPreview: "brew upgrade reviewed", label: "Reviewed plan" }],
          packageIds: ["formula:reviewed"],
        },
      ],
      excluded: [],
      notes: [],
      warnings: [],
    };
    fakeIpc.respond("execute_plan", () => {
      throw { code: "internal", message: "The plan could not be queued." };
    });
    fakeIpc.respond("build_upgrade_plan", () => rebuiltPlan);

    render(<UpgradePlanSheet plan={basePlan} />);
    fireEvent.click(screen.getByRole("button", { name: "Upgrade" }));

    await vi.waitFor(() =>
      expect(screen.getByRole("button", { name: "Refresh plan" })).toBeInTheDocument(),
    );
    expect(screen.getByRole("button", { name: "Upgrade" })).toBeDisabled();
    expect(fakeIpc.callsFor("execute_plan")).toHaveLength(1);

    fireEvent.click(screen.getByRole("button", { name: "Upgrade" }));
    expect(fakeIpc.callsFor("execute_plan")).toHaveLength(1);

    fireEvent.click(screen.getByRole("button", { name: "Refresh plan" }));
    await vi.waitFor(() => expect(screen.getByText("brew upgrade reviewed")).toBeInTheDocument());
    expect(fakeIpc.callsFor("execute_plan")).toHaveLength(1);
    expect(screen.getByRole("button", { name: "Upgrade" })).toBeEnabled();

    fakeIpc.respond("execute_plan", () => ({ opIds: ["op-reviewed"] }));
    fireEvent.click(screen.getByRole("button", { name: "Upgrade" }));
    await vi.waitFor(() => expect(fakeIpc.callsFor("execute_plan")).toHaveLength(2));
    const retry = fakeIpc.callsFor("execute_plan")[1].args as {
      args: { plan: UpgradePlan };
    };
    expect(retry.args.plan).toEqual(rebuiltPlan);
  });

  it("ignores an older rebuild response that arrives after the latest request", async () => {
    const first = deferred<UpgradePlan>();
    const second = deferred<UpgradePlan>();
    let buildCount = 0;
    fakeIpc.respond("build_upgrade_plan", () => {
      buildCount += 1;
      return buildCount === 1 ? first.promise : second.promise;
    });

    const latestPlan: UpgradePlan = {
      ...basePlan,
      planId: "plan-latest",
      groups: [
        {
          ...basePlan.groups[0],
          commands: [{ argvPreview: "brew upgrade latest", label: "Latest plan" }],
        },
      ],
    };
    const obsoletePlan: UpgradePlan = {
      ...greedyPlan,
      planId: "plan-obsolete",
      groups: [
        {
          ...greedyPlan.groups[0],
          commands: [{ argvPreview: "brew upgrade obsolete", label: "Obsolete plan" }],
        },
      ],
    };

    render(<UpgradePlanSheet plan={basePlan} />);
    const greedyToggle = screen.getByLabelText("Include self-updating casks");

    fireEvent.click(greedyToggle);

    // The real control is disabled while pending. Temporarily bypass that DOM
    // guard to exercise the defensive latest-response sequencing directly.
    (greedyToggle as HTMLInputElement).disabled = false;
    fireEvent.click(greedyToggle);
    expect(fakeIpc.callsFor("build_upgrade_plan")).toHaveLength(2);

    second.resolve(latestPlan);
    await vi.waitFor(() => expect(screen.getByText("brew upgrade latest")).toBeInTheDocument());

    first.resolve(obsoletePlan);
    await act(async () => {
      await first.promise;
    });
    expect(screen.getByText("brew upgrade latest")).toBeInTheDocument();
    expect(screen.queryByText("brew upgrade obsolete")).not.toBeInTheDocument();
    expect(screen.getByLabelText("Include self-updating casks")).not.toBeChecked();
  });
});

describe("all_packages_plan_request", () => {
  const allPackagesPlan: UpgradePlan = {
    ...basePlan,
    planId: "plan-all-packages",
    request: {
      ...basePlan.request,
      selection: null,
    },
  };

  it("preserves selection null when a toggle rebuilds the plan", async () => {
    const toggledPlan: UpgradePlan = {
      ...allPackagesPlan,
      planId: "plan-all-packages-greedy",
      request: {
        ...allPackagesPlan.request,
        includeGreedyCasks: true,
      },
    };
    fakeIpc.respond("build_upgrade_plan", () => toggledPlan);

    render(<UpgradePlanSheet plan={allPackagesPlan} />);
    fireEvent.click(screen.getByLabelText("Include self-updating casks"));

    await vi.waitFor(() => expect(fakeIpc.callsFor("build_upgrade_plan")).toHaveLength(1));
    expect(fakeIpc.callsFor("build_upgrade_plan")[0].args).toEqual({
      args: {
        selection: null,
        includeSelfUpdates: true,
        includeGreedyCasks: true,
      },
    });
  });

  it("preserves selection null through stale recovery and a manual retry", async () => {
    const refreshedPlan: UpgradePlan = {
      ...allPackagesPlan,
      planId: "plan-all-packages-refreshed",
      groups: [
        {
          ...basePlan.groups[0],
          commands: [{ argvPreview: "brew upgrade all-reviewed", label: "Reviewed plan" }],
        },
      ],
      excluded: [],
      notes: [],
      warnings: [],
    };
    let buildCount = 0;
    fakeIpc.respond("execute_plan", () => {
      throw {
        code: "plan_stale",
        message: "The available updates changed. Review the refreshed plan and confirm again.",
      };
    });
    fakeIpc.respond("build_upgrade_plan", () => {
      buildCount += 1;
      if (buildCount === 1) throw new Error("offline");
      return refreshedPlan;
    });

    render(<UpgradePlanSheet plan={allPackagesPlan} />);
    fireEvent.click(screen.getByRole("button", { name: "Upgrade" }));

    await vi.waitFor(() =>
      expect(screen.getByRole("button", { name: "Refresh plan" })).toBeInTheDocument(),
    );
    expect(fakeIpc.callsFor("build_upgrade_plan")[0].args).toEqual({
      args: allPackagesPlan.request,
    });
    expect(fakeIpc.callsFor("execute_plan")).toHaveLength(1);

    fireEvent.click(screen.getByRole("button", { name: "Refresh plan" }));
    await vi.waitFor(() =>
      expect(screen.getByText("brew upgrade all-reviewed")).toBeInTheDocument(),
    );
    expect(fakeIpc.callsFor("build_upgrade_plan")[1].args).toEqual({
      args: allPackagesPlan.request,
    });
    expect(fakeIpc.callsFor("execute_plan")).toHaveLength(1);

    fakeIpc.respond("execute_plan", () => ({ opIds: ["op-all-reviewed"] }));
    fireEvent.click(screen.getByRole("button", { name: "Upgrade" }));
    await vi.waitFor(() => expect(fakeIpc.callsFor("execute_plan")).toHaveLength(2));
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
      request: {
        selection: [
          { managerId: "npm", packageId: "globalPackage:@google/gemini-cli" },
          { managerId: "npm", packageId: "globalPackage:typescript" },
        ],
        includeSelfUpdates: true,
        includeGreedyCasks: false,
      },
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
