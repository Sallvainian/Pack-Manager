import { useManagersStore } from "../../store/managers";
import { SkeletonRows } from "../primitives/SkeletonRows";
import { ManagerCard } from "./ManagerCard";

export function DashboardView() {
  const detection = useManagersStore((s) => s.detection);
  const detecting = useManagersStore((s) => s.detecting);

  const managers = detection?.managers ?? [];

  return (
    <div className="flex h-full flex-col">
      {/* Refresh All / Update Everything live in the Sidebar only. They were
          duplicated here, which meant two controls for one action and two
          places to keep in sync. */}
      <header className="flex items-center gap-3 border-b border-border px-6 py-4">
        <h1 className="text-[20px] font-semibold text-text-primary">Packages</h1>
      </header>

      <div className="flex-1 overflow-auto p-6">
        {detecting && managers.length === 0 ? (
          <SkeletonRows rows={6} rowHeight={120} />
        ) : (
          <div className="grid gap-4" style={{ gridTemplateColumns: "repeat(auto-fill, minmax(320px, 1fr))" }}>
            {managers.map((m) => (
              <ManagerCard key={m.id} info={m} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
