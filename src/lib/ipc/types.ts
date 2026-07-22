/**
 * 1:1 TypeScript mirror of `src-tauri/src/ipc.rs` (+ event payloads from
 * `events.rs` and `IpcError` from `error.rs`) — SPEC §5.9/§5.10.
 *
 * Drift between the two sides is guarded by the contract fixtures in
 * `dev/fixtures/ipc/*.json`: Rust asserts serialization byte-equality
 * (`ipc_contract_matches_committed_fixtures`); the Vitest test
 * (`ipc_types_accept_contract_fixtures`, in `types.test.ts`) runs the type
 * guards below over the same files. If you edit a type here, the Rust side
 * (and the fixtures) must move in the same change, and vice versa.
 *
 * Convention: Rust `Option<T>` + `skip_serializing_if` ⇒ optional (`?`);
 * plain `Option<T>` ⇒ `T | null`.
 */

// ---------------------------------------------------------------------------
// Core string enums
// ---------------------------------------------------------------------------

export const MANAGER_IDS = ["brew", "mise", "npm", "uv", "rustup", "mas"] as const;
export type ManagerId = (typeof MANAGER_IDS)[number];

export const MANAGED_BY = ["brew", "mise", "rustup", "standalone"] as const;
export type ManagedBy = (typeof MANAGED_BY)[number];

export const PACKAGE_KINDS = [
  "formula",
  "cask",
  "caskGreedy",
  "tool",
  "globalPackage",
  "toolchain",
  "app",
] as const;
export type PackageKind = (typeof PACKAGE_KINDS)[number];

export const OP_KINDS = ["refresh", "upgrade", "selfUpdate", "healthFix"] as const;
export type OpKind = (typeof OP_KINDS)[number];

export const OP_STATUSES = [
  "queued",
  "running",
  "succeeded",
  "failed",
  "cancelled",
  "timedOut",
  "interrupted",
] as const;
export type OpStatus = (typeof OP_STATUSES)[number];

export const PATH_SOURCES = ["loginShell", "staticFallback", "merged"] as const;
export type PathSource = (typeof PATH_SOURCES)[number];

export const MANAGER_STATUSES = ["present", "absent"] as const;
export type ManagerStatus = (typeof MANAGER_STATUSES)[number];

export const HEALTH_SEVERITIES = ["warning", "error"] as const;
export type HealthSeverity = (typeof HEALTH_SEVERITIES)[number];

export const STREAM_KINDS = ["out", "err"] as const;
export type StreamKind = (typeof STREAM_KINDS)[number];

export const EXCLUDE_REASONS = ["pinned", "greedyCask", "rustDedup", "alreadyRunning"] as const;
export type ExcludeReason = (typeof EXCLUDE_REASONS)[number];

export const ERROR_CODES = [
  "tool_not_found",
  "spawn_failed",
  "timeout",
  "non_zero_exit",
  "brew_lock_busy",
  "parse_failed",
  "cancelled",
  "self_update_unavailable",
  "env_capture_failed",
  "io",
  "internal",
] as const;
export type ErrorCode = (typeof ERROR_CODES)[number];

export const LOG_LEVELS = ["error", "warn", "info", "debug", "trace"] as const;
export type LogLevel = (typeof LOG_LEVELS)[number];

// ---------------------------------------------------------------------------
// Interfaces
// ---------------------------------------------------------------------------

export interface EnvInfo {
  path: string;
  entries: string[];
  source: PathSource;
  home: string;
}

export interface DetectionReport {
  managers: ManagerInfo[];
  env: EnvInfo;
}

export interface ManagerInfo {
  id: ManagerId;
  displayName: string;
  status: ManagerStatus;
  binaryPath?: string;
  canonicalPath?: string;
  version?: string;
  managedBy: ManagedBy;
  evidence?: string;
  selfUpdate: SelfUpdateRoute;
  /** Absent managers only, e.g. "brew install mas". */
  installHint?: string;
}

