import { useState } from "react";
import { buildUpgradePlan, refreshAll } from "../../lib/ipc/client";
import { useTotalOutdated } from "../../store";
import { absentManagers, presentManagers, useManagersStore } from "../../store/managers";
import { useUiStore } from "../../store/ui";
import { Button } from "../primitives/Button";
import { CopyableCommand } from "../primitives/CopyableCommand";
import { SkeletonRows } from "../primitives/SkeletonRows";
import { Tooltip } from "../primitives/Tooltip";
import { SidebarManagerItem } from "./SidebarManagerItem";

function NavButton({
  label,
  active,
  onClick,
}: {
  label: string;
  active: boolean;
  onClick: () => void;
}) {
  return (
    <button
      type="button"
      aria-current={active ? "page" : undefined}
      onClick={onClick}
      className={[
        "relative flex w-full items-center rounded-control px-2.5 py-1.5 text-left text-[13px]",
        "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent",
        active
          ? "bg-bg-raised text-text-primary before:absolute before:inset-y-1 before:left-0 before:w-0.5 before:rounded-full before:bg-accent"
          : "text-text-secondary hover:bg-bg-raised hover:text-text-primary",
      ].join(" ")}
    >
      {label}
    </button>
  );
}

export function Sidebar() {
  const detection = useManagersStore((s) => s.detection);
  const detecting = useManagersStore((s) => s.detecting);
  const view = useUiStore((s) => s.view);
  const navigate = useUiStore((s) => s.navigate);
  const settings = useUiStore((s) => s.settings);
  const openDialog = useUiStore((s) => s.openDialog);
  const total = useTotalOutdated();
  const [showAbsent, setShowAbsent] = useState(false);

  const present = presentManagers(detection);
  const absent = absentManagers(detection);
  const loading = detecting && present.length === 0;

  async function updateEverything() {
    const plan = await buildUpgradePlan({
      selection: null,
      includeSelfUpdates: true,
      includeGreedyCasks: settings?.includeGreedyByDefault ?? false,
    });
    openDialog({ kind: "upgradePlan", plan });
  }

  return (
    <nav className="flex h-full w-60 shrink-0 flex-col border-r border-border bg-bg-surface">
      <div className="px-3 pb-2 pt-[38px]" data-tauri-drag-region>
        <div className="flex items-center gap-2 px-1" data-tauri-drag-region>
          <span
            aria-hidden="true"
            className="flex h-6 w-6 items-center justify-center rounded bg-accent/20 text-[13px] text-accent"
          >
            ▣
          </span>
          <span className="text-[15px] font-semibold text-text-primary">Pack-Manager</span>
        </div>
      </div>

      <div className="flex flex-col gap-2 px-3 py-2">
        <Button variant="secondary" size="md" className="w-full" onClick={() => void refreshAll()}>
          Refresh All
        </Button>
        {total === 0 ? (
          <Tooltip content="Nothing to update" className="w-full">
            <Button variant="primary" size="md" className="w-full" disabled>
              Update Everything
            </Button>
          </Tooltip>
        ) : (
          <Button variant="primary" size="md" className="w-full" onClick={() => void updateEverything()}>
            Update Everything ({total})
          </Button>
        )}
      </div>

      <div className="flex-1 overflow-y-auto px-3 py-1">
        <NavButton
          label="Dashboard"
          active={view.kind === "dashboard"}
          onClick={() => navigate({ kind: "dashboard" })}
        />

        <div className="my-2 border-t border-border" />

        {loading ? (
          <SkeletonRows rows={5} rowHeight={28} className="px-1" />
        ) : (
          <div className="flex flex-col gap-0.5">
            {present.map((m) => (
              <SidebarManagerItem key={m.id} info={m} />
            ))}
          </div>
        )}

        {absent.length > 0 && (
          <div className="mt-2">
            <button
              type="button"
              onClick={() => setShowAbsent((v) => !v)}
              className="flex w-full items-center gap-1 px-2.5 py-1 text-[11px] uppercase tracking-wide text-text-muted hover:text-text-secondary"
            >
              <span>{showAbsent ? "▾" : "▸"}</span> Not installed ({absent.length})
            </button>
            {showAbsent && (
              <div className="flex flex-col gap-2 px-2 py-1 opacity-60">
                {absent.map((m) => (
                  <div key={m.id} className="text-[12px] text-text-secondary">
                    <div className="flex items-center gap-2">
                      <span>{m.displayName}</span>
                    </div>
                    {m.installHint && (
                      <div className="mt-1">
                        <CopyableCommand command={m.installHint} label="Install" />
                      </div>
                    )}
                  </div>
                ))}
              </div>
            )}
          </div>
        )}
      </div>

      <div className="flex flex-col gap-0.5 border-t border-border px-3 py-2">
        <NavButton
          label="History"
          active={view.kind === "history"}
          onClick={() => navigate({ kind: "history" })}
        />
        <NavButton
          label="Settings"
          active={view.kind === "settings"}
          onClick={() => navigate({ kind: "settings" })}
        />
      </div>
    </nav>
  );
}
