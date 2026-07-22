import { buildUpgradePlan, refreshAll } from "../../lib/ipc/client";
import { useTotalOutdated } from "../../store";
import { useManagersStore } from "../../store/managers";
import { useUiStore } from "../../store/ui";
import { Button } from "../primitives/Button";
import { SkeletonRows } from "../primitives/SkeletonRows";
import { Tooltip } from "../primitives/Tooltip";
import { ManagerCard } from "./ManagerCard";

export function DashboardView() {
  const detection = useManagersStore((s) => s.detection);
  const detecting = useManagersStore((s) => s.detecting);
  const settings = useUiStore((s) => s.settings);
  const openDialog = useUiStore((s) => s.openDialog);
  const total = useTotalOutdated();

  async function updateEverything() {
    const plan = await buildUpgradePlan({
      selection: null,
      includeSelfUpdates: true,
      includeGreedyCasks: settings?.includeGreedyByDefault ?? false,
    });
    openDialog({ kind: "upgradePlan", plan });
  }

  const managers = detection?.managers ?? [];

  return (
    <div className="flex h-full flex-col">
      <header className="flex items-center gap-3 border-b border-border px-6 py-4">
        <h1 className="text-[20px] font-semibold text-text-primary">Packages</h1>
        <div className="ml-auto flex items-center gap-2">
          <Button variant="secondary" size="md" onClick={() => void refreshAll()}>
            Refresh All
          </Button>
          {total === 0 ? (
            <Tooltip content="Nothing to update">
              <Button variant="primary" size="md" disabled>
                Update Everything
              </Button>
            </Tooltip>
          ) : (
            <Button variant="primary" size="md" onClick={() => void updateEverything()}>
              Update Everything ({total})
            </Button>
          )}
        </div>
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
