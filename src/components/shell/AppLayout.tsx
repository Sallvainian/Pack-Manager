import { ActivityDrawer } from "../activity/ActivityDrawer";
import { DashboardView } from "../dashboard/DashboardView";
import { DialogHost } from "../dialogs/DialogHost";
import { HistoryView } from "../history/HistoryView";
import { ManagerPane } from "../manager/ManagerPane";
import { SettingsView } from "../settings/SettingsView";
import { useUiStore } from "../../store/ui";
import { Sidebar } from "./Sidebar";
import { StatusBar } from "./StatusBar";
import { ToastHost } from "./ToastHost";

/** Store-routed main view (no router lib — SPEC §4.4). */
function MainView() {
  const view = useUiStore((s) => s.view);
  switch (view.kind) {
    case "dashboard":
      return <DashboardView />;
    case "manager":
      return <ManagerPane managerId={view.managerId} />;
    case "history":
      return <HistoryView />;
    case "settings":
      return <SettingsView />;
  }
}

export function AppLayout() {
  return (
    <div className="flex h-screen w-screen flex-col overflow-hidden bg-bg-base text-text-primary">
      <div className="flex min-h-0 flex-1">
        <Sidebar />
        <div className="flex min-w-0 flex-1 flex-col">
          <main className="min-h-0 flex-1 overflow-hidden">
            <MainView />
          </main>
          <ActivityDrawer />
        </div>
      </div>
      <StatusBar />
      <DialogHost />
      <ToastHost />
    </div>
  );
}
