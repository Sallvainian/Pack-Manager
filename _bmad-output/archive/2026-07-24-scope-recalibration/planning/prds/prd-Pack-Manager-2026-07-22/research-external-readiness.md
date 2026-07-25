---
title: External macOS and Tauri Release-Readiness Research
status: complete
research_date: 2026-07-22
source_policy: Current primary and official sources only
---

# External macOS and Tauri Release-Readiness Research

## Executive summary

For Pack-Manager's existing direct-download distribution model, launch readiness
has two independent trust chains:

1. macOS distribution trust: the shipped app must be Developer ID signed,
   hardened, notarized, and accepted by Gatekeeper.
2. Tauri updater trust: the update archive, detached signature, and HTTPS update
   metadata must form a complete, internally consistent release set.

Apple explicitly says notarization is an automated malware and code-signing
check, not App Review. It is therefore a necessary distribution gate, but not
evidence that Pack-Manager's product behavior works. Tauri likewise requires
signed updater artifacts and will not allow updater signature verification to
be disabled. The launch-grade PRD should consequently require both platform
trust evidence and an end-to-end install/update acceptance journey.

## Scope, authority, and method

This is a narrow research companion for the Pack-Manager readiness PRD. The
repository remains authoritative for product behavior and release mechanics,
especially `docs/SPEC.md`, `docs/DECISIONS.md`, `src-tauri/tauri.conf.json`,
`.github/workflows/release.yml`, and `AGENTS.md`.

