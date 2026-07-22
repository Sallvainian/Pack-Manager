/**
 * HealthBanner — per-manager health issues (SPEC §4.8 row 3, §F13). Warning- or
 * error-tinted; every issue keeps its diagnostic detail visible, while only a
 * backend-recognized fix exposes copy and Run fix controls.
 */
import { runHealthFix } from "../../lib/ipc/client";
import type { HealthIssue, ManagerId } from "../../lib/ipc/types";
import { Button } from "../primitives/Button";
import { CopyableCommand } from "../primitives/CopyableCommand";

export interface HealthBannerProps {
  managerId: ManagerId;
  issues: HealthIssue[];
}

export function HealthBanner({ managerId, issues }: HealthBannerProps) {
  if (issues.length === 0) return null;

  return (
    <div className="flex flex-col gap-2">
      {issues.map((issue) => {
        const tint =
          issue.severity === "error"
            ? "border-danger/30 bg-danger/12"
            : "border-warning/30 bg-warning/12";
        return (
          <div
            key={issue.id}
            role="status"
            className={["rounded-card border px-3 py-2.5", tint].join(" ")}
          >
            <div className="text-[13px] font-medium text-text-primary">{issue.title}</div>
            {issue.detail && issue.detail !== issue.title && (
              <div className="mt-0.5 text-[12px] text-text-secondary">{issue.detail}</div>
            )}
            {(issue.fixCommand || issue.fixable) && (
              <div className="mt-2 flex flex-wrap items-center gap-2">
                {issue.fixCommand && <CopyableCommand command={issue.fixCommand} label="Fix" />}
                {issue.fixable && (
                  <Button
                    variant="secondary"
                    size="sm"
                    onClick={() => void runHealthFix(managerId, issue.id)}
                  >
                    Run fix
                  </Button>
                )}
              </div>
            )}
          </div>
        );
      })}
    </div>
  );
}