export type SelfUpdateRoute =
  | { kind: "inBand"; commandPreview: string; note?: string }
  | { kind: "routed"; executor: ManagerId; commandPreview: string; why: string }
  | { kind: "viaRefresh"; note: string }
  | { kind: "unavailable"; reason: string };

export interface PackageMeta {
  executables?: string[];
  requested?: string;
  source?: string;
  wanted?: string;
  dependedBy?: string;
  pinnedVersion?: string;
}

export interface Package {
  /** `${kind}:${name}`, name verbatim; split on FIRST ':' only ("tool:npm:prettier"). */
  id: string;
  name: string;
  kind: PackageKind;
  /** Verbatim; null = unknown (no fabricated deltas). */
  installed: string | null;
  latest: string | null;
  /** The manager's verdict — authoritative. */
  outdated: boolean;
  pinned: boolean;
  meta?: PackageMeta;
}

export interface SelfStatus {
  installed: string | null;
  latest: string | null;
  updateAvailable: boolean;
}

export interface ManagerSnapshot {
  managerId: ManagerId;
  /** RFC3339. */
  refreshedAt: string;
  /** Excludes the manager's own self row. */
  packages: Package[];
  selfStatus?: SelfStatus;
  health: HealthIssue[];
}

export interface HealthIssue {
  id: string;
  managerId: ManagerId;
  severity: HealthSeverity;
  title: string;
  detail: string;
  fixCommand?: string;
  fixable: boolean;
}

export interface PlanSelection {
  managerId: ManagerId;
  packageId: string;
}

export interface PlanRequest {
  /** null = all outdated, all managers. */
  selection: PlanSelection[] | null;
  includeSelfUpdates: boolean;
  includeGreedyCasks: boolean;
}

export interface PlanCommand {
  argvPreview: string;
  label: string;
}

export interface PlanGroup {
  subject: ManagerId;
  executor: ManagerId;
  locks: ManagerId[];
  commands: PlanCommand[];
  packageIds: string[];
  selfUpdate: boolean;
}

export interface ExcludedPackage {
  managerId: ManagerId;
  packageId: string;
  reason: ExcludeReason;
}

export interface UpgradePlan {
  planId: string;
  groups: PlanGroup[];
  excluded: ExcludedPackage[];
  notes: string[];
  warnings: string[];
}

export interface OpRef {
  opId: string;
}

export interface OpIds {
  opIds: string[];
}

export interface IpcError {
  code: ErrorCode;
  message: string;
  detail?: string;
  managerId?: ManagerId;
  opId?: string;
  /** Always populated for op-scoped errors — "View log" never dangles. */
  logPath?: string;
}

export interface OperationRecord {
  opId: string;
  kind: OpKind;
  executor: ManagerId;
  subject: ManagerId;
  status: OpStatus;
  commandLine: string;
  packageIds: string[];
  queuedAt: string;
  startedAt: string | null;
  finishedAt: string | null;
  exitCode: number | null;
  error: IpcError | null;
  logPath: string;
}

export interface LogLine {
  stream: StreamKind;
  line: string;
  tsMs: number;
}

/** Record + ring-buffer replay (cap 5000). */
export interface OperationDetail {
  record: OperationRecord;
  lines: LogLine[];
  truncated: boolean;
}

export interface AppState {
  detection: DetectionReport;
  snapshots: ManagerSnapshot[];
  operations: OperationRecord[];
  settings: Settings;
}

export interface Settings {
  runBrewUpdateOnRefresh: boolean;
  autoRefreshOnLaunch: boolean;
  stallAfterSecs: number;
  upgradeHardCapMins: number;
  logLevel: LogLevel;
  autoOpenDrawer: boolean;
  includeGreedyByDefault: boolean;
}

export interface DiagnosticsResult {
  zipPath: string;
}

// ---------------------------------------------------------------------------
// Event payloads (SPEC §5.9 events table; Rust side in events.rs)
// ---------------------------------------------------------------------------

