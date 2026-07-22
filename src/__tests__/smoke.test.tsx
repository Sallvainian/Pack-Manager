import "../test/setup";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { render, screen, within } from "@testing-library/react";

vi.mock("../lib/ipc/bridge", () => import("../test/fakeIpc"));

import App from "../App";
import type { AppState } from "../lib/ipc/types";
import { resetStores } from "../store";
import * as fakeIpc from "../test/fakeIpc";
import { defaultSettings, detectionReport, npmSnapshot } from "../test/fixtures";

const appState: AppState = {
  detection: detectionReport,
  snapshots: [npmSnapshot],
  operations: [],
  settings: { ...defaultSettings, autoRefreshOnLaunch: true },
};

beforeEach(() => {
  fakeIpc.reset();
  resetStores();
});

describe("App shell", () => {
  it("renders the full shell against fake data and hydrates from get_state", async () => {
    fakeIpc.respond("get_state", () => appState);
    fakeIpc.respond("refresh_all", () => ({ opIds: [] }));
    fakeIpc.respond("get_app_update_state", () => ({
      currentVersion: "0.1.1",
      state: { kind: "idle" },
      lastTrigger: null,
    }));

    render(<App />);

    // Static shell chrome renders immediately.
    expect(screen.getByText("Pack-Manager")).toBeInTheDocument();
    expect(screen.getByText("Packages")).toBeInTheDocument();

    // After bootstrap, cards appear and the npm card is filled from the snapshot.
    const npmCard = await screen.findByLabelText("npm");
    expect(within(npmCard).getByText("4")).toBeInTheDocument();
    expect(screen.getByLabelText("Homebrew")).toBeInTheDocument();

    // Launch refresh fired because autoRefreshOnLaunch is on.
    await vi.waitFor(() => expect(fakeIpc.called("refresh_all")).toBe(true));
  });
});
