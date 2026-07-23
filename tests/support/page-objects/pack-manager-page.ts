import type { Locator, Page } from "@playwright/test";

import type { ManagerId } from "../../../src/lib/ipc/types";

/**
 * A small, composition-friendly page object for the browser-rendered shell.
 * It deliberately uses accessible roles first and the package row's stable
 * test id where a dynamic row needs an unambiguous identity.
 */
export function createPackManagerPage(page: Page) {
  const dashboardHeading = page.getByRole("heading", {
    name: "Packages",
    level: 1,
  });

  function managerCard(displayName: string): Locator {
    return page.getByRole("region", { name: displayName, exact: true });
  }

  function managerHeading(displayName: string): Locator {
    return page.getByRole("heading", {
      name: displayName,
      exact: true,
      level: 1,
    });
  }

  function packagesTable(managerId: ManagerId): Locator {
    return page.getByRole("table", { name: `${managerId} packages` });
  }

  function packageRow(packageId: string): Locator {
    return page.getByTestId(`row-${packageId}`);
  }

  async function viewManager(displayName: string): Promise<void> {
    await managerCard(displayName)
      .getByRole("button", { name: displayName, exact: true })
      .click();
  }

  async function searchPackages(query: string): Promise<void> {
    await page.getByRole("searchbox", { name: "Search packages" }).fill(query);
  }

  return {
    dashboardHeading,
    managerCard,
    managerHeading,
    packageRow,
    packagesTable,
    searchPackages,
    viewManager,
  };
}

export type PackManagerPage = ReturnType<typeof createPackManagerPage>;
