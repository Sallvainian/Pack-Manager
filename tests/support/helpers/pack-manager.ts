import type { Page } from "@playwright/test";

import type { AppState } from "../../../src/lib/ipc/types";
import type { FakeTauriIpc } from "../fixtures";

/**
 * Arrange the browser-side Tauri seam before the app loads.
 *
 * This helper controls only the frontend test double. It does not start or
 * make claims about the native Tauri runtime or any package-manager process.
 */
export async function givenPackManagerState(
  tauriIpc: FakeTauriIpc,
  state: AppState,
): Promise<void> {
  await tauriIpc.respond("get_state", state);
}

/** Navigate after all deterministic IPC responses have been arranged. */
export async function openPackManager(page: Page): Promise<void> {
  await page.goto("/");
}
