import { test, expect } from "../support/fixtures";
import {
  givenPackManagerState,
  openPackManager,
} from "../support/helpers/pack-manager";
import { createPackManagerPage } from "../support/page-objects/pack-manager-page";

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
          outlineStyle: style.outlineStyle,
          outlineWidth: style.outlineWidth,
          outlineColor: style.outlineColor,
        };
      });

      // D35 draws focus as a real outline, not a ring/box-shadow: WebKit does
      // not paint box-shadow on native-appearance form controls, and this app
      // ships in WKWebView. See docs/DECISIONS.md D35.
      expect(focusTreatment.focusVisible).toBe(true);
      expect(focusTreatment.outlineStyle).toBe("solid");
      expect(focusTreatment.outlineWidth).toBe("2px");
      // --color-focus-ring #F4F7FB, a dedicated indicator rather than the
      // accent. The negative guard is the point of the token: focus must stay
      // distinguishable from accent-coloured selection (--color-accent #65A7FF).
      expect(focusTreatment.outlineColor).toBe("rgb(244, 247, 251)");
      expect(focusTreatment.outlineColor).not.toBe("rgb(101, 167, 255)");
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

  test("keeps a selected row and a focused control inside it distinguishable", async ({
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
          packages: [gemini],
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

    await test.step("Given a manager view with one selectable package", async () => {
      await givenPackManagerState(tauriIpc, appState);
      await openPackManager(page);
      const pm = createPackManagerPage(page);
      await expect(pm.dashboardHeading).toBeVisible();
      await pm.viewManager("npm");
      await expect(pm.managerHeading("npm")).toBeVisible();
    });

    const checkbox = page
      .getByTestId(`row-${gemini.id}`)
      .getByRole("checkbox")
      .first();

    await test.step("When the row is selected from the keyboard, leaving that checkbox focused", async () => {
      // Deliberately keyboard-driven: a pointer click sets :focus but not
      // :focus-visible, so the ring this asserts would never be drawn.
      await checkbox.focus();
      await page.keyboard.press("Space");
      await expect(checkbox).toBeChecked();
      await expect(checkbox).toBeFocused();
    });

    await test.step("Then selection reads as the accent wash and focus as the dedicated ring", async () => {
      const rowBackground = await page
        .getByTestId(`row-${gemini.id}`)
        .evaluate((row) => window.getComputedStyle(row).backgroundColor);
      const focus = await checkbox.evaluate((el) => {
        const s = window.getComputedStyle(el);
        return {
          outlineStyle: s.outlineStyle,
          outlineWidth: s.outlineWidth,
          outlineColor: s.outlineColor,
        };
      });

      // --color-accent-subtle #172A46 carries selection.
      expect(rowBackground).toBe("rgb(23, 42, 70)");
      // --color-focus-ring #F4F7FB carries focus, drawn as an outline so it
      // paints on a native checkbox in WebKit/WKWebView too. The two are
      // separate channels — selection never substitutes for focus, nor the
      // reverse.
      expect(focus.outlineStyle).toBe("solid");
      expect(focus.outlineWidth).toBe("2px");
      expect(focus.outlineColor).toBe("rgb(244, 247, 251)");
      expect(focus.outlineColor).not.toBe("rgb(23, 42, 70)");
    });
  });

  test("[P0] paints bright accent fills with ink that clears the 4.5:1 contrast floor", async ({
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
          packages: [gemini],
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

    await test.step("Given one outdated package, so the primary action is enabled", async () => {
      await givenPackManagerState(tauriIpc, appState);
      await openPackManager(page);
      const pm = createPackManagerPage(page);
      await expect(pm.dashboardHeading).toBeVisible();
    });

    await test.step("Then the enabled primary button measures at or above 4.5:1", async () => {
      const updateEverything = page.getByRole("button", {
        name: /^Update Everything \(/,
      });
      await expect(updateEverything).toBeEnabled();

      const measured = await updateEverything.evaluate((el) => {
        const channels = (value: string): number[] => {
          const parts = value.match(/-?\d+(\.\d+)?/g);
          return parts ? parts.slice(0, 3).map(Number) : [0, 0, 0];
        };
        // WCAG 2.1 relative luminance, then the (L1+0.05)/(L2+0.05) ratio.
        const luminance = (rgb: number[]): number => {
          const linear = (raw: number): number => {
            const c = raw / 255;
            return c <= 0.03928 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4);
          };
          return (
            0.2126 * linear(rgb[0]) +
            0.7152 * linear(rgb[1]) +
            0.0722 * linear(rgb[2])
          );
        };
        const style = window.getComputedStyle(el);
        const [lighter, darker] = [
          luminance(channels(style.color)),
          luminance(channels(style.backgroundColor)),
        ].sort((a, b) => b - a);
        return {
          color: style.color,
          backgroundColor: style.backgroundColor,
          ratio: (lighter + 0.05) / (darker + 0.05),
        };
      });

      // --color-accent #65A7FF filled with --color-on-accent #07101D.
      expect(measured.backgroundColor).toBe("rgb(101, 167, 255)");
      expect(measured.color).toBe("rgb(7, 16, 29)");
      // The regression this guards: text-white on this fill measures 2.46:1.
      // The on-accent/on-success tokens exist precisely to prevent it, and
      // release-time contrast is otherwise a by-eye check that missed it.
      expect(measured.color).not.toBe("rgb(255, 255, 255)");
      expect(measured.ratio).toBeGreaterThanOrEqual(4.5);
    });
  });
});
