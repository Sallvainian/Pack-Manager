import { useEffect } from "react";
import { AppLayout } from "./components/shell/AppLayout";
import { getState, logFrontendEvent, refreshAll } from "./lib/ipc/client";
import { subscribeEvents } from "./lib/ipc/events";
import { useManagersStore } from "./store/managers";
import { useOperationsStore } from "./store/operations";
import { usePackagesStore } from "./store/packages";
import { useUiStore } from "./store/ui";

/**
 * Rehydrate stores from `get_state`, then trigger the launch refresh when
 * enabled (SPEC §5.12 step 3). Live updates arrive via events thereafter.
 */
export async function bootstrap(): Promise<void> {
  const state = await getState();
  useManagersStore.getState().setDetection(state.detection);
  for (const snapshot of state.snapshots) {
    usePackagesStore.getState().setSnapshot(snapshot.managerId, snapshot);
  }
  useOperationsStore.getState().setRecords(state.operations);
  useUiStore.getState().setSettings(state.settings);
  if (state.settings.autoRefreshOnLaunch) {
    await refreshAll();
  }
}

function App() {
  useEffect(() => {
    let unlisten: (() => void) | undefined;
    let cancelled = false;

    void subscribeEvents()
      .then((u) => {
        if (cancelled) u();
        else unlisten = u;
      })
      .catch((e) => void logFrontendEvent("error", `event subscription failed: ${String(e)}`));

    void bootstrap().catch((e) => void logFrontendEvent("error", `bootstrap failed: ${String(e)}`));

    return () => {
      cancelled = true;
      unlisten?.();
    };
  }, []);

  return <AppLayout />;
}

export default App;
