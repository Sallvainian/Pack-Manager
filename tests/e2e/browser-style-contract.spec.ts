import { test, expect } from "../support/fixtures";
import {
  givenPackManagerState,
  openPackManager,
} from "../support/helpers/pack-manager";

test.describe("browser-rendered style contract", () => {
  test("[P0] AUT-004 applies dark tokens, keyboard focus treatment, and reduced-motion suppression", async ({
    page,
    factories,
    tauriIpc,
  }) => {
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
    const appState = factories.createAppState({
      detection: factories.createDetectionReport({ managers: [npm] }),
      snapshots: [
        factories.createManagerSnapshot({
          managerId: "npm",
          packages: [],
          selfStatus: {
            installed: "11.0.0",
            latest: "11.0.0",
            updateAvailable: false,
          },
          health: [],
        }),
      ],
      operations: [],
      settings: factories.createSettings({ autoRefreshOnLaunch: false }),
    });

    await test.step("Given reduced-motion media and deterministic app state", async () => {
      await givenPackManagerState(tauriIpc, appState);
      await page.emulateMedia({ reducedMotion: "reduce" });
    });

    await test.step("When the real browser renders the application shell", async () => {
      await openPackManager(page);
      await expect(
        page.getByRole("heading", { name: "Packages", level: 1 }),
      ).toBeVisible();
    });

    await test.step("Then the body uses the exact dark surface and text tokens", async () => {
      const bodyTokens = await page.locator("body").evaluate((body) => {
        const style = window.getComputedStyle(body);
        return {
          backgroundColor: style.backgroundColor,
          color: style.color,
        };
      });

      expect(bodyTokens).toEqual({
        // --color-bg-base #090C13 and --color-text-primary #F4F7FB.
        backgroundColor: "rgb(9, 12, 19)",
        color: "rgb(244, 247, 251)",
      });
    });

    await test.step("And keyboard focus receives the dedicated focus-ring treatment", async () => {
      const refreshAll = page.getByRole("button", {
        name: "Refresh All",
        exact: true,
      });

      await refreshAll.focus();
      await expect(refreshAll).toBeFocused();

      const focusTreatment = await refreshAll.evaluate((button) => {
        const style = window.getComputedStyle(button);
        return {
          focusVisible: button.matches(":focus-visible"),
          boxShadow: style.boxShadow,
        };
      });

      expect(focusTreatment.focusVisible).toBe(true);
      expect(focusTreatment.boxShadow).not.toBe("none");
      // --color-focus-ring #F4F7FB, a dedicated indicator rather than the
      // accent. The negative guard is the point of the token: focus must stay
      // distinguishable from accent-coloured selection (--color-accent #65A7FF).
      expect(focusTreatment.boxShadow).toContain("rgb(244, 247, 251)");
      expect(focusTreatment.boxShadow).not.toContain("rgb(101, 167, 255)");
    });

    await test.step("And reduced motion removes CSS transitions and animations", async () => {
      const refreshAll = page.getByRole("button", {
        name: "Refresh All",
        exact: true,
      });
      const motion = await refreshAll.evaluate((button) => {
        const style = window.getComputedStyle(button);
        return {
          transitionProperty: style.transitionProperty,
          transitionDuration: style.transitionDuration,
          animationName: style.animationName,
          animationDuration: style.animationDuration,
        };
      });

      expect(motion).toEqual({
        transitionProperty: "none",
        transitionDuration: "0s",
        animationName: "none",
        animationDuration: "0s",
      });
    });

    // This is a browser DOM/CSS contract only. It does not claim measured
    // contrast compliance or validate the native Tauri package.
  });
});
