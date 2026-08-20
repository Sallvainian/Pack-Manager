import { configDefaults, defineConfig } from "vitest/config";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  test: {
    environment: "jsdom",
    // BMAD test artifacts are paste-ready source snippets with destination-
    // relative imports, not live suites. Keep them out of Vitest discovery.
    exclude: [
      ...configDefaults.exclude,
      "tests/e2e/**",
      "_bmad-output/test-artifacts/generated/**",
    ],
  },
});