export const EVENT_DETECTION_UPDATED = "detection:updated";
export const EVENT_SNAPSHOT_UPDATED = "snapshot:updated";
export const EVENT_OP_STATUS = "op:status";
export const EVENT_OP_OUTPUT = "op:output";
export const EVENT_OP_STALLED = "op:stalled";

/** Payload of `snapshot:updated` (health rides in the snapshot). */
export interface SnapshotUpdatedEvent {
  managerId: ManagerId;
  snapshot: ManagerSnapshot;
}

/** Payload of `op:status` — emitted on enqueue (queued), start, phase change, finish. */
export interface OpStatusEvent {
  opId: string;
  kind: OpKind;
  executor: ManagerId;
  subject: ManagerId;
  status: OpStatus;
  queuePosition?: number;
  phaseLabel?: string;
  commandLine: string;
  exitCode?: number;
  error?: IpcError;
  startedAt?: string;
  finishedAt?: string;
  logPath: string;
}

/** Payload of `op:output` — flushed every 50ms or 64 lines or 8KiB, whichever first. */
export interface OpOutputEvent {
  opId: string;
  batch: LogLine[];
}

/** Payload of `op:stalled`. */
export interface OpStalledEvent {
  opId: string;
  silentForSecs: number;
}

// ---------------------------------------------------------------------------
// Type guards — runtime checks used by the contract test and event dispatch.
// Guards check every required field, enum membership, and the types of
// optional fields when present. Unknown extra fields are tolerated.
// ---------------------------------------------------------------------------

type Rec = Record<string, unknown>;

function isRec(v: unknown): v is Rec {
  return typeof v === "object" && v !== null && !Array.isArray(v);
}

function isStr(v: unknown): v is string {
  return typeof v === "string";
}

function isNum(v: unknown): v is number {
  return typeof v === "number" && Number.isFinite(v);
}

function isBool(v: unknown): v is boolean {
  return typeof v === "boolean";
}

function isStrArray(v: unknown): v is string[] {
  return Array.isArray(v) && v.every(isStr);
}

function oneOf<T extends string>(values: readonly T[]): (v: unknown) => v is T {
  return (v: unknown): v is T => isStr(v) && (values as readonly string[]).includes(v);
}

/** Optional field: absent or passing `check`. */
function opt(v: unknown, check: (x: unknown) => boolean): boolean {
  return v === undefined || check(v);
}

/** Nullable field: null or passing `check`. */
function nullable(v: unknown, check: (x: unknown) => boolean): boolean {
  return v === null || check(v);
}

export const isManagerId = oneOf(MANAGER_IDS);
export const isManagedBy = oneOf(MANAGED_BY);
export const isPackageKind = oneOf(PACKAGE_KINDS);
export const isOpKind = oneOf(OP_KINDS);
export const isOpStatus = oneOf(OP_STATUSES);
export const isPathSource = oneOf(PATH_SOURCES);
export const isManagerStatus = oneOf(MANAGER_STATUSES);
export const isHealthSeverity = oneOf(HEALTH_SEVERITIES);
export const isStreamKind = oneOf(STREAM_KINDS);
export const isExcludeReason = oneOf(EXCLUDE_REASONS);
export const isErrorCode = oneOf(ERROR_CODES);
export const isLogLevel = oneOf(LOG_LEVELS);

export function isEnvInfo(v: unknown): v is EnvInfo {
  return (
    isRec(v) && isStr(v.path) && isStrArray(v.entries) && isPathSource(v.source) && isStr(v.home)
  );
}

export function isSelfUpdateRoute(v: unknown): v is SelfUpdateRoute {
  if (!isRec(v)) return false;
  switch (v.kind) {
    case "inBand":
      return isStr(v.commandPreview) && opt(v.note, isStr);
    case "routed":
      return isManagerId(v.executor) && isStr(v.commandPreview) && isStr(v.why);
    case "viaRefresh":
      return isStr(v.note);
    case "unavailable":
      return isStr(v.reason);
    default:
      return false;
  }
}

