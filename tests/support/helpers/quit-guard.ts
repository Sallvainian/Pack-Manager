import type { Locator, Page } from "@playwright/test";

import {
  EVENT_QUIT_REQUESTED,
  type QuitRequestedEvent,
} from "../../../src/lib/ipc/types";
import { expect } from "../fixtures";
import type { FakeTauriIpc } from "../fixtures";

/** Emit only after the application has installed its quit-event listener. */
export async function whenQuitIsRequested(
  tauriIpc: FakeTauriIpc,
  opIds: readonly string[],
): Promise<void> {
  await expect
    .poll(() => tauriIpc.listenerCount(EVENT_QUIT_REQUESTED), {
      message: `no listener registered for ${EVENT_QUIT_REQUESTED}`,
    })
    .toBeGreaterThan(0);

  const payload: QuitRequestedEvent = { opIds: [...opIds] };
  await tauriIpc.emit(EVENT_QUIT_REQUESTED, payload);
}

/** Browser page object for the frontend limb of the native quit guard. */
export function createQuitGuard(page: Page) {
  const dialog: Locator = page.getByRole("alertdialog", {
    name: "Operations still running",
  });

  return {
    dialog,
    heading: dialog.getByRole("heading", { name: "Operations still running" }),
    operation: (title: string): Locator =>
      dialog.getByRole("listitem").filter({ hasText: title }),
    operations: (): Locator => dialog.getByRole("listitem"),
    keepRunning: dialog.getByRole("button", { name: "Keep running" }),
    cancelAndQuit: dialog.getByRole("button", {
      name: "Cancel operations and quit",
    }),
  };
}

export const QUIT_COMMANDS = {
  cancelOperation: "cancel_operation",
  confirmQuit: "confirm_quit",
} as const;
