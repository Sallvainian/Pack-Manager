import "../../test/setup";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/react";

vi.mock("../../lib/ipc/bridge", () => import("../../test/fakeIpc"));

import { HealthBanner } from "./HealthBanner";
import type { HealthIssue } from "../../lib/ipc/types";
import { resetStores } from "../../store";
import * as fakeIpc from "../../test/fakeIpc";

const issue: HealthIssue = {
  id: "uv:aider-chat",
  managerId: "uv",
  severity: "warning",
  title: "Tool `aider-chat` environment is broken.",
  detail:
    "warning: Tool `aider-chat` environment not found (run `uv tool install aider-chat --reinstall` to reinstall)",
  fixCommand: "uv tool install aider-chat --reinstall",
  fixable: true,
};

beforeEach(() => {
  fakeIpc.reset();
  resetStores();
});

describe("health_banner_renders_fix_command", () => {
  it("renders the issue title, the copyable fix command, and wires Run fix", async () => {
    fakeIpc.respond("run_health_fix", () => ({ opId: "op-fix-1" }));
    render(<HealthBanner managerId="uv" issues={[issue]} />);

    expect(screen.getByText("Tool `aider-chat` environment is broken.")).toBeInTheDocument();
    expect(screen.getByText("uv tool install aider-chat --reinstall")).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: "Run fix" }));
    await vi.waitFor(() => expect(fakeIpc.called("run_health_fix")).toBe(true));
    expect(fakeIpc.callsFor("run_health_fix")[0].args).toEqual({
      args: { managerId: "uv", issueId: "uv:aider-chat" },
    });
  });
});
