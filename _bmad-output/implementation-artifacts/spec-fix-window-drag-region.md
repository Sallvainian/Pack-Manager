---
title: 'Restore window dragging from the title bar strip'
type: 'bugfix'
created: '2026-07-25'
status: 'done'
route: 'one-shot'
---

# Restore window dragging from the title bar strip

## Intent

**Problem:** The window could be resized but never dragged. `core:window:default` does not include `allow-start-dragging`, so the ACL rejected every `start_dragging` call the drag region made — meaning the sidebar's long-standing `data-tauri-drag-region` had never worked. On top of that, only the sidebar reserved the overlay title bar, so the main column offered no drag surface at all, and the sidebar's bare attribute drags only on direct hits, so its logo and wordmark swallowed the mousedown.

**Approach:** Grant `core:window:allow-start-dragging` in the single capability file, give the main column the same 38px title-bar reserve the sidebar already had and mark it as a drag region, and promote the sidebar's region to `deep` so its child spans drag too.

## Suggested Review Order

1. [`src-tauri/capabilities/default.json`](../../src-tauri/capabilities/default.json) — the root cause. Confirm `core:window:allow-start-dragging` is the only permission added, and that granting it under `security.csp: null` is a risk you accept.
2. [`src/components/shell/AppLayout.tsx`](../../src/components/shell/AppLayout.tsx) — the 38px strip above `<main>`. This shifts all main-column content down 38px; verify that reads right and that the strip clears no interactive control.
3. [`src/components/shell/Sidebar.tsx`](../../src/components/shell/Sidebar.tsx) — `data-tauri-drag-region="deep"`. Verify the redundant attribute on the inner row was correctly dropped, not lost.
4. [`deferred-work.md`](./deferred-work.md) — six deferred items, including the now-stale project-context capability rule and the missing DECISIONS entry.
