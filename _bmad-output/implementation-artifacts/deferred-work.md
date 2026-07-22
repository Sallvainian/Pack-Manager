- source_spec: `/Users/sallvain/Projects/Pack-Manager/_bmad-output/implementation-artifacts/spec-harden-command-trust-boundaries.md`
  summary: Decide whether command previews need shell-style escaping so distinct structured argv can never render identically.
  evidence: `command_preview` predates this story and joins argv with spaces; changing its visible output requires the separate product decision reserved by the spec.

- source_spec: `/Users/sallvain/Projects/Pack-Manager/_bmad-output/implementation-artifacts/spec-harden-command-trust-boundaries.md`
  summary: Order concurrent redetection requests so an older slow probe cannot publish after a newer probe.
  evidence: Concurrent redetections were already last-completion-wins before this story; the new revision barrier prevents unsafe plan execution but does not define frontend publication order.

- source_spec: `/Users/sallvain/Projects/Pack-Manager/_bmad-output/implementation-artifacts/spec-harden-command-trust-boundaries.md`
  summary: Give direct self-update and health-fix commands the same coherent state-capture and revision-aware admission used by bulk plans.
  evidence: These pre-existing handlers read detection, registry, settings, and ToolEnv outside the coordinator before normal queue submission, so concurrent refresh or redetection can make their constructed inputs stale.

- source_spec: `/Users/sallvain/Projects/Pack-Manager/_bmad-output/implementation-artifacts/spec-harden-command-trust-boundaries.md`
  summary: Define whether plan confirmation must re-probe package-manager native state changed by processes outside Pack-Manager.
  evidence: Plan validation consistently protects the app's coherent cached epoch, but external package, pin, shim, or path changes do not advance that in-process revision; live re-probing would materially change confirmation latency and semantics.
