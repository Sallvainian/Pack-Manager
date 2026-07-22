import { useEffect } from "react";
import { AppLayout } from "./components/shell/AppLayout";
import { describeError } from "./lib/errors";
import { getAppUpdateState, getState, logFrontendEvent } from "./lib/ipc/client";
import { scheduleLaunchRefresh, subscribeEvents } from "./lib/ipc/events";
import { useAppUpdateStore } from "./store/appUpdate";
import { useManagersStore } from "./store/managers";
import { useOperationsStore } from "./store/operations";
import { usePackagesStore } from "./store/packages";
import { useUiStore } from "./store/ui";

/**
 * Rehydrate stores from `get_state`, then request the launch refresh when
 * enabled (SPEC §5.12 step 3). Backend detection is async — the launch refresh
 * is gated on detection readiness ({@link scheduleLaunchRefresh}) instead of
 * racing it, and a hydration failure is logged (with the real error payload)
 * without cancelling the launch refresh.
 */
export async function bootstrap(): Promise<void> {
  // Matches the backend default (settings.rs `Settings::default`), used only
  // when hydration failed and no settings were ever loaded.
  let autoRefresh = true;
  try {
    const state = await getState();
    // Never clobber a real detection (delivered via `detection:updated` while
    // `get_state` was in flight) with the pre-detection placeholder.
    const current = useManagersStore.getState().detection;
    if (state.detection.managers.length > 0 || !current || current.managers.length === 0) {
      useManagersStore.getState().setDetection(state.detection);
    }
    for (const snapshot of state.snapshots) {
      usePackagesStore.getState().setSnapshot(snapshot.managerId, snapshot);
    }
    useOperationsStore.getState().setRecords(state.operations);
    useUiStore.getState().setSettings(state.settings);
    autoRefresh = state.settings.autoRefreshOnLaunch;
  } catch (e) {
    void logFrontendEvent("error", `bootstrap failed: ${describeError(e)}`);
    autoRefresh = useUiStore.getState().settings?.autoRefreshOnLaunch ?? true;
  }
  // Independent of hydration: a stale update state is worse than none, and a
  // failure here must not affect the launch refresh (DECISIONS D25).
  try {
    useAppUpdateStore.getState().setStatus(await getAppUpdateState());
  } catch (e) {
    void logFrontendEvent("error", `app update state hydration failed: ${describeError(e)}`);
  }
  if (autoRefresh) scheduleLaunchRefresh();
}

function App() {
  useEffect(() => {
    let unlisten: (() => void) | undefined;
    let cancelled = false;

    // Subscribe BEFORE hydrating: `detection:updated` is emitted only after the
    // backend stores detection, so with listeners registered first a `get_state`
    // that returned the placeholder is always followed by a received event —
    // the deferred launch refresh cannot miss it.
    void (async () => {
      try {
        const u = await subscribeEvents();
        if (cancelled) u();
        else unlisten = u;
      } catch (e) {
        void logFrontendEvent("error", `event subscription failed: ${describeError(e)}`);
      }
      try {
        await bootstrap();
      } catch (e) {
        void logFrontendEvent("error", `bootstrap failed: ${describeError(e)}`);
      }
    })();

    return () => {
      cancelled = true;
      unlisten?.();
    };
  }, []);

  return <AppLayout />;
}

export default App;
