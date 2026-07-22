import { defineConfig, devices } from "@playwright/test";

declare const process: {
  env: Record<string, string | undefined>;
  loadEnvFile(path?: string): void;
};

try {
  process.loadEnvFile(".env");
} catch (error) {
  if ((error as { code?: string }).code !== "ENOENT") throw error;
}

const localBaseURL = "http://127.0.0.1:1420";
const baseURL = process.env.BASE_URL || localBaseURL;
const explicitlyEnabled = (value: string | undefined): boolean =>
  /^(1|true|yes|on)$/i.test(value?.trim() ?? "");

function parseBaseURL(value: string): URL {
  let parsed: URL;
  try {
    parsed = new URL(value);
  } catch {
    throw new Error(`BASE_URL must be a valid URL; received ${value}`);
  }

  if (parsed.protocol !== "http:" && parsed.protocol !== "https:") {
    throw new Error(
      `BASE_URL must use http or https; received ${parsed.protocol}`,
    );
  }
  return parsed;
}

function isLoopback(hostname: string): boolean {
  return (
    hostname === "localhost" ||
    hostname.endsWith(".localhost") ||
    /^127(?:\.\d{1,3}){3}$/.test(hostname) ||
    hostname === "[::1]"
  );
}

const parsedBaseURL = parseBaseURL(baseURL);
const isCI = explicitlyEnabled(process.env.CI);
const allowRemoteE2E = explicitlyEnabled(process.env.ALLOW_REMOTE_E2E);

if (!isLoopback(parsedBaseURL.hostname) && !allowRemoteE2E) {
  throw new Error(
    `Refusing non-loopback BASE_URL ${parsedBaseURL.origin}. ` +
      "Set ALLOW_REMOTE_E2E=1 only when that target is intentional.",
  );
}

export default defineConfig({
  testDir: "./tests/e2e",
  outputDir: "./test-results",
  timeout: 60_000,
  expect: {
    timeout: 10_000,
  },
  fullyParallel: true,
  forbidOnly: isCI,
  retries: isCI ? 2 : 0,
  workers: isCI ? 2 : undefined,
  reporter: [
    ["list"],
    ["html", { outputFolder: "playwright-report", open: "never" }],
    ["junit", { outputFile: "test-results/results.xml" }],
  ],
  use: {
    baseURL,
    actionTimeout: 15_000,
    navigationTimeout: 30_000,
    trace: "retain-on-failure-and-retries",
    screenshot: "only-on-failure",
    video: "retain-on-failure",
  },
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
    {
      name: "webkit",
      use: { ...devices["Desktop Safari"] },
    },
  ],
  webServer:
    parsedBaseURL.origin === localBaseURL
      ? {
          command: "npm run dev -- --host 127.0.0.1",
          url: localBaseURL,
          reuseExistingServer: !isCI,
          timeout: 120_000,
        }
      : undefined,
});
