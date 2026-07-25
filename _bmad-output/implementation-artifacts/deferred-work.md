- source_spec: `/Users/sallvain/Projects/Pack-Manager/_bmad-output/implementation-artifacts/spec-harden-command-trust-boundaries.md`
  summary: Decide whether command previews need shell-style escaping so distinct structured argv can never render identically.
  evidence: `command_preview` predates this story and joins argv with spaces; changing its visible output requires the separate product decision reserved by the spec.

- source_spec: `/Users/sallvain/Projects/Pack-Manager/_bmad-output/implementation-artifacts/spec-harden-command-trust-boundaries.md`
  summary: Order concurrent redetection requests so an older slow probe cannot publish after a newer probe.
  evidence: Concurrent redetections were already last-completion-wins before this story; the new revision barrier prevents unsafe plan execution but does not define frontend publication order.

- source_spec: `/Users/sallvain/Projects/Pack-Manager/_bmad-output/implementation-artifacts/spec-harden-command-trust-boundaries.md`
  summary: Give direct self-update and health-fix commands the same coherent state-capture and revision-aware admission used by bulk plans.
  evidence: These pre-existing handlers read detection, registry, settings, and ToolEnv outside the coordinator before normal queue submission, so concurrent refresh or redetection can make their constructed inputs stale.

- source_spec: `/Users/sallvain/Projects/Pack-Manager/_bmad-output/implementation-artifacts/spec-harden-command-trust-boundaries.md`
  summary: Define whether plan confirmation must re-probe package-manager native state changed by processes outside Pack-Manager.
  evidence: Plan validation consistently protects the app's coherent cached epoch, but external package, pin, shim, or path changes do not advance that in-process revision; live re-probing would materially change confirmation latency and semantics.

- source_spec: `/Users/sallvain/Projects/Pack-Manager/_bmad-output/implementation-artifacts/spec-fix-window-drag-region.md`
  summary: Regenerate the project-context rule that still claims the capability file grants only `core:default` and `opener:default`.
  evidence: `_bmad-output/project-context.md:136` states that verbatim; `src-tauri/capabilities/default.json` now carries a third permission. The file is workflow output, so it must be regenerated rather than hand-edited.

- source_spec: `/Users/sallvain/Projects/Pack-Manager/_bmad-output/implementation-artifacts/spec-fix-window-drag-region.md`
  summary: Record a DECISIONS entry for the `core:window:allow-start-dragging` grant so it is not pruned as unexplained scope.
  evidence: `docs/DECISIONS.md` runs D1-D33 with no entry covering window capabilities; the grant's only rationale currently lives in source comments, and removing it silently kills window dragging with no test to catch it.

- source_spec: `/Users/sallvain/Projects/Pack-Manager/_bmad-output/implementation-artifacts/spec-fix-window-drag-region.md`
  summary: Decide whether `security.csp: null` should be tightened now that a window-manipulation command is reachable from the webview.
  evidence: `src-tauri/tauri.conf.json` sets `"csp": null`, and project-context calls any capability change security-sensitive; the accepted risk was never written down either way.

- source_spec: `/Users/sallvain/Projects/Pack-Manager/_bmad-output/implementation-artifacts/spec-fix-window-drag-region.md`
  summary: Update SPEC §4.3 and its shell diagram to describe the main column's title-bar strip alongside the sidebar's.
  evidence: `docs/SPEC.md:212` documents only the sidebar's 38px drag padding and diagrams MainView flush to the top of the right column, which no longer matches `AppLayout.tsx`.

- source_spec: `/Users/sallvain/Projects/Pack-Manager/_bmad-output/implementation-artifacts/spec-fix-window-drag-region.md`
  summary: Extract the 38px title-bar reserve into one shared token consumed by both the sidebar and the main column.
  evidence: `Sidebar.tsx` `pt-[38px]` and `AppLayout.tsx` `h-[38px]` must agree to keep the drag band level, but nothing links them and Tailwind v4 here has no titlebar spacing token.

- source_spec: `/Users/sallvain/Projects/Pack-Manager/_bmad-output/implementation-artifacts/spec-fix-window-drag-region.md`
  summary: Add a regression guard pinning the capability permission list and asserting the shell renders its drag regions.
  evidence: `src-tauri/gen/schemas` is gitignored, so a dropped or misspelled permission identifier is only caught when a machine regenerates schemas at build time; no test references `data-tauri-drag-region`.

- source_spec: `/Users/sallvain/Projects/Pack-Manager/_bmad-output/implementation-artifacts/spec-adopt-design-tokens-and-focus-ring.md`
  summary: Add `ring-offset-*` to the 16 `focus-visible` ring sites that draw a focus ring flush against the element with no separating offset.
  evidence: `docs/SPEC.md` §4.1 requires focus be "offset against surface, on every interactive element" and `DESIGN.md:202` specifies a "separated 2px `focusRing` outline", but only 6 of the 22 focus sites carry `ring-offset-*` (`Button.tsx:38`, `Checkbox.tsx:38`, `Chip.tsx:40`, `CopyableCommand.tsx:36`, `PackageRow.tsx:99`, `PackageTable.tsx:110`). The other 16 span LiveLogView (3), SettingsView (2), PackageRow (2), and one each in OperationRow, ManagerCard, ManagerPane, PackageTable, PackageToolbar, Sidebar, SidebarManagerItem, ToastHost, UpdateStatusItem. This predates D35, which changed only the ring colour; each site also needs the correct `ring-offset-bg-*` for the surface it sits on, so it is a per-site visual decision rather than a mechanical sweep.

- source_spec: `/Users/sallvain/Projects/Pack-Manager/_bmad-output/implementation-artifacts/spec-adopt-design-tokens-and-focus-ring.md`
  summary: Three keyboard-focusable controls render no focus indicator at all — the "Outdated only" checkbox, the activity-drawer toggle, and the status-bar "Open logs folder" button.
  evidence: Measured in a real browser by walking the Tab order and reading computed style: each reports `:focus-visible` true with `boxShadow: "none"`. They carry no `focus-visible:ring-*` class in HEAD (`PackageToolbar.tsx:72`, `ActivityDrawer.tsx:83`, `StatusBar.tsx:53`), so this predates D35 and was not introduced by the token swap — the swap only retargeted sites that already had a ring. `EXPERIENCE.md:318` requires "Every interactive element uses a separate `{colors.focusRing}` indicator", so these three are an accessibility-floor violation that no test currently catches.