export function isManagerInfo(v: unknown): v is ManagerInfo {
  return (
    isRec(v) &&
    isManagerId(v.id) &&
    isStr(v.displayName) &&
    isManagerStatus(v.status) &&
    opt(v.binaryPath, isStr) &&
    opt(v.canonicalPath, isStr) &&
    opt(v.version, isStr) &&
    isManagedBy(v.managedBy) &&
    opt(v.evidence, isStr) &&
    isSelfUpdateRoute(v.selfUpdate) &&
    opt(v.installHint, isStr)
  );
}

export function isDetectionReport(v: unknown): v is DetectionReport {
  return (
    isRec(v) && Array.isArray(v.managers) && v.managers.every(isManagerInfo) && isEnvInfo(v.env)
  );
}

export function isPackageMeta(v: unknown): v is PackageMeta {
  return (
    isRec(v) &&
    opt(v.executables, isStrArray) &&
    opt(v.requested, isStr) &&
    opt(v.source, isStr) &&
    opt(v.wanted, isStr) &&
    opt(v.dependedBy, isStr) &&
    opt(v.pinnedVersion, isStr)
  );
}

export function isPackage(v: unknown): v is Package {
  return (
    isRec(v) &&
    isStr(v.id) &&
    isStr(v.name) &&
    isPackageKind(v.kind) &&
    nullable(v.installed, isStr) &&
    nullable(v.latest, isStr) &&
    isBool(v.outdated) &&
    isBool(v.pinned) &&
    opt(v.meta, isPackageMeta)
  );
}

export function isSelfStatus(v: unknown): v is SelfStatus {
  return (
    isRec(v) &&
    nullable(v.installed, isStr) &&
    nullable(v.latest, isStr) &&
    isBool(v.updateAvailable)
  );
}

export function isHealthIssue(v: unknown): v is HealthIssue {
  return (
    isRec(v) &&
    isStr(v.id) &&
    isManagerId(v.managerId) &&
    isHealthSeverity(v.severity) &&
    isStr(v.title) &&
    isStr(v.detail) &&
    opt(v.fixCommand, isStr) &&
    isBool(v.fixable)
  );
}

export function isManagerSnapshot(v: unknown): v is ManagerSnapshot {
  return (
    isRec(v) &&
    isManagerId(v.managerId) &&
    isStr(v.refreshedAt) &&
    Array.isArray(v.packages) &&
    v.packages.every(isPackage) &&
    opt(v.selfStatus, isSelfStatus) &&
    Array.isArray(v.health) &&
    v.health.every(isHealthIssue)
  );
}

export function isPlanSelection(v: unknown): v is PlanSelection {
  return isRec(v) && isManagerId(v.managerId) && isStr(v.packageId);
}

export function isPlanRequest(v: unknown): v is PlanRequest {
  return (
    isRec(v) &&
    nullable(v.selection, (s) => Array.isArray(s) && s.every(isPlanSelection)) &&
    isBool(v.includeSelfUpdates) &&
    isBool(v.includeGreedyCasks)
  );
}

export function isPlanCommand(v: unknown): v is PlanCommand {
  return isRec(v) && isStr(v.argvPreview) && isStr(v.label);
}

export function isPlanGroup(v: unknown): v is PlanGroup {
  return (
    isRec(v) &&
    isManagerId(v.subject) &&
    isManagerId(v.executor) &&
    Array.isArray(v.locks) &&
    v.locks.every(isManagerId) &&
    Array.isArray(v.commands) &&
    v.commands.every(isPlanCommand) &&
    isStrArray(v.packageIds) &&
    isBool(v.selfUpdate)
  );
}

export function isExcludedPackage(v: unknown): v is ExcludedPackage {
  return isRec(v) && isManagerId(v.managerId) && isStr(v.packageId) && isExcludeReason(v.reason);
}

export function isUpgradePlan(v: unknown): v is UpgradePlan {
  return (
    isRec(v) &&
    isStr(v.planId) &&
    Array.isArray(v.groups) &&
    v.groups.every(isPlanGroup) &&
    Array.isArray(v.excluded) &&
    v.excluded.every(isExcludedPackage) &&
    isStrArray(v.notes) &&
    isStrArray(v.warnings)
  );
}

