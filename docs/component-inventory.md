# Pack-Manager Component Inventory

- **Date:** 2026-07-22
- **Scope:** React interface, frontend state, local IPC seam, styling, and packaged assets

## Overview

Pack-Manager has 37 production TSX component modules organized by user-facing area. The interface is a store-driven single-window application: `AppLayout` selects views from Zustand state rather than a URL router, while globally mounted activity, dialog, toast, and status components remain available across views.

The frontend does not call package-manager processes directly. All native work crosses the typed Tauri IPC boundary in `src/lib/ipc/`, and backend events update the frontend stores.

## Application Shell

| Component | Location | Responsibility |
| --- | --- | --- |
| `App` | `src/App.tsx` | Subscribes to native events before hydrating state, restores update state, and schedules launch refresh. |
| `AppLayout` | `src/components/shell/AppLayout.tsx` | Root shell and store-driven view switcher; mounts global overlays and status UI. |
| `Sidebar` | `src/components/shell/Sidebar.tsx` | Primary navigation for dashboard, managers, history, and settings. |
| `SidebarManagerItem` | `src/components/shell/SidebarManagerItem.tsx` | Manager navigation item with derived status. |
| `StatusBar` | `src/components/shell/StatusBar.tsx` | Global operation and application status. |
| `UpdateStatusItem` | `src/components/shell/UpdateStatusItem.tsx` | Pack-Manager update availability/download/install affordance. |
| `ToastHost` | `src/components/shell/ToastHost.tsx` | Global transient notifications. |

## Feature Components

### Dashboard

| Component | Responsibility |
| --- | --- |
| `DashboardView` | Displays the manager-card overview. |
| `ManagerCard` | Isolates each manager's present, absent, stale, error, busy, and package states. |
| `ManagedByChip` | Explains dynamically detected manager ownership and self-update routing. |

Location: `src/components/dashboard/`

### Package Manager View

| Component | Responsibility |
| --- | --- |
| `ManagerPane` | Composes manager health, self-update, filtering, package table, and selection actions. |
| `SelfUpdateCard` | Shows and starts the manager's derived self-update route. |
| `HealthBanner` | Presents manager health issues and supported fixes. |
| `PackageToolbar` | Search, outdated-only filtering, refresh, and bulk actions. |
| `PackageTable` | Package table with virtualization above 100 rows. |
| `PackageRow` | Selection, installed/latest versions, status, and row upgrade action. |
| `VersionDelta` | Display-only segmented version highlighting; never determines outdatedness. |
| `StatusBadge` | Maps package and active-operation state to visual status. |
| `SelectionToolbar` | Bulk-upgrade and clear-selection actions. |

Location: `src/components/manager/`

Selection is filter-aware and supports shift ranges. Pinned packages and greedy casks that have not been opted into are deliberately non-selectable.

### Activity and Operation History

| Component/helper | Responsibility |
| --- | --- |
| `ActivityDrawer` | Persistent operation drawer with adjustable height and focus management. |
| `OperationList` | Lists current and recent operations. |
| `OperationRow` | Displays operation identity, state, duration, and cancellation action. |
| `LiveLogView` | Streams output, virtualizes above 200 lines, follows the tail, and reports ring-buffer overflow. |
| `useOperationEffects` | Converts operation transitions into drawer and toast behavior. |
| `opDisplay.ts` | Pure operation titles, status metadata, duration, and carriage-return formatting. |
| `HistoryView` | Filters past operations and lazily loads transcript tails. |

Locations: `src/components/activity/` and `src/components/history/`

### Dialogs

| Component | Responsibility |
| --- | --- |
| `DialogHost` | Renders the active discriminated dialog from UI state. |
| `UpgradePlanSheet` | Trust checkpoint showing exact commands, exclusions, locks, notes, and staleness warnings before execution. |
| `StalledOperationDialog` | Offers cancellation or a copy-to-terminal handoff for silent work. |
| `QuitGuardDialog` | Prevents accidental exit or updater restart while operations are active. |

Location: `src/components/dialogs/`

### Settings

