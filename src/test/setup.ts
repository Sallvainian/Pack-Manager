/**
 * test/setup.ts — jsdom + jest-dom test environment (SPEC §7.5).
 *
 * This unit does not register a global vitest `setupFiles` entry (that config
 * file is owned by U1), so test files import this module for its side effects:
 *   `import "../test/setup";`
 * It installs the jest-dom matchers and the jsdom polyfills our components rely
 * on (matchMedia, ResizeObserver, scrollIntoView, clipboard).
 */
import "@testing-library/jest-dom/vitest";
import { cleanup } from "@testing-library/react";
import { afterEach, vi } from "vitest";

// Vitest is not configured with `globals: true`, so React Testing Library's
// auto-cleanup never registers. Every rendering test imports this module, so we
// register the DOM teardown here to keep tests isolated (no accumulated nodes).
afterEach(() => cleanup());

if (!window.matchMedia) {
  window.matchMedia = ((query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    addListener: vi.fn(),
    removeListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })) as unknown as typeof window.matchMedia;
}

if (!("ResizeObserver" in globalThis)) {
  class ResizeObserverStub {
    observe(): void {}
    unobserve(): void {}
    disconnect(): void {}
  }
  (globalThis as { ResizeObserver?: unknown }).ResizeObserver = ResizeObserverStub;
}

if (!Element.prototype.scrollIntoView) {
  Element.prototype.scrollIntoView = vi.fn();
}

if (!navigator.clipboard) {
  Object.defineProperty(navigator, "clipboard", {
    configurable: true,
    value: { writeText: vi.fn().mockResolvedValue(undefined) },
  });
}
