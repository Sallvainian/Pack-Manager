import "../../test/setup";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";

vi.mock("../../lib/ipc/bridge", () => import("../../test/fakeIpc"));

import { SelfUpdateCard } from "./SelfUpdateCard";
import type { OpStatusEvent } from "../../lib/ipc/types";
import { resetStores, useManagersStore, useOperationsStore, usePackagesStore, useUiStore } from "../../store";
import * as fakeIpc from "../../test/fakeIpc";
import { defaultSettings, detectionReport, miseSnapshot, npmSnapshot, uvSnapshot } from "../../test/fixtures";

beforeEach(() => {
  fakeIpc.reset();
  resetStores();
  useManagersStore.getState().setDetection(detectionReport);
  useUiStore.getState().setSettings(defaultSettings);
});

function brewRunning(): OpStatusEvent {
  return {
    opId: "op-brew-refresh",
    kind: "refresh",
    executor: "brew",
    subject: "brew",
    status: "running",
    commandLine: "/opt/homebrew/bin/brew update",
    logPath: "/tmp/brew.log",
  };
}

describe("self_update_card_shows_routed_subtitle_and_queued_behind_executor", () => {
  it("renders the routed command + why, and 'Queued behind Homebrew' when brew is busy", () => {
    usePackagesStore.getState().setSnapshot("mise", miseSnapshot);
    useOperationsStore.getState().applyStatus(brewRunning());
    render(<SelfUpdateCard managerId="mise" />);

    const section = screen.getByLabelText("mise self-update");
    expect(section.textContent).toContain("brew upgrade mise");
    expect(section.textContent).toContain("on the Homebrew queue");
    expect(screen.getByText("Queued behind Homebrew")).toBeInTheDocument();
  });
});

describe("npm_card_shows_mise_reset_note", () => {
  it("shows the permanent mise-reset note on the npm self-update card", () => {
    usePackagesStore.getState().setSnapshot("npm", npmSnapshot);
    render(<SelfUpdateCard managerId="npm" />);
    const section = screen.getByLabelText("npm self-update");
    expect(section.textContent).toContain("upgrading node via mise resets them");
    expect(section.textContent).toContain("npm install -g npm@latest");
  });
});

describe("self_update_disabled_when_executor_absent", () => {
  it("disables the Update button when the routed executor is not installed", () => {
    const report = {
      ...detectionReport,
      managers: detectionReport.managers.map((m) =>
        m.id === "uv"
          ? {
              ...m,
              selfUpdate: {
                kind: "routed" as const,
                executor: "mas" as const,
                commandPreview: "mas upgrade uv",
                why: "uv is managed by mas",
              },
            }
          : m,
      ),
    };
    useManagersStore.getState().setDetection(report);
    usePackagesStore.getState().setSnapshot("uv", uvSnapshot);
    render(<SelfUpdateCard managerId="uv" />);

    const button = screen.getByRole("button", { name: /Update uv/ });
    expect(button).toBeDisabled();
    expect(screen.getByText("mas is not installed")).toBeInTheDocument();
  });
});
