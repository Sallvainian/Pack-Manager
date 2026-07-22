import "../test/setup";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, within } from "@testing-library/react";

vi.mock("../lib/ipc/bridge", () => import("../test/fakeIpc"));

import { DashboardView } from "../components/dashboard/DashboardView";
import type { IpcError } from "../lib/ipc/types";
import { resetStores, useManagersStore, usePackagesStore } from "../store";
import * as fakeIpc from "../test/fakeIpc";
import { cleanSnapshot, detectionReport, npmSnapshot } from "../test/fixtures";

beforeEach(() => {
  fakeIpc.reset();
  resetStores();
  useManagersStore.getState().setDetection(detectionReport);
});

describe("dashboard_fills_cards_independently", () => {
  it("fills the npm card while brew is still a skeleton", () => {
    usePackagesStore.getState().setSnapshot("npm", npmSnapshot);
    render(<DashboardView />);

    const npmCard = screen.getByLabelText("npm");
    expect(within(npmCard).getByText("4")).toBeInTheDocument();
    expect(within(npmCard).getByText(/updates available/)).toBeInTheDocument();

    const brewCard = screen.getByLabelText("Homebrew");
    expect(within(brewCard).getByText(/Refreshing/)).toBeInTheDocument();
    expect(within(brewCard).queryByText("4")).not.toBeInTheDocument();
  });
});

describe("mas_absent_renders_not_installed_with_hint", () => {
  it("shows a muted Not installed card with the install command", () => {
    render(<DashboardView />);
    const masCard = screen.getByLabelText("mas");
    expect(within(masCard).getByText("Not installed")).toBeInTheDocument();
    expect(within(masCard).getByText("brew install mas")).toBeInTheDocument();
  });
});

describe("manager_error_isolates_with_retry_and_stale_snapshot", () => {
  it("shows an error card with Retry while other cards stay populated", async () => {
    const err: IpcError = {
      code: "non_zero_exit",
      message: "The command exited with an error. View the log for details.",
      detail: "Error: some brew failure\nsecond line",
      managerId: "brew",
      opId: "op-brew-1",
      logPath: "/tmp/brew.log",
    };
    // brew had a prior snapshot, then a refresh failed.
    usePackagesStore.getState().setSnapshot("brew", { ...npmSnapshot, managerId: "brew" });
    usePackagesStore.getState().markStale("brew");
    useManagersStore.getState().setManagerError("brew", err);
    usePackagesStore.getState().setSnapshot("npm", npmSnapshot);

    fakeIpc.respond("refresh_manager", () => ({ opId: "op-brew-2" }));
    render(<DashboardView />);

    const brewCard = screen.getByLabelText("Homebrew");
    expect(within(brewCard).getByText("Command failed")).toBeInTheDocument();
    expect(within(brewCard).getByText(/Showing last successful data/)).toBeInTheDocument();

    // Other managers unaffected.
    const npmCard = screen.getByLabelText("npm");
    expect(within(npmCard).getByText("4")).toBeInTheDocument();

    fireEvent.click(within(brewCard).getByRole("button", { name: "Retry" }));
    await vi.waitFor(() => expect(fakeIpc.called("refresh_manager")).toBe(true));
    expect(fakeIpc.callsFor("refresh_manager")[0].args).toEqual({ args: { managerId: "brew" } });
  });
});

describe("all_clean_renders_up_to_date", () => {
  it("shows the up-to-date state when nothing is outdated", () => {
    usePackagesStore.getState().setSnapshot("uv", cleanSnapshot("uv"));
    render(<DashboardView />);
    const uvCard = screen.getByLabelText("uv");
    expect(within(uvCard).getByText(/Up to date/)).toBeInTheDocument();
  });
});
