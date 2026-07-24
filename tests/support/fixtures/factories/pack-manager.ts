import { Faker, en } from "@faker-js/faker";

import {
  MANAGER_IDS,
  PACKAGE_KINDS,
  type AppState,
  type AppUpdateStatus,
  type DetectionReport,
  type ManagerId,
  type ManagerInfo,
  type ManagerSnapshot,
  type OperationRecord,
  type Package,
  type Settings,
  type UpgradePlan,
} from "../../../../src/lib/ipc/types";

export const DEFAULT_FACTORY_SEED = 22_072_026;

const FIXED_TIME = "2026-07-22T12:00:00.000Z";
const DISPLAY_NAMES: Record<ManagerId, string> = {
  brew: "Homebrew",
  mise: "mise",
  npm: "npm",
  uv: "uv",
  rustup: "rustup",
  mas: "Mac App Store",
};

export type FactoryEntityKind =
  | "appState"
  | "detectionReport"
  | "managerInfo"
  | "managerSnapshot"
  | "operationRecord"
  | "package"
  | "upgradePlan";

export interface FactoryEntity {
  kind: FactoryEntityKind;
  id: string;
}

export interface PackManagerFactories {
  readonly seed: number;
  createAppState(overrides?: Partial<AppState>): AppState;
  createAppUpdateStatus(overrides?: Partial<AppUpdateStatus>): AppUpdateStatus;
  createDetectionReport(overrides?: Partial<DetectionReport>): DetectionReport;
  createManagerInfo(overrides?: Partial<ManagerInfo>): ManagerInfo;
  createManagerSnapshot(overrides?: Partial<ManagerSnapshot>): ManagerSnapshot;
  createOperationRecord(overrides?: Partial<OperationRecord>): OperationRecord;
  createPackage(overrides?: Partial<Package>): Package;
  createSettings(overrides?: Partial<Settings>): Settings;
  createUpgradePlan(overrides?: Partial<UpgradePlan>): UpgradePlan;
  createdEntities(): readonly FactoryEntity[];
  cleanup(): void;
}

function version(faker: Faker): string {
  return [
    faker.number.int({ min: 0, max: 9 }),
    faker.number.int({ min: 0, max: 20 }),
    faker.number.int({ min: 0, max: 40 }),
  ].join(".");
}

/**
 * Build a per-test factory suite. A private Faker instance keeps parallel tests
 * from changing each other's random stream, while the explicit seed makes a
 * failing test reproducible from its title-derived seed.
 */
