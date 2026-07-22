/**
 * bridge.ts — the SINGLE importer of `@tauri-apps/api` in the whole frontend.
 *
 * Every command call goes through {@link invoke}; every event subscription
 * through {@link listen}. Nothing else imports `@tauri-apps/api` so this module
 * is the one mock seam: tests replace it via `vi.mock('../lib/ipc/bridge', () =>
 * import('../test/fakeIpc'))` (SPEC §5.11, §7.5).
 */
export { invoke } from "@tauri-apps/api/core";
export { listen } from "@tauri-apps/api/event";
export type { UnlistenFn } from "@tauri-apps/api/event";
