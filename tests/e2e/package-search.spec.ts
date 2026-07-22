import { test, expect } from "../support/fixtures";
import {
  givenPackManagerState,
  openPackManager,
} from "../support/helpers/pack-manager";
import { createPackManagerPage } from "../support/page-objects/pack-manager-page";

test.describe("package search", () => {
  test("filters a manager package list by its visible package name", async ({
    page,
    factories,
    tauriIpc,
  }) => {
    const packageUnderTest = factories.createPackage({
      id: "globalPackage:pack-manager-audit",
      name: "pack-manager-audit",
      kind: "globalPackage",
      installed: "1.2.3",
      latest: "1.3.0",
      outdated: true,
      pinned: false,
    });
    const unrelatedPackage = factories.createPackage({
      id: "globalPackage:unrelated-cli",
      name: "unrelated-cli",
      kind: "globalPackage",
      installed: "4.5.6",
      latest: "4.6.0",
      outdated: true,
      pinned: false,
    });
    const npmManager = factories.createManagerInfo({
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
    const npmSnapshot = factories.createManagerSnapshot({
      managerId: npmManager.id,
      packages: [packageUnderTest, unrelatedPackage],
      health: [],
    });
    const appState = factories.createAppState({
      detection: factories.createDetectionReport({ managers: [npmManager] }),
      snapshots: [npmSnapshot],
      operations: [],
      settings: factories.createSettings({ autoRefreshOnLaunch: false }),
    });
    const app = createPackManagerPage(page);

    await test.step("Given deterministic manager and package data", async () => {
      await givenPackManagerState(tauriIpc, appState);
    });

    await test.step("When the user opens npm and searches for one package", async () => {
      await openPackManager(page);
      await expect(app.dashboardHeading).toBeVisible();

      await app.viewManager(npmManager.displayName);
      await expect(app.managerHeading(npmManager.displayName)).toBeVisible();
      await expect(app.packagesTable(npmManager.id)).toBeVisible();
      await expect(app.packageRow(packageUnderTest.id)).toBeVisible();
      await expect(app.packageRow(unrelatedPackage.id)).toBeVisible();

      await app.searchPackages(packageUnderTest.name);
    });

    await test.step("Then only the matching package row remains visible", async () => {
      await expect(app.packageRow(packageUnderTest.id)).toBeVisible();
      await expect(app.packageRow(packageUnderTest.id)).toContainText(
        packageUnderTest.name,
      );
      await expect(app.packageRow(unrelatedPackage.id)).toBeHidden();
    });
  });
});
