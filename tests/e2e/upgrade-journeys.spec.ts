import type { PlanRequest } from "../../src/lib/ipc/types";
import { test, expect } from "../support/fixtures";
import {
  givenPackManagerState,
  openPackManager,
} from "../support/helpers/pack-manager";
import { createPackManagerPage } from "../support/page-objects/pack-manager-page";

test.describe("upgrade journeys", () => {
  test("[P0] AUT-002 reviews the exact selected-package plan before execution", async ({
    page,
    factories,
    tauriIpc,
  }) => {
    const gemini = factories.createPackage({
      id: "globalPackage:@google/gemini-cli",
      name: "@google/gemini-cli",
      kind: "globalPackage",
      installed: "0.2.0",
      latest: "0.3.0",
      outdated: true,
      pinned: false,
    });
    const typescript = factories.createPackage({
      id: "globalPackage:typescript",
      name: "typescript",
      kind: "globalPackage",
      installed: "5.8.2",
      latest: "5.9.0",
      outdated: true,
      pinned: false,
    });
    const npm = factories.createManagerInfo({
      id: "npm",
      displayName: "npm",
      status: "present",
      binaryPath: "/test/bin/npm",
      canonicalPath: "/test/bin/npm",
      version: "11.0.0",
      managedBy: "standalone",
      selfUpdate: {
        kind: "inBand",
        commandPreview: "npm install -g npm@latest",
      },
    });
    const snapshot = factories.createManagerSnapshot({
      managerId: "npm",
      packages: [gemini, typescript],
      selfStatus: {
        installed: "11.0.0",
        latest: "11.0.0",
        updateAvailable: false,
      },
      health: [],
    });
    const expectedRequest: PlanRequest = {
      selection: [
        { managerId: "npm", packageId: gemini.id },
        { managerId: "npm", packageId: typescript.id },
      ],
      includeSelfUpdates: true,
      includeGreedyCasks: false,
    };
    const exactPreview =
      "npm install -g @google/gemini-cli@latest typescript@latest";
    const returnedPlan = factories.createUpgradePlan({
      planId: "plan-selected-packages",
      request: expectedRequest,
      groups: [
        {
          subject: "npm",
          executor: "npm",
          locks: ["npm", "mise"],
          commands: [
            {
              argvPreview: exactPreview,
              label: "Upgrade selected npm globals",
            },
          ],
          packageIds: [gemini.id, typescript.id],
          selfUpdate: false,
        },
      ],
      excluded: [],
      notes: [],
      warnings: [],
    });
    const appState = factories.createAppState({
      detection: factories.createDetectionReport({ managers: [npm] }),
      snapshots: [snapshot],
      operations: [],
      settings: factories.createSettings({
        autoRefreshOnLaunch: false,
        includeGreedyByDefault: false,
      }),
    });
    const app = createPackManagerPage(page);

    await test.step("Given two authoritative outdated packages and deterministic IPC", async () => {
      await givenPackManagerState(tauriIpc, appState);
      await tauriIpc.respond("build_upgrade_plan", returnedPlan);
      await tauriIpc.respond("execute_plan", {
        opIds: ["op-selected-packages"],
      });
    });

    await test.step("When the user selects both packages and requests a bulk upgrade", async () => {
      await openPackManager(page);
      await expect(app.dashboardHeading).toBeVisible();

      await app.viewManager(npm.displayName);
      await expect(app.managerHeading(npm.displayName)).toBeVisible();

      const geminiCheckbox = page.getByRole("checkbox", {
        name: "Select @google/gemini-cli",
      });
      const typescriptCheckbox = page.getByRole("checkbox", {
        name: "Select typescript",
      });

      await geminiCheckbox.click();
      await expect(geminiCheckbox).toBeChecked();
      await typescriptCheckbox.click();
      await expect(typescriptCheckbox).toBeChecked();

      await page
        .getByRole("button", { name: "Upgrade selected", exact: true })
        .click();
    });

    await test.step("Then the sheet shows the exact backend preview", async () => {
      const dialog = page.getByRole("dialog", { name: "Upgrade plan" });
      await expect(dialog).toBeVisible();
      await expect(
        dialog.getByRole("heading", { name: "Upgrade 2 packages" }),
      ).toBeVisible();
      await expect(
        dialog.getByText(exactPreview, { exact: true }),
      ).toBeVisible();

      await expect
        .poll(() => tauriIpc.callsFor("build_upgrade_plan"))
        .toEqual([
          {
            command: "build_upgrade_plan",
            args: { args: expectedRequest },
          },
        ]);
    });

    await test.step("And confirmation passes the unchanged returned plan to execution", async () => {
      const dialog = page.getByRole("dialog", { name: "Upgrade plan" });
      await dialog
        .getByRole("button", { name: "Upgrade", exact: true })
        .click();

      await expect
        .poll(() => tauriIpc.callsFor("execute_plan"))
        .toEqual([
          {
            command: "execute_plan",
            args: { args: { plan: returnedPlan } },
          },
        ]);
      await expect(dialog).toBeHidden();
    });
  });

  test("[P0] AUT-003 executes a one-row Upgrade immediately without a plan dialog", async ({
    page,
    factories,
    tauriIpc,
  }) => {
    const eslint = factories.createPackage({
      id: "globalPackage:eslint",
      name: "eslint",
      kind: "globalPackage",
      installed: "9.30.0",
      latest: "9.31.0",
      outdated: true,
      pinned: false,
    });
    const npm = factories.createManagerInfo({
      id: "npm",
      displayName: "npm",
      status: "present",
      binaryPath: "/test/bin/npm",
      canonicalPath: "/test/bin/npm",
      version: "11.0.0",
      managedBy: "standalone",
      selfUpdate: {
        kind: "inBand",
        commandPreview: "npm install -g npm@latest",
      },
    });
    const snapshot = factories.createManagerSnapshot({
      managerId: "npm",
      packages: [eslint],
      selfStatus: {
        installed: "11.0.0",
        latest: "11.0.0",
        updateAvailable: false,
      },
      health: [],
    });
    const expectedRequest: PlanRequest = {
      selection: [{ managerId: "npm", packageId: eslint.id }],
      includeSelfUpdates: false,
      includeGreedyCasks: false,
    };
    const returnedPlan = factories.createUpgradePlan({
      planId: "plan-one-row",
      request: expectedRequest,
      groups: [
        {
          subject: "npm",
          executor: "npm",
          locks: ["npm", "mise"],
          commands: [
            {
              argvPreview: "npm install -g eslint@latest",
              label: "Upgrade eslint",
            },
          ],
          packageIds: [eslint.id],
          selfUpdate: false,
        },
      ],
      excluded: [],
      notes: [],
      warnings: [],
    });
    const appState = factories.createAppState({
      detection: factories.createDetectionReport({ managers: [npm] }),
      snapshots: [snapshot],
      operations: [],
      settings: factories.createSettings({
        autoRefreshOnLaunch: false,
        includeGreedyByDefault: true,
      }),
    });
    const app = createPackManagerPage(page);

    await test.step("Given one authoritative outdated package and deterministic IPC", async () => {
      await givenPackManagerState(tauriIpc, appState);
      await tauriIpc.respond("build_upgrade_plan", returnedPlan);
      await tauriIpc.respond("execute_plan", { opIds: ["op-one-row"] });
    });

    await test.step("When the user invokes Upgrade on that package row", async () => {
      await openPackManager(page);
      await app.viewManager(npm.displayName);
      await expect(app.managerHeading(npm.displayName)).toBeVisible();

      const row = app.packageRow(eslint.id);
      await expect(row).toBeVisible();
      await expect(
        page.getByRole("dialog", { name: "Upgrade plan" }),
      ).toHaveCount(0);
      await row.getByRole("button", { name: "Upgrade", exact: true }).click();
    });

    await test.step("Then the one-item request and returned plan execute without confirmation", async () => {
      await expect
        .poll(() => tauriIpc.callsFor("build_upgrade_plan"))
        .toEqual([
          {
            command: "build_upgrade_plan",
            args: { args: expectedRequest },
          },
        ]);
      await expect
        .poll(() => tauriIpc.callsFor("execute_plan"))
        .toEqual([
          {
            command: "execute_plan",
            args: { args: { plan: returnedPlan } },
          },
        ]);
      await expect(
        page.getByRole("dialog", { name: "Upgrade plan" }),
      ).toHaveCount(0);
    });
  });
});
