import "../test/setup";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, within } from "@testing-library/react";

vi.mock("../lib/ipc/bridge", () => import("../test/fakeIpc"));

import { Sidebar } from "../components/shell/Sidebar";
import { resetStores, useManagersStore, usePackagesStore, useUiStore } from "../store";
import * as fakeIpc from "../test/fakeIpc";
import { detectionReport, npmSnapshot } from "../test/fixtures";

beforeEach(() => {
  fakeIpc.reset();
  resetStores();
  useManagersStore.getState().setDetection(detectionReport);
});

describe("sidebar present managers", () => {
  it("lists present managers in order and shows the outdated count pill", () => {
    usePackagesStore.getState().setSnapshot("npm", npmSnapshot);
    render(<Sidebar />);

    for (const name of ["Homebrew", "mise", "npm", "uv", "rustup"]) {
      expect(screen.getByText(name)).toBeInTheDocument();
    }
    // mas is absent → not a present item.
    expect(screen.queryByText("mas")).not.toBeInTheDocument();

    const npmItem = screen.getByText("npm").closest("button")!;
    expect(within(npmItem).getByText("4")).toBeInTheDocument();
  });

  it("navigates to a manager pane on click", () => {
    render(<Sidebar />);
    fireEvent.click(screen.getByText("uv").closest("button")!);
    expect(useUiStore.getState().view).toEqual({ kind: "manager", managerId: "uv" });
  });

  it("routes Dashboard / History / Settings", () => {
    render(<Sidebar />);
    fireEvent.click(screen.getByText("History"));
    expect(useUiStore.getState().view).toEqual({ kind: "history" });
    fireEvent.click(screen.getByText("Settings"));
    expect(useUiStore.getState().view).toEqual({ kind: "settings" });
    fireEvent.click(screen.getByText("Dashboard"));
    expect(useUiStore.getState().view).toEqual({ kind: "dashboard" });
  });
});

describe("sidebar absent disclosure", () => {
  it("reveals the Not installed managers with their install hint", () => {
    render(<Sidebar />);
    expect(screen.queryByText("brew install mas")).not.toBeInTheDocument();
    fireEvent.click(screen.getByText(/Not installed \(1\)/));
    expect(screen.getByText("mas")).toBeInTheDocument();
    expect(screen.getByText("brew install mas")).toBeInTheDocument();
  });
});