`SettingsView` in `src/components/settings/SettingsView.tsx` manages persisted preferences, log level, automatic refresh/update options, updater controls, the detected environment report, diagnostics export, and maintenance actions.

## Reusable Primitives

`src/components/primitives/` provides the shared visual vocabulary:

- `Button` — primary, secondary, ghost, and danger variants in small and medium sizes.
- `Checkbox` — package and bulk-selection control.
- `Chip` — semantic status/metadata badge.
- `Tooltip` — contextual explanation.
- `Spinner` and `SkeletonRows` — active and structural loading states.
- `EmptyState` and `ErrorState` — reusable no-data and recoverable-failure presentations.
- `CopyableCommand` — command presentation with clipboard action.

Use these primitives before introducing feature-local equivalents so new UI remains aligned with the existing design tokens and accessibility behavior.

## Frontend State

Five independent Zustand stores live under `src/store/`:

| Store | Owns | Persistence |
| --- | --- | --- |
| `managers.ts` | Detection report, detection activity, and manager-specific errors. | Rehydrated from native state/events. |
| `packages.ts` | Snapshots, stale flags, selection anchors, search, and outdated-only filters. | Session only. |
| `operations.ts` | Normalized operation records and per-operation output capped at 5,000 lines. | Current session plus native journal rehydration. |
| `ui.ts` | Active view, drawer, dialog, toasts, settings working copy, and row highlighting. | Settings are saved natively; other UI state is session only. |
| `appUpdate.ts` | Native app-update status. | Rehydrated from the native updater state. |

`src/store/index.ts` provides cross-store derived selectors such as manager phase and total actionable outdated packages. Manager phase is derived from operation records and errors rather than duplicated as mutable state.

## IPC and Event Integration

- `src/lib/ipc/bridge.ts` is the only frontend import site for `@tauri-apps/api`, making it the test seam.
- `src/lib/ipc/client.ts` provides typed wrappers for all 20 registered Rust commands. UI components call these wrappers, never raw `invoke`.
- `src/lib/ipc/events.ts` subscribes once to six native event channels and updates stores.
- `src/lib/ipc/types.ts` mirrors Rust's camelCase wire models, supplies runtime guards, and is checked against shared JSON contract fixtures.

The event channels are `detection:updated`, `snapshot:updated`, `op:status`, `op:output`, `op:stalled`, and `appUpdate:status`.

## Styling, Assets, and Localization

- `src/styles/theme.css` imports Tailwind CSS 4 and defines the dark-only design tokens for surfaces, text, status/severity colors, radii, system fonts, and reduced-motion behavior.
- The web interface has no image, web-font, or `public/` asset dependency.
- `src-tauri/icons/` contains 17 generated packaging icons (15 PNG, one ICNS, one ICO; approximately 304 KiB).
- `dev/icon/` contains the 1024-pixel source image and reproducible Python generator (approximately 54 KiB).
- No localization framework or locale resources exist. `index.html` declares English, and interface text is currently hardcoded in English.

## Testing Support

The frontend suite contains 22 test files and 120 passing tests at scan time. Important test seams are:

- `src/test/fakeIpc.ts` for command calls, listeners, event emission, and listener-leak checks.
- `src/test/fixtures.ts` for realistic typed manager/application state.
- Shared `dev/fixtures/ipc/*.json` payloads for Rust–TypeScript contract drift detection.
- Vitest, jsdom, React Testing Library, and fake timers for component, store, keyboard, dialog, updater, history, and operation behavior.

There is no Playwright/Cypress end-to-end suite. Machine-dependent smoke tests are implemented as ignored Rust integration tests.

## Extension Guidance

- Add new screen-level UI beneath the closest feature folder and route it through the `ActiveView` union in `src/store/ui.ts`.
- Add broadly reusable controls to `src/components/primitives/` and export them from its barrel file.
- Keep native calls in `src/lib/ipc/client.ts`; keep `@tauri-apps/api` imports confined to `bridge.ts`.
- Update Rust wire types, TypeScript mirrors/guards, and shared IPC fixtures together.
- Prefer derived selectors over duplicating backend or cross-store state.
- Preserve the rule that the package manager's own outdated verdict is authoritative; `VersionDelta` is display-only.
