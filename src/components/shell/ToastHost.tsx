import { useEffect } from "react";
import { useUiStore, type Toast } from "../../store/ui";

const TONE: Record<Toast["kind"], string> = {
  success: "border-l-success",
  error: "border-l-danger",
  info: "border-l-info",
};

/** Auto-dismisses one non-persistent toast after 4s (SPEC §4.10). */
function ToastRow({ toast }: { toast: Toast }) {
  const dismiss = useUiStore((s) => s.dismissToast);
  const setDrawerOpen = useUiStore((s) => s.setDrawerOpen);
  const setFocusedOp = useUiStore((s) => s.setFocusedOp);

  useEffect(() => {
    if (toast.persistent) return;
    const t = window.setTimeout(() => dismiss(toast.id), 4000);
    return () => window.clearTimeout(t);
  }, [toast.id, toast.persistent, dismiss]);

  function viewLog() {
    if (!toast.opId) return;
    setFocusedOp(toast.opId);
    setDrawerOpen(true);
  }

  return (
    <div
      role="status"
      className={[
        "pointer-events-auto flex w-80 items-start gap-3 rounded-card border border-border border-l-4 bg-bg-overlay px-3 py-2 shadow-lg",
        TONE[toast.kind],
      ].join(" ")}
    >
      <div className="flex-1 text-[13px] text-text-primary">{toast.message}</div>
      <div className="flex shrink-0 items-center gap-2">
        {toast.opId && (
          <button
            type="button"
            onClick={viewLog}
            className="text-[12px] text-accent hover:underline focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent"
          >
            View log
          </button>
        )}
        <button
          type="button"
          aria-label="Dismiss"
          onClick={() => dismiss(toast.id)}
          className="text-[13px] text-text-muted hover:text-text-secondary"
        >
          ✕
        </button>
      </div>
    </div>
  );
}

export function ToastHost() {
  const toasts = useUiStore((s) => s.toasts);
  const visible = toasts.slice(-3);
  return (
    <div
      aria-live="polite"
      className="pointer-events-none fixed right-4 top-4 z-[100] flex flex-col gap-2"
    >
      {visible.map((t) => (
        <ToastRow key={t.id} toast={t} />
      ))}
    </div>
  );
}