export function createPackManagerFactories(
  seed = DEFAULT_FACTORY_SEED,
): PackManagerFactories {
  const faker = new Faker({ locale: [en] });
  const entities: FactoryEntity[] = [];

  faker.seed(seed);
  faker.setDefaultRefDate(FIXED_TIME);

  function track<T>(kind: FactoryEntityKind, id: string, value: T): T {
    entities.push({ kind, id });
    return value;
  }

  function createSettings(overrides: Partial<Settings> = {}): Settings {
    return {
      runBrewUpdateOnRefresh: true,
      autoRefreshOnLaunch: false,
      stallAfterSecs: 120,
      upgradeHardCapMins: 30,
      logLevel: "debug",
      autoOpenDrawer: true,
      includeGreedyByDefault: false,
      autoCheckForUpdates: false,
      ...overrides,
    };
  }

  function createPackage(overrides: Partial<Package> = {}): Package {
    const kind = overrides.kind ?? faker.helpers.arrayElement(PACKAGE_KINDS);
    const name = overrides.name ?? faker.word.noun().toLowerCase();
    const installed =
      overrides.installed === undefined ? version(faker) : overrides.installed;
    const latest =
      overrides.latest === undefined ? version(faker) : overrides.latest;

    return track("package", overrides.id ?? `${kind}:${name}`, {
      id: overrides.id ?? `${kind}:${name}`,
      name,
      kind,
      installed,
      latest,
      // The package manager's verdict is authoritative. Version strings are
      // display data and may not be semver-comparable, so never infer this.
      outdated: overrides.outdated ?? false,
      pinned: false,
      ...overrides,
    });
  }

  function createManagerInfo(
    overrides: Partial<ManagerInfo> = {},
  ): ManagerInfo {
    const id = overrides.id ?? faker.helpers.arrayElement(MANAGER_IDS);
    const status = overrides.status ?? "present";
    const binaryPath = `/test/bin/${id}`;
    const presenceFields =
      status === "present"
        ? {
            binaryPath,
            canonicalPath: binaryPath,
            version: overrides.version ?? version(faker),
            evidence: `synthetic test binary at ${binaryPath}`,
          }
        : { installHint: `install ${id}` };

    return track("managerInfo", id, {
      id,
      displayName: DISPLAY_NAMES[id],
      status,
      managedBy: "standalone",
      selfUpdate:
        status === "present"
          ? { kind: "inBand", commandPreview: `${id} self-update` }
          : {
              kind: "unavailable",
              reason: `${DISPLAY_NAMES[id]} is not installed`,
            },
      ...presenceFields,
      ...overrides,
    });
  }

  function createDetectionReport(
    overrides: Partial<DetectionReport> = {},
  ): DetectionReport {
    const managers =
      overrides.managers ?? MANAGER_IDS.map((id) => createManagerInfo({ id }));
    const report: DetectionReport = {
      managers,
      env: {
        path: "/test/bin:/usr/bin:/bin",
        entries: ["/test/bin", "/usr/bin", "/bin"],
        source: "staticFallback",
        home: "/test/home",
      },
      ...overrides,
    };

    return track("detectionReport", `detection-${entities.length}`, report);
  }

  function createManagerSnapshot(
    overrides: Partial<ManagerSnapshot> = {},
  ): ManagerSnapshot {
    const managerId =
      overrides.managerId ?? faker.helpers.arrayElement(MANAGER_IDS);
    const packages = overrides.packages ?? [
      createPackage({
        kind: managerId === "brew" ? "formula" : "tool",
        name: `${managerId}-sample`,
      }),
    ];
    const selfStatus = overrides.selfStatus ?? {
      installed: version(faker),
      latest: version(faker),
      updateAvailable: true,
    };
    const snapshot: ManagerSnapshot = {
      managerId,
      refreshedAt: FIXED_TIME,
      packages,
      selfStatus,
      health: [],
      ...overrides,
    };

    return track(
      "managerSnapshot",
      `${managerId}@${snapshot.refreshedAt}`,
      snapshot,
    );
  }

  function createOperationRecord(
    overrides: Partial<OperationRecord> = {},
  ): OperationRecord {
    const opId = overrides.opId ?? faker.string.uuid();
    const executor = overrides.executor ?? "npm";
    const subject = overrides.subject ?? executor;
    const record: OperationRecord = {
      opId,
      kind: "upgrade",
      executor,
      subject,
      status: "queued",
      commandLine: `${executor} upgrade synthetic-package`,
      packageIds: ["globalPackage:synthetic-package"],
      queuedAt: FIXED_TIME,
      startedAt: null,
      finishedAt: null,
      exitCode: null,
      error: null,
      logPath: `/test/logs/${opId}.log`,
      ...overrides,
    };

    return track("operationRecord", record.opId, record);
  }

  function createUpgradePlan(
    overrides: Partial<UpgradePlan> = {},
  ): UpgradePlan {
    const planId = overrides.planId ?? faker.string.uuid();
    const packageId = "globalPackage:synthetic-package";
    const plan: UpgradePlan = {
      planId,
      request: {
        selection: [{ managerId: "npm", packageId }],
        includeSelfUpdates: false,
        includeGreedyCasks: false,
      },
      groups: [
        {
          subject: "npm",
          executor: "npm",
          locks: ["npm"],
          commands: [
            {
              argvPreview: "npm install -g synthetic-package@latest",
              label: "Upgrade synthetic-package",
            },
          ],
          packageIds: [packageId],
          selfUpdate: false,
        },
      ],
      excluded: [],
      notes: [],
      warnings: [],
      ...overrides,
    };

    return track("upgradePlan", plan.planId, plan);
  }

  function createAppState(overrides: Partial<AppState> = {}): AppState {
    const detection = overrides.detection ?? createDetectionReport();
    const snapshots =
      overrides.snapshots ??
      MANAGER_IDS.map((managerId) => createManagerSnapshot({ managerId }));
    const state: AppState = {
      detection,
      snapshots,
      operations: overrides.operations ?? [],
      settings: overrides.settings ?? createSettings(),
      ...overrides,
    };

    return track("appState", `app-state-${entities.length}`, state);
  }

  function createAppUpdateStatus(
    overrides: Partial<AppUpdateStatus> = {},
  ): AppUpdateStatus {
    return {
      currentVersion: "0.0.0-test",
      state: { kind: "idle" },
      lastTrigger: null,
      ...overrides,
    };
  }

  return {
    seed,
    createAppState,
    createAppUpdateStatus,
    createDetectionReport,
    createManagerInfo,
    createManagerSnapshot,
    createOperationRecord,
    createPackage,
    createSettings,
    createUpgradePlan,
    createdEntities: () => entities.map((entity) => ({ ...entity })),
    cleanup: () => {
      entities.length = 0;
      faker.seed(seed);
      faker.setDefaultRefDate(FIXED_TIME);
    },
  };
}
