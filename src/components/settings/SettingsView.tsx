import {
  detectManagers,
  exportDiagnostics,
  revealLogsDir,
  setSettings as setSettingsCmd,
} from "../../lib/ipc/client";
import type { LogLevel, Settings } from "../../lib/ipc/types";
import { LOG_LEVELS } from "../../lib/ipc/types";
import { useManagersStore } from "../../store/managers";
import { useUiStore } from "../../store/ui";
import { Button } from "../primitives/Button";
import { Checkbox } from "../primitives/Checkbox";
import { CopyableCommand } from "../primitives/CopyableCommand";

function ToggleRow({
  label,
  hint,
  checked,
  onChange,
}: {
  label: string;
  hint: string;
  checked: boolean;
  onChange: (v: boolean) => void;
}) {
  return (
    <label className="flex items-start gap-3 py-2">
      <Checkbox checked={checked} onChange={onChange} aria-label={label} />
      <span className="flex-1">
        <span className="block text-[13px] text-text-primary">{label}</span>
        <span className="block text-[12px] text-text-muted">{hint}</span>
      </span>
    </label>
  );
}

function NumberRow({
  label,
  hint,
  value,
  onCommit,
}: {
  label: string;
  hint: string;
  value: number;
  onCommit: (v: number) => void;
}) {
  return (
    <label className="flex items-center gap-3 py-2">
      <span className="flex-1">
        <span className="block text-[13px] text-text-primary">{label}</span>
        <span className="block text-[12px] text-text-muted">{hint}</span>
      </span>
      <input
        type="number"
        defaultValue={value}
        aria-label={label}
        onBlur={(e) => {
          const n = Number(e.target.value);
          if (Number.isFinite(n) && n !== value) onCommit(n);
        }}
        className="h-8 w-24 rounded-control border border-border-strong bg-bg-raised px-2 text-right font-mono text-[13px] text-text-primary focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent"
      />
    </label>
  );
}

