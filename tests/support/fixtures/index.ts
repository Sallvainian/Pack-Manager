import { test as base, type Response, type Route } from "@playwright/test";

import {
  createPackManagerFactories,
  type PackManagerFactories,
} from "./factories/pack-manager";
import { FakeTauriIpc } from "./tauri-ipc";

declare const process: {
  env: Record<string, string | undefined>;
};

const localBaseURL = "http://127.0.0.1:1420";

export interface NetworkIsolation {
  /** Requests rejected before they can leave the configured test origin. */
  readonly blockedRequestUrls: readonly string[];
}

interface HttpErrorResponse {
  method: string;
  status: number;
  url: string;
}

export interface PackManagerFixtures {
  /** Per-test deterministic, schema-complete model factories. */
  factories: PackManagerFactories;
  /** Automatic same-origin guard and its per-test rejection ledger. */
  networkIsolation: NetworkIsolation;
  /** Browser-local replacement for Tauri command and event transport. */
  tauriIpc: FakeTauriIpc;
}

function explicitlyEnabled(value: string | undefined): boolean {
  return /^(1|true|yes|on)$/i.test(value?.trim() ?? "");
}

function seedForTest(identity: string): number {
  let hash = 2_166_136_261;
  for (let index = 0; index < identity.length; index += 1) {
    hash ^= identity.charCodeAt(index);
    hash = Math.imul(hash, 16_777_619);
  }
  return hash >>> 0;
}

const packManagerFixture = base.extend<PackManagerFixtures>({
  networkIsolation: [
    async ({ context, baseURL }, use, testInfo) => {
      const blockedRequestUrls: string[] = [];
      const networkIsolation: NetworkIsolation = { blockedRequestUrls };
      const httpErrors: HttpErrorResponse[] = [];
      const seenHttpErrors = new Set<string>();
      const allowedOrigin = new URL(baseURL ?? localBaseURL).origin;
      const routePattern = "**/*";
      const allowRemoteE2E = explicitlyEnabled(process.env.ALLOW_REMOTE_E2E);
      const sameOriginOnly = async (route: Route): Promise<void> => {
        const requestUrl = route.request().url();
        let requestOrigin: string;
        try {
          requestOrigin = new URL(requestUrl).origin;
        } catch {
          blockedRequestUrls.push(requestUrl);
          await route.abort("blockedbyclient");
          return;
        }

        if (requestOrigin === allowedOrigin) {
          await route.continue();
          return;
        }

        blockedRequestUrls.push(requestUrl);
        await route.abort("blockedbyclient");
      };

      const captureHttpError = (response: Response): void => {
        const status = response.status();
        if (status < 400) return;

        const error: HttpErrorResponse = {
          method: response.request().method(),
          status,
          url: response.url(),
        };
        const key = `${error.method}:${error.status}:${error.url}`;
        if (seenHttpErrors.has(key)) return;
        seenHttpErrors.add(key);
        httpErrors.push(error);
      };

      context.on("response", captureHttpError);
      if (!allowRemoteE2E) await context.route(routePattern, sameOriginOnly);

      let httpMonitorError: Error | undefined;
      try {
        await use(networkIsolation);
      } finally {
        context.off("response", captureHttpError);
        if (!allowRemoteE2E)
          await context.unroute(routePattern, sameOriginOnly);

        if (httpErrors.length > 0) {
          await testInfo.attach("network-errors.json", {
            body: JSON.stringify(httpErrors, null, 2),
            contentType: "application/json",
          });

          const testAlreadyFailed = [
            "failed",
            "timedOut",
            "interrupted",
            "skipped",
          ].includes(testInfo.status ?? "");
          const monitoringDisabled = testInfo.annotations.some(
            (annotation) => annotation.type === "skipNetworkMonitoring",
          );
          if (!testAlreadyFailed && !monitoringDisabled) {
            const summary = httpErrors
              .map((error) => `  ${error.method} ${error.status} ${error.url}`)
              .join("\n");
            httpMonitorError = new Error(
              `HTTP errors detected during browser test:\n${summary}`,
            );
          }
        }
      }

      if (httpMonitorError) throw httpMonitorError;
    },
    { auto: true },
  ],

  factories: async ({}, use, testInfo) => {
    const identity = testInfo.titlePath.join("::");
    const factories = createPackManagerFactories(seedForTest(identity));

    try {
      await use(factories);
    } finally {
      factories.cleanup();
    }
  },

  tauriIpc: [
    async ({ page, factories }, use) => {
      const tauriIpc = new FakeTauriIpc(page, {
        get_state: factories.createAppState(),
        get_app_update_state: factories.createAppUpdateStatus(),
        log_frontend_event: null,
      });
      await tauriIpc.install();

      try {
        await use(tauriIpc);
      } finally {
        await tauriIpc.cleanup();
      }
    },
    { auto: true },
  ],
});

/**
 * Project-wide test object with local Pack-Manager fixtures. Authentication,
 * HTTP API client, and database fixtures are intentionally absent.
 */
export const test = packManagerFixture;

export { expect } from "@playwright/test";
export {
  DEFAULT_FACTORY_SEED,
  createPackManagerFactories,
  type FactoryEntity,
  type FactoryEntityKind,
  type PackManagerFactories,
} from "./factories/pack-manager";
export { FakeTauriIpc, type TauriIpcCall } from "./tauri-ipc";