export function isOpRef(v: unknown): v is OpRef {
  return isRec(v) && isStr(v.opId);
}

export function isOpIds(v: unknown): v is OpIds {
  return isRec(v) && isStrArray(v.opIds);
}

export function isIpcError(v: unknown): v is IpcError {
  return (
    isRec(v) &&
    isErrorCode(v.code) &&
    isStr(v.message) &&
    opt(v.detail, isStr) &&
    opt(v.managerId, isManagerId) &&
    opt(v.opId, isStr) &&
    opt(v.logPath, isStr)
  );
}

export function isOperationRecord(v: unknown): v is OperationRecord {
  return (
    isRec(v) &&
    isStr(v.opId) &&
    isOpKind(v.kind) &&
    isManagerId(v.executor) &&
    isManagerId(v.subject) &&
    isOpStatus(v.status) &&
    isStr(v.commandLine) &&
    isStrArray(v.packageIds) &&
    isStr(v.queuedAt) &&
    nullable(v.startedAt, isStr) &&
    nullable(v.finishedAt, isStr) &&
    nullable(v.exitCode, isNum) &&
    nullable(v.error, isIpcError) &&
    isStr(v.logPath)
  );
}

export function isLogLine(v: unknown): v is LogLine {
  return isRec(v) && isStreamKind(v.stream) && isStr(v.line) && isNum(v.tsMs);
}

export function isOperationDetail(v: unknown): v is OperationDetail {
  return (
    isRec(v) &&
    isOperationRecord(v.record) &&
    Array.isArray(v.lines) &&
    v.lines.every(isLogLine) &&
    isBool(v.truncated)
  );
}

export function isSettings(v: unknown): v is Settings {
  return (
    isRec(v) &&
    isBool(v.runBrewUpdateOnRefresh) &&
    isBool(v.autoRefreshOnLaunch) &&
    isNum(v.stallAfterSecs) &&
    isNum(v.upgradeHardCapMins) &&
    isLogLevel(v.logLevel) &&
    isBool(v.autoOpenDrawer) &&
    isBool(v.includeGreedyByDefault)
  );
}

export function isAppState(v: unknown): v is AppState {
  return (
    isRec(v) &&
    isDetectionReport(v.detection) &&
    Array.isArray(v.snapshots) &&
    v.snapshots.every(isManagerSnapshot) &&
    Array.isArray(v.operations) &&
    v.operations.every(isOperationRecord) &&
    isSettings(v.settings)
  );
}

export function isDiagnosticsResult(v: unknown): v is DiagnosticsResult {
  return isRec(v) && isStr(v.zipPath);
}

export function isSnapshotUpdatedEvent(v: unknown): v is SnapshotUpdatedEvent {
  return isRec(v) && isManagerId(v.managerId) && isManagerSnapshot(v.snapshot);
}

export function isOpStatusEvent(v: unknown): v is OpStatusEvent {
  return (
    isRec(v) &&
    isStr(v.opId) &&
    isOpKind(v.kind) &&
    isManagerId(v.executor) &&
    isManagerId(v.subject) &&
    isOpStatus(v.status) &&
    opt(v.queuePosition, isNum) &&
    opt(v.phaseLabel, isStr) &&
    isStr(v.commandLine) &&
    opt(v.exitCode, isNum) &&
    opt(v.error, isIpcError) &&
    opt(v.startedAt, isStr) &&
    opt(v.finishedAt, isStr) &&
    isStr(v.logPath)
  );
}

export function isOpOutputEvent(v: unknown): v is OpOutputEvent {
  return isRec(v) && isStr(v.opId) && Array.isArray(v.batch) && v.batch.every(isLogLine);
}

export function isOpStalledEvent(v: unknown): v is OpStalledEvent {
  return isRec(v) && isStr(v.opId) && isNum(v.silentForSecs);
}
