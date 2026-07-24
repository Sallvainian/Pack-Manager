import { describe, expect, it } from "vitest";

import { createPackManagerFactories } from "./pack-manager";

describe("package factory outdated verdict", () => {
  it("[P0] does not infer outdatedness from differing version strings", () => {
    const factories = createPackManagerFactories();

    const pkg = factories.createPackage({
      installed: "1.6.2.dev0",
      latest: "2.0.14-1",
    });

    expect(pkg.outdated).toBe(false);
  });

  it("[P0] preserves an explicit package-manager verdict", () => {
    const factories = createPackManagerFactories();

    const explicitlyOutdated = factories.createPackage({
      installed: "stable",
      latest: "stable",
      outdated: true,
    });
    const explicitlyClean = factories.createPackage({
      installed: "1.0.0",
      latest: "9.0.0",
      outdated: false,
    });

    expect(explicitlyOutdated.outdated).toBe(true);
    expect(explicitlyClean.outdated).toBe(false);
  });
});