The external sources were checked on 2026-07-22 and are limited to official
Apple and Tauri guidance. They are used to confirm release expectations, not to
select a new architecture, workflow product, update service, or distribution
channel. The Tauri macOS signing page was updated May 17, 2026, and its GitHub
pipeline page was updated June 29, 2026, making them current for this snapshot
([Tauri macOS Code Signing](https://v2.tauri.app/distribute/sign/macos/);
[Tauri GitHub pipeline](https://v2.tauri.app/distribute/pipelines/github/)).

## Confirmed external requirements

### Direct macOS distribution

- Apple says Gatekeeper checks downloaded software from outside the App Store
  for an identified developer, Apple notarization, and evidence that it has not
  been altered. It also requests user approval on first open
  ([Apple Platform Security: Gatekeeper and runtime protection](https://support.apple.com/guide/security/gatekeeper-and-runtime-protection-sec5599b66df/web)).
- Apple's notarization prerequisites include valid signatures on distributed
  executables, an appropriate Developer ID certificate, hardened runtime, a
  secure timestamp, correctly formed entitlements, and no enabled
  `com.apple.security.get-task-allow` debug entitlement. A successful
  submission produces a ticket that can be stapled to the software and that
  Gatekeeper can also retrieve online
  ([Apple: Notarizing macOS software before distribution](https://developer.apple.com/documentation/security/notarizing-macos-software-before-distribution)).
- Apple describes notarization as an automated scan for malicious content and
  code-signing problems, and explicitly distinguishes it from App Review.
  Passing notarization does not establish product correctness, usability, or
  upgrade safety
  ([Apple: Notarizing macOS software before distribution](https://developer.apple.com/documentation/security/notarizing-macos-software-before-distribution)).
- Tauri's direct-distribution guidance selects a **Developer ID Application**
  certificate for apps shipped outside the App Store and states that
  notarization is required when that certificate is used. Ad-hoc signing still
  leaves users with manual security approval and is not equivalent to the
  repository's signed/notarized release promise
  ([Tauri: macOS Code Signing](https://v2.tauri.app/distribute/sign/macos/)).

### Tauri updater and GitHub Release contract

- Tauri requires a cryptographic signature for updater artifacts and says this
  verification cannot be disabled. The public key is embedded in app
  configuration; the private key signs releases. Tauri warns that losing the
  private key prevents publishing future updates to already-installed copies
  ([Tauri: Updater — Signing updates](https://v2.tauri.app/plugin/updater/#signing-updates)).
- With updater artifacts enabled, Tauri's macOS output is the `.app`, an
  `.app.tar.gz` updater archive, and the archive's `.sig`
  ([Tauri: Updater — Building](https://v2.tauri.app/plugin/updater/#building)).
- Production updater endpoints enforce TLS. For a static update file, the
  version must be valid SemVer and each platform entry requires an artifact URL
  and the **contents** of its generated signature. Tauri validates the whole
  metadata file before checking its version, so one incomplete platform entry
  can invalidate the response
  ([Tauri: Updater — Configuration and static JSON](https://v2.tauri.app/plugin/updater/#tauri-configuration)).
- Tauri documents GitHub Releases as a supported pairing: a pipeline can build
  and publish the app, while the updater queries the resulting release. This
  confirms the repository's chosen distribution shape; it does not require
  replacing Pack-Manager's existing custom release-please workflow with
  `tauri-action`
  ([Tauri: GitHub release pipeline](https://v2.tauri.app/distribute/pipelines/github/)).

## Product and release-gate implications

Every item in this section is an **inference** from the official requirements
above applied to Pack-Manager's repository-defined behavior. These are outcome
and evidence gates, not implementation prescriptions.

| Gate | Pack-Manager implication | Launch evidence |
|---|---|---|
| **EXT-RG-01 — Public distribution trust** | **Inference:** A public GitHub Release is not launch-ready if its app or installer is unsigned, unnotarized, or rejected by Gatekeeper, even when the workflow itself is green. The repository's graceful unsigned mode may remain useful for forks or diagnostic builds, but it cannot count as public-release success. | The exact shipped app and DMG carry the expected Developer ID identity and valid signatures; notarization is accepted and stapled; macOS Gatekeeper accepts the downloaded deliverable without a bypass. |
| **EXT-RG-02 — One coherent release set** | **Inference:** The tag, shipped bundle version, updater version, asset names/URLs, and signatures are one atomic release claim. Any mismatch or missing member makes the release not ready. | The release tag and bundle version agree, and the DMG, ZIP, `.app.tar.gz`, `.sig`, and `latest.json` all exist for that version at the URLs the metadata advertises. |
| **EXT-RG-03 — Updater integrity and reachability** | **Inference:** Both repository-supported Mac architectures must resolve through valid static metadata to the universal updater archive, and the embedded updater public key must validate that archive's current detached signature. Merely uploading files is insufficient. | The HTTPS `latest.json` is reachable; its SemVer is current; `darwin-aarch64` and `darwin-x86_64` entries are complete; their URLs return the intended archive; signature verification succeeds. |
| **EXT-RG-04 — Real install and upgrade journey** | **Inference:** Platform trust must be exercised from the user's entry points, not only inspected inside CI. A clean DMG install must launch through Finder/Gatekeeper, and an actually installed prior public version must discover, download, and install the candidate through Pack-Manager's repository-defined explicit **Restart to update** action. | A clean-machine-style first-install check and a previous-release-to-candidate update check both pass; the updated app relaunches as the intended version without an administrator prompt. |
| **EXT-RG-05 — Signing continuity** | **Inference:** Availability of the Apple release credentials and the matching Tauri updater private key is a release dependency. Because Tauri says loss of the updater key strands installed clients, an unexplained missing or mismatched key is a launch blocker rather than a reason to publish a degraded release. | The release run proves access to the expected identities and produces signatures accepted by the app's configured updater public key. Key-storage design and rotation procedure remain outside this research's scope. |

## PRD-ready conclusions

- Treat the **published release**, not the build job, as the release-gate
  subject.
- Treat the signed/notarized first-install artifacts and signed updater
  artifacts as one version-coherent evidence set.
- Require an end-to-end update from a real prior public version; metadata shape
  checks alone cannot demonstrate that installed users can update.
- Preserve the repository's existing interaction contract: background checking
  and downloading are allowed, installation remains gated by the user's
  explicit restart action, and a non-writable install location falls back to
  manual installation rather than an administrator prompt.
- Keep Apple notarization evidence distinct from functional product-readiness
  evidence.

## Explicit non-expansion of scope

This research does **not** add Mac App Store distribution, App Sandbox work, a
new updater provider, silent installation, telemetry, package installation or
removal, a new release automation framework, or implementation-specific key
management. It only converts Apple and Tauri's current distribution contracts
into observable release outcomes for the product and workflow already defined
by the repository.