export function SettingsView() {
  const settings = useUiStore((s) => s.settings);
  const applySettings = useUiStore((s) => s.setSettings);
  const detection = useManagersStore((s) => s.detection);
  const setDetecting = useManagersStore((s) => s.setDetecting);

  async function patch(p: Partial<Settings>) {
    const merged = await setSettingsCmd(p);
    applySettings(merged);
  }

  async function redetect() {
    setDetecting(true);
    try {
      await detectManagers();
    } finally {
      setDetecting(false);
    }
  }

  function environmentReportText(): string {
    if (!detection) return "";
    const lines: string[] = [];
    lines.push(`PATH source: ${detection.env.source}`);
    lines.push(`PATH: ${detection.env.path}`);
    lines.push(`HOME: ${detection.env.home}`);
    lines.push("");
    for (const m of detection.managers) {
      lines.push(
        `${m.displayName} [${m.status}] ${m.version ?? ""} ${m.binaryPath ?? ""} managedBy=${m.managedBy}${m.evidence ? ` — ${m.evidence}` : ""}`,
      );
    }
    return lines.join("\n");
  }

  return (
    <div className="flex h-full flex-col">
      <header className="flex items-center gap-3 border-b border-border px-6 py-4">
        <h1 className="text-[20px] font-semibold text-text-primary">Settings</h1>
      </header>

      <div className="flex-1 overflow-auto p-6">
        <div className="mx-auto flex max-w-2xl flex-col gap-8">
          {/* --- Preferences ---------------------------------------------- */}
          <section aria-label="Preferences">
            <h2 className="mb-2 text-[15px] font-semibold text-text-primary">Preferences</h2>
            {settings ? (
              <div className="divide-y divide-border rounded-card border border-border bg-bg-surface px-4">
                <ToggleRow
                  label="Run brew update on refresh"
                  hint="Refreshes Homebrew metadata (and self-updates brew) before listing."
                  checked={settings.runBrewUpdateOnRefresh}
                  onChange={(v) => void patch({ runBrewUpdateOnRefresh: v })}
                />
                <ToggleRow
                  label="Auto-refresh on launch"
                  hint="Refresh every manager when the app starts."
                  checked={settings.autoRefreshOnLaunch}
                  onChange={(v) => void patch({ autoRefreshOnLaunch: v })}
                />
                <ToggleRow
                  label="Auto-open activity drawer"
                  hint="Open the drawer when an upgrade or self-update starts."
                  checked={settings.autoOpenDrawer}
                  onChange={(v) => void patch({ autoOpenDrawer: v })}
                />
                <ToggleRow
                  label="Include self-updating casks by default"
                  hint="Greedy casks are excluded from Upgrade All unless enabled."
                  checked={settings.includeGreedyByDefault}
                  onChange={(v) => void patch({ includeGreedyByDefault: v })}
                />
                <NumberRow
                  label="Stall threshold (seconds)"
                  hint="Warn when a command produces no output for this long."
                  value={settings.stallAfterSecs}
                  onCommit={(v) => void patch({ stallAfterSecs: v })}
                />
                <NumberRow
                  label="Upgrade hard cap (minutes)"
                  hint="Force-stop an upgrade that runs past this limit."
                  value={settings.upgradeHardCapMins}
                  onCommit={(v) => void patch({ upgradeHardCapMins: v })}
                />
                <label className="flex items-center gap-3 py-2">
                  <span className="flex-1">
                    <span className="block text-[13px] text-text-primary">Log level</span>
                    <span className="block text-[12px] text-text-muted">
                      Verbosity of Pack-Manager's own logs.
                    </span>
                  </span>
                  <select
                    aria-label="Log level"
                    value={settings.logLevel}
                    onChange={(e) => void patch({ logLevel: e.target.value as LogLevel })}
                    className="h-8 rounded-control border border-border-strong bg-bg-raised px-2 text-[13px] text-text-primary focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent"
                  >
                    {LOG_LEVELS.map((lvl) => (
                      <option key={lvl} value={lvl}>
                        {lvl}
                      </option>
                    ))}
                  </select>
                </label>
              </div>
            ) : (
              <div className="text-[13px] text-text-muted">Loading settings…</div>
            )}
          </section>

          {/* --- Environment Report --------------------------------------- */}
          <section aria-label="Environment Report">
            <div className="mb-2 flex items-center gap-2">
              <h2 className="text-[15px] font-semibold text-text-primary">Environment Report</h2>
              <button
                type="button"
                onClick={() => void navigator.clipboard?.writeText(environmentReportText())}
                className="text-[12px] text-accent hover:underline"
              >
                Copy
              </button>
            </div>
            {detection ? (
              <div className="rounded-card border border-border bg-bg-surface p-4">
                <div className="font-mono text-[12px] text-text-secondary">
                  <div>
                    <span className="text-text-muted">source</span> {detection.env.source}
                  </div>
                  <div className="mt-1 break-all">
                    <span className="text-text-muted">PATH</span> {detection.env.path}
                  </div>
                </div>
                <div className="mt-3 flex flex-col gap-2">
                  {detection.managers.map((m) => (
                    <div key={m.id} className="border-t border-border pt-2 text-[12px]">
                      <div className="flex items-center gap-2">
                        <span className="text-[13px] font-medium text-text-primary">
                          {m.displayName}
                        </span>
                        <span className="text-text-muted">{m.status}</span>
                        {m.version && (
                          <span className="font-mono text-text-secondary">{m.version}</span>
                        )}
                        <span className="ml-auto text-text-muted">managedBy: {m.managedBy}</span>
                      </div>
                      {m.binaryPath && (
                        <div className="font-mono text-[11px] text-text-muted">{m.binaryPath}</div>
                      )}
                      {m.evidence && (
                        <div className="text-[11px] text-text-muted">{m.evidence}</div>
                      )}
                    </div>
                  ))}
                </div>
              </div>
            ) : (
              <div className="text-[13px] text-text-muted">Detection has not run yet.</div>
            )}
          </section>

          {/* --- Actions --------------------------------------------------- */}
          <section aria-label="Actions">
            <h2 className="mb-2 text-[15px] font-semibold text-text-primary">Maintenance</h2>
            <div className="flex flex-wrap items-center gap-2">
              <Button variant="secondary" onClick={() => void revealLogsDir()}>
                Open Logs Folder
              </Button>
              <Button variant="secondary" onClick={() => void exportDiagnostics()}>
                Export diagnostics
              </Button>
              <Button variant="secondary" onClick={() => void redetect()}>
                Re-detect
              </Button>
            </div>
            <p className="mt-3 text-[11px] text-text-muted">
              mas is unverified; notarization is out of scope for the MVP.
            </p>
            <div className="mt-3">
              <CopyableCommand command="brew install mas" label="Optional" />
            </div>
          </section>
        </div>
      </div>
    </div>
  );
}
